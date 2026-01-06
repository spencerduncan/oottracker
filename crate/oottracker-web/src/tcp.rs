//! TCP listener for PJ64 integration.
//!
//! This module provides a TCP server that listens on port 24801 for connections
//! from Project64's Lua script. When RAM data is received, it updates the
//! corresponding room state and broadcasts changes via WebSocket.

use {
    crate::{edit_room, Error, Rooms},
    futures::stream::TryStreamExt as _,
    oottracker::{
        proto::{self, Packet, TCP_PORT},
        save::GameMode,
    },
    sqlx::SqlitePool,
    std::{env, net::Ipv4Addr},
    tokio::net::TcpListener,
    tokio_stream::wrappers::TcpListenerStream,
    tracing::{info, warn},
};

/// Default room name for PJ64 connections.
const DEFAULT_PJ64_ROOM: &str = "pj64";

/// Starts the TCP listener for PJ64 connections.
///
/// The room name can be configured via the `PJ64_ROOM` environment variable.
/// If not set, defaults to "pj64".
pub async fn run_tcp_listener(pool: SqlitePool, rooms: Rooms) -> Result<(), Error> {
    let room_name = env::var("PJ64_ROOM").unwrap_or_else(|_| DEFAULT_PJ64_ROOM.to_string());

    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, TCP_PORT)).await?;
    info!(
        "TCP listener started on port {} for room '{}'",
        TCP_PORT, room_name
    );

    let mut incoming = TcpListenerStream::new(listener);

    while let Some(tcp_stream) = incoming.try_next().await? {
        let peer_addr = tcp_stream
            .peer_addr()
            .map(|a| a.to_string())
            .unwrap_or_else(|_| "unknown".to_string());
        info!("PJ64 client connected from {}", peer_addr);

        let pool = pool.clone();
        let rooms = Rooms::clone(&rooms);
        let room_name = room_name.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_pj64_connection(pool, rooms, room_name, tcp_stream).await {
                warn!("PJ64 connection error: {}", e);
            }
            info!("PJ64 client disconnected from {}", peer_addr);
        });
    }

    Ok(())
}

/// Handles a single PJ64 connection, reading packets and updating room state.
async fn handle_pj64_connection(
    pool: SqlitePool,
    rooms: Rooms,
    room_name: String,
    tcp_stream: tokio::net::TcpStream,
) -> Result<(), Error> {
    let mut packet_stream = proto::read(tcp_stream);

    while let Some(packet) = packet_stream.try_next().await.map_err(Error::from)? {
        match packet {
            Packet::RamInit(ram) => {
                // Only update if the game is in gameplay mode
                if ram.save.game_mode == GameMode::Gameplay {
                    edit_room(&pool, &rooms, room_name.clone(), |room| {
                        room.model.ram = ram.clone();
                        room.model.update_knowledge();
                        Ok(())
                    })
                    .await?;
                }
            }
            Packet::SaveInit(save) => {
                edit_room(&pool, &rooms, room_name.clone(), |room| {
                    room.model.ram.save = save.clone();
                    room.model.update_knowledge();
                    Ok(())
                })
                .await?;
            }
            Packet::SaveDelta(delta) => {
                edit_room(&pool, &rooms, room_name.clone(), |room| {
                    room.model.ram.save = &room.model.ram.save + &delta;
                    room.model.update_knowledge();
                    Ok(())
                })
                .await?;
            }
            Packet::KnowledgeInit(knowledge) => {
                edit_room(&pool, &rooms, room_name.clone(), |room| {
                    room.model.knowledge = knowledge.clone();
                    Ok(())
                })
                .await?;
            }
            Packet::ModelInit(model) => {
                edit_room(&pool, &rooms, room_name.clone(), |room| {
                    room.model = model.clone();
                    room.model.update_knowledge();
                    Ok(())
                })
                .await?;
            }
            Packet::ModelDelta(delta) => {
                edit_room(&pool, &rooms, room_name.clone(), |room| {
                    room.model += delta.clone();
                    room.model.update_knowledge();
                    Ok(())
                })
                .await?;
            }
            Packet::MmRamInit(mm_save) => {
                edit_room(&pool, &rooms, room_name.clone(), |room| {
                    room.model.ram.mm_save = Some(mm_save.clone());
                    room.model.update_knowledge();
                    Ok(())
                })
                .await?;
            }
            Packet::UpdateCell(_, _) => {
                // UpdateCell is typically for Firebase connections, not PJ64
                warn!("Received unexpected UpdateCell packet from PJ64");
            }
            Packet::Goodbye => {
                // Connection closing, handled by the stream ending
                break;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_room_name() {
        assert_eq!(DEFAULT_PJ64_ROOM, "pj64");
    }

    #[test]
    fn test_tcp_port() {
        assert_eq!(TCP_PORT, 24801);
    }
}
