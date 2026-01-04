//! Integration tests for emulator connections.
//!
//! These tests verify:
//! - TCP protocol connection handling (port 24801)
//! - Binary packet serialization/deserialization
//! - Mock emulator connection behavior
//! - Connection timeout and reconnection handling

mod common;

use {
    async_proto::Protocol,
    futures::{pin_mut, stream, StreamExt},
    oottracker::{
        knowledge::Knowledge,
        net::{Connection, Error, NullConnection, TcpConnection},
        proto::{self, Packet, ReadError, TCP_PORT, VERSION},
        save::Save,
        ModelState, Ram,
    },
    std::{io::Cursor, net::Ipv4Addr, sync::Arc, time::Duration},
    tokio::{
        io::AsyncWriteExt,
        net::{TcpListener, TcpStream},
        sync::Mutex,
        time::timeout,
    },
};

/// Helper macro to run async tests using tokio runtime
macro_rules! async_test {
    ($test_name:ident, $body:expr) => {
        #[test]
        fn $test_name() {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async { $body });
        }
    };
}

// ============================================================================
// Protocol Constants Tests
// ============================================================================

#[test]
fn test_protocol_version() {
    // Version 6 is the current protocol version
    assert_eq!(VERSION, 6);
}

#[test]
fn test_tcp_port() {
    // Standard tracker port is 24801
    assert_eq!(TCP_PORT, 24801);
}

// ============================================================================
// Binary Packet Serialization Tests
// ============================================================================

mod packet_serialization {
    use super::*;

    #[test]
    fn test_goodbye_packet_roundtrip() {
        let packet = Packet::Goodbye;
        let mut buf = Vec::new();
        packet.write_sync(&mut buf).unwrap();

        let mut cursor = Cursor::new(buf);
        let decoded = Packet::read_sync(&mut cursor).unwrap();
        assert!(matches!(decoded, Packet::Goodbye));
    }

    #[test]
    fn test_save_init_packet_roundtrip() {
        let save = Save::default();
        let packet = Packet::SaveInit(save.clone());
        let mut buf = Vec::new();
        packet.write_sync(&mut buf).unwrap();

        let mut cursor = Cursor::new(buf);
        let decoded = Packet::read_sync(&mut cursor).unwrap();
        match decoded {
            Packet::SaveInit(decoded_save) => {
                assert_eq!(decoded_save, save);
            }
            _ => panic!("Expected SaveInit packet"),
        }
    }

    #[test]
    fn test_ram_init_packet_roundtrip() {
        let ram = Ram::default();
        let packet = Packet::RamInit(ram.clone());
        let mut buf = Vec::new();
        packet.write_sync(&mut buf).unwrap();

        let mut cursor = Cursor::new(buf);
        let decoded = Packet::read_sync(&mut cursor).unwrap();
        match decoded {
            Packet::RamInit(decoded_ram) => {
                assert_eq!(decoded_ram.save, ram.save);
            }
            _ => panic!("Expected RamInit packet"),
        }
    }

    #[test]
    fn test_knowledge_init_packet_roundtrip() {
        let knowledge = Knowledge::default();
        let packet = Packet::KnowledgeInit(knowledge.clone());
        let mut buf = Vec::new();
        packet.write_sync(&mut buf).unwrap();

        let mut cursor = Cursor::new(buf);
        let decoded = Packet::read_sync(&mut cursor).unwrap();
        match decoded {
            Packet::KnowledgeInit(decoded_knowledge) => {
                assert_eq!(decoded_knowledge, knowledge);
            }
            _ => panic!("Expected KnowledgeInit packet"),
        }
    }

    #[test]
    fn test_update_cell_packet_roundtrip() {
        use oottracker::ui::TrackerCellId;
        use serde_json::json;

        // Use a valid TrackerCellId variant
        let cell_id = TrackerCellId::GoMode;
        let value = json!({"test": "value"});
        let packet = Packet::UpdateCell(cell_id, value.clone());
        let mut buf = Vec::new();
        packet.write_sync(&mut buf).unwrap();

        let mut cursor = Cursor::new(buf);
        let decoded = Packet::read_sync(&mut cursor).unwrap();
        match decoded {
            Packet::UpdateCell(decoded_cell_id, decoded_value) => {
                assert_eq!(decoded_cell_id, cell_id);
                assert_eq!(decoded_value, value);
            }
            _ => panic!("Expected UpdateCell packet"),
        }
    }
}

// ============================================================================
// TCP Protocol Handshake Tests
// ============================================================================

mod tcp_handshake {
    use super::*;

    #[test]
    fn test_handshake_sync() {
        let mut buf = Vec::new();
        VERSION.write_sync(&mut buf).unwrap();

        assert_eq!(buf.len(), 1);
        assert_eq!(buf[0], VERSION);
    }

    #[test]
    fn test_version_mismatch_error_display_old_client() {
        let error = ReadError::VersionMismatch {
            server: 6,
            client: 5,
        };
        let display = format!("{}", error);
        assert!(display.contains("outdated auto-tracking plugin"));
        assert!(display.contains("protocol 5"));
        assert!(display.contains("protocol 6"));
    }

    #[test]
    fn test_version_mismatch_error_display_new_client() {
        let error = ReadError::VersionMismatch {
            server: 6,
            client: 7,
        };
        let display = format!("{}", error);
        assert!(display.contains("app is outdated"));
        assert!(display.contains("protocol 7"));
        assert!(display.contains("protocol 6"));
    }

    async_test!(test_read_stream_with_correct_version, {
        // Create a mock TCP connection using localhost
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).await.unwrap();
        let addr = listener.local_addr().unwrap();

        // Spawn a client that sends correct version and goodbye packet
        let client_handle = tokio::spawn(async move {
            let mut stream = TcpStream::connect(addr).await.unwrap();
            // Send version
            VERSION.write(&mut stream).await.unwrap();
            // Send Goodbye packet
            Packet::Goodbye.write(&mut stream).await.unwrap();
        });

        // Accept connection and read
        let (tcp_stream, _) = listener.accept().await.unwrap();
        let packet_stream = proto::read(tcp_stream);
        pin_mut!(packet_stream);

        // The stream should complete without packets (Goodbye terminates)
        let result = packet_stream.next().await;
        assert!(result.is_none() || matches!(result, Some(Ok(Packet::Goodbye))));

        client_handle.await.unwrap();
    });

    async_test!(test_read_stream_with_wrong_version, {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).await.unwrap();
        let addr = listener.local_addr().unwrap();

        // Spawn a client that sends wrong version
        let client_handle = tokio::spawn(async move {
            let mut stream = TcpStream::connect(addr).await.unwrap();
            // Send wrong version
            5u8.write(&mut stream).await.unwrap();
        });

        // Accept connection and read
        let (tcp_stream, _) = listener.accept().await.unwrap();
        let packet_stream = proto::read(tcp_stream);
        pin_mut!(packet_stream);

        let result = packet_stream.next().await;
        assert!(matches!(
            result,
            Some(Err(ReadError::VersionMismatch {
                server: 6,
                client: 5
            }))
        ));

        client_handle.await.unwrap();
    });

    async_test!(test_write_stream, {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).await.unwrap();
        let addr = listener.local_addr().unwrap();

        // Spawn a reader that verifies the protocol
        let reader_handle = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();

            // Read version
            let version = u8::read(&mut stream).await.unwrap();
            assert_eq!(version, VERSION);

            // Read SaveInit packet
            let packet = Packet::read(&mut stream).await.unwrap();
            assert!(matches!(packet, Packet::SaveInit(_)));

            // Read Goodbye packet
            let packet = Packet::read(&mut stream).await.unwrap();
            assert!(matches!(packet, Packet::Goodbye));
        });

        // Connect and write packets
        let stream = TcpStream::connect(addr).await.unwrap();
        let packets = stream::iter(vec![Packet::SaveInit(Save::default())]);
        proto::write(stream, packets).await.unwrap();

        reader_handle.await.unwrap();
    });
}

// ============================================================================
// Connection Trait Tests
// ============================================================================

mod connection_trait {
    use super::*;

    #[test]
    fn test_null_connection_cannot_change_state() {
        let conn = NullConnection;
        assert!(!conn.can_change_state());
    }

    #[test]
    fn test_null_connection_display_kind() {
        let conn = NullConnection;
        assert_eq!(conn.display_kind(), "nothing");
    }

    #[test]
    fn test_null_connection_hash_consistency() {
        let conn1 = NullConnection;
        let conn2 = NullConnection;
        assert_eq!(conn1.hash(), conn2.hash());
    }

    async_test!(test_null_connection_set_state_fails, {
        let conn = NullConnection;
        let state = ModelState::default();
        let result = conn.set_state(&state).await;
        assert!(matches!(result, Err(Error::CannotChangeState)));
    });

    async_test!(test_null_connection_packet_stream_pending, {
        let conn = NullConnection;
        let stream = conn.packet_stream();
        pin_mut!(stream);

        // The stream should never yield a packet (it's pending forever)
        // We test this by using a timeout
        let result = timeout(Duration::from_millis(50), stream.next()).await;
        assert!(result.is_err()); // Timeout expected
    });

    #[test]
    fn test_tcp_connection_cannot_change_state() {
        let conn = TcpConnection;
        assert!(!conn.can_change_state());
    }

    #[test]
    fn test_tcp_connection_display_kind() {
        let conn = TcpConnection;
        assert_eq!(conn.display_kind(), "TCP");
    }

    #[test]
    fn test_tcp_connection_hash_consistency() {
        let conn1 = TcpConnection;
        let conn2 = TcpConnection;
        assert_eq!(conn1.hash(), conn2.hash());
    }

    async_test!(test_tcp_connection_set_state_fails, {
        let conn = TcpConnection;
        let state = ModelState::default();
        let result = conn.set_state(&state).await;
        assert!(matches!(result, Err(Error::CannotChangeState)));
    });
}

// ============================================================================
// Error Type Tests
// ============================================================================

mod error_types {
    use super::*;

    #[test]
    fn test_cannot_change_state_error_display() {
        let error = Error::CannotChangeState;
        let display = format!("{}", error);
        assert!(display.contains("read-only"));
    }

    #[test]
    fn test_io_error_display() {
        let io_err = std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "test");
        let error = Error::Io(Arc::new(io_err));
        let display = format!("{}", error);
        assert!(display.contains("I/O error"));
    }

    #[test]
    fn test_websocket_error_display() {
        let error = Error::Websocket {
            debug: "debug info".to_string(),
            display: "user-friendly message".to_string(),
        };
        let display = format!("{}", error);
        assert_eq!(display, "user-friendly message");
    }

    #[test]
    fn test_unexpected_websocket_message_display() {
        let error = Error::UnexpectedWebsocketMessage;
        let display = format!("{}", error);
        assert!(display.contains("unexpected WebSocket message"));
    }
}

// ============================================================================
// Mock Emulator Connection Tests
// ============================================================================

mod mock_emulator {
    use super::*;

    /// A mock emulator server that simulates an emulator connecting to the tracker
    struct MockEmulatorServer {
        listener: TcpListener,
    }

    impl MockEmulatorServer {
        async fn new() -> Self {
            let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).await.unwrap();
            Self { listener }
        }

        fn port(&self) -> u16 {
            self.listener.local_addr().unwrap().port()
        }

        async fn accept_and_serve(&self, packets: Vec<Packet>) {
            let (mut stream, _) = self.listener.accept().await.unwrap();

            // Send version handshake
            VERSION.write(&mut stream).await.unwrap();

            // Send all packets
            for packet in packets {
                packet.write(&mut stream).await.unwrap();
            }

            // Send goodbye
            Packet::Goodbye.write(&mut stream).await.unwrap();
        }
    }

    async_test!(test_mock_emulator_sends_save_init, {
        let server = MockEmulatorServer::new().await;
        let port = server.port();

        // Spawn server task
        let server_task = tokio::spawn(async move {
            let save = Save::default();
            server.accept_and_serve(vec![Packet::SaveInit(save)]).await;
        });

        // Connect as client and read
        let stream = TcpStream::connect((Ipv4Addr::LOCALHOST, port))
            .await
            .unwrap();
        let packet_stream = proto::read(stream);
        pin_mut!(packet_stream);

        let packet = packet_stream.next().await.unwrap().unwrap();
        assert!(matches!(packet, Packet::SaveInit(_)));

        server_task.await.unwrap();
    });

    async_test!(test_mock_emulator_version_mismatch, {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).await.unwrap();
        let port = listener.local_addr().unwrap().port();

        // Server sends wrong version
        let server_task = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            // Send wrong version (version 5 instead of 6)
            5u8.write(&mut stream).await.unwrap();
        });

        let stream = TcpStream::connect((Ipv4Addr::LOCALHOST, port))
            .await
            .unwrap();
        let packet_stream = proto::read(stream);
        pin_mut!(packet_stream);

        let result = packet_stream.next().await.unwrap();
        assert!(matches!(
            result,
            Err(ReadError::VersionMismatch {
                server: 6,
                client: 5
            })
        ));

        server_task.await.unwrap();
    });

    async_test!(test_mock_emulator_connection_drop, {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).await.unwrap();
        let port = listener.local_addr().unwrap().port();

        // Server sends version then drops connection without goodbye
        let server_task = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            VERSION.write(&mut stream).await.unwrap();
            Packet::SaveInit(Save::default())
                .write(&mut stream)
                .await
                .unwrap();
            // Drop connection without Goodbye
            drop(stream);
        });

        let stream = TcpStream::connect((Ipv4Addr::LOCALHOST, port))
            .await
            .unwrap();
        let packet_stream = proto::read(stream);
        pin_mut!(packet_stream);

        // First packet should be SaveInit
        let packet = packet_stream.next().await.unwrap().unwrap();
        assert!(matches!(packet, Packet::SaveInit(_)));

        // Next read should error (connection closed)
        let result = packet_stream.next().await;
        assert!(result.is_none() || matches!(result, Some(Err(_))));

        server_task.await.unwrap();
    });

    async_test!(test_mock_emulator_rapid_packets, {
        let server = MockEmulatorServer::new().await;
        let port = server.port();

        // Generate many packets
        let packets: Vec<_> = (0..100)
            .map(|_| Packet::SaveInit(Save::default()))
            .collect();

        let server_task = tokio::spawn({
            let packets = packets.clone();
            async move {
                server.accept_and_serve(packets).await;
            }
        });

        let stream = TcpStream::connect((Ipv4Addr::LOCALHOST, port))
            .await
            .unwrap();
        let packet_stream = proto::read(stream);
        pin_mut!(packet_stream);

        let mut count = 0;
        while let Some(result) = packet_stream.next().await {
            result.unwrap();
            count += 1;
        }

        assert_eq!(count, 100);
        server_task.await.unwrap();
    });
}

// ============================================================================
// Timeout and Error Handling Tests
// ============================================================================

mod timeout_handling {
    use super::*;

    async_test!(test_connection_timeout, {
        // Try to connect to a port that will timeout (blackhole)
        // Use a non-routable IP that will cause timeout
        let result = timeout(
            Duration::from_millis(100),
            TcpStream::connect((Ipv4Addr::new(10, 255, 255, 1), TCP_PORT)),
        )
        .await;

        assert!(result.is_err()); // Should timeout
    });

    async_test!(test_read_timeout_on_slow_server, {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).await.unwrap();
        let port = listener.local_addr().unwrap().port();

        // Server that accepts but doesn't send anything
        let _server_task = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            // Hold the connection open but don't send anything
            tokio::time::sleep(Duration::from_secs(10)).await;
            drop(stream);
        });

        let stream = TcpStream::connect((Ipv4Addr::LOCALHOST, port))
            .await
            .unwrap();
        let packet_stream = proto::read(stream);
        pin_mut!(packet_stream);

        // Try to read with a timeout
        let result = timeout(Duration::from_millis(100), packet_stream.next()).await;
        assert!(result.is_err()); // Should timeout waiting for version
    });

    async_test!(test_partial_packet_handling, {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).await.unwrap();
        let port = listener.local_addr().unwrap().port();

        // Server that sends version but then incomplete packet data
        let _server_task = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            // Send version
            VERSION.write(&mut stream).await.unwrap();
            // Send partial packet (just the discriminant byte, not the full packet)
            stream.write_all(&[1u8]).await.unwrap();
            // Don't send the rest - hold connection then drop
            tokio::time::sleep(Duration::from_millis(100)).await;
            drop(stream);
        });

        let stream = TcpStream::connect((Ipv4Addr::LOCALHOST, port))
            .await
            .unwrap();
        let packet_stream = proto::read(stream);
        pin_mut!(packet_stream);

        // Try to read - should eventually error or timeout
        let result = timeout(Duration::from_millis(200), packet_stream.next()).await;
        // Either timeout or receive error due to incomplete packet
        assert!(result.is_err() || matches!(result.unwrap(), Some(Err(_))));
    });

    async_test!(test_reconnection_simulation, {
        // Test that we can create multiple sequential connections
        // simulating reconnection behavior
        for _ in 0..3 {
            let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).await.unwrap();
            let port = listener.local_addr().unwrap().port();

            let server_task = tokio::spawn(async move {
                let (mut stream, _) = listener.accept().await.unwrap();
                VERSION.write(&mut stream).await.unwrap();
                Packet::SaveInit(Save::default())
                    .write(&mut stream)
                    .await
                    .unwrap();
                Packet::Goodbye.write(&mut stream).await.unwrap();
            });

            let stream = TcpStream::connect((Ipv4Addr::LOCALHOST, port))
                .await
                .unwrap();
            let packet_stream = proto::read(stream);
            pin_mut!(packet_stream);

            let packet = packet_stream.next().await.unwrap().unwrap();
            assert!(matches!(packet, Packet::SaveInit(_)));

            server_task.await.unwrap();

            // Small delay between connection attempts
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    });
}

// ============================================================================
// Bidirectional Communication Tests
// ============================================================================

mod bidirectional {
    use super::*;
    use tokio::sync::mpsc;

    async_test!(test_bidirectional_communication, {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).await.unwrap();
        let port = listener.local_addr().unwrap().port();

        // Create channels for coordination
        let (tx, mut rx) = mpsc::channel::<()>(1);

        // Server: Read from client, then respond
        let server_task = tokio::spawn(async move {
            let (mut tcp_stream, _) = listener.accept().await.unwrap();

            // Read version from client
            let version = u8::read(&mut tcp_stream).await.unwrap();
            assert_eq!(version, VERSION);

            // Read packet from client
            let packet = Packet::read(&mut tcp_stream).await.unwrap();
            assert!(matches!(packet, Packet::SaveInit(_)));

            // Respond with our own version and packets
            VERSION.write(&mut tcp_stream).await.unwrap();
            Packet::RamInit(Ram::default())
                .write(&mut tcp_stream)
                .await
                .unwrap();
            Packet::Goodbye.write(&mut tcp_stream).await.unwrap();

            // Signal completion
            tx.send(()).await.unwrap();
        });

        // Client: Send packets to server
        let mut stream = TcpStream::connect((Ipv4Addr::LOCALHOST, port))
            .await
            .unwrap();

        // Send version and packet
        VERSION.write(&mut stream).await.unwrap();
        Packet::SaveInit(Save::default())
            .write(&mut stream)
            .await
            .unwrap();

        // Wait for server response
        rx.recv().await.unwrap();

        server_task.await.unwrap();
    });
}

// ============================================================================
// Stress Tests
// ============================================================================

mod stress_tests {
    use super::*;

    async_test!(test_concurrent_connections, {
        let listener = Arc::new(TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).await.unwrap());
        let port = listener.local_addr().unwrap().port();
        let listener_clone = Arc::clone(&listener);

        // Track successful connections
        let success_count = Arc::new(Mutex::new(0u32));

        // Spawn multiple concurrent server handlers
        let server_count = success_count.clone();
        let server_task = tokio::spawn(async move {
            for _ in 0..5 {
                match timeout(Duration::from_millis(500), listener_clone.accept()).await {
                    Ok(Ok((mut stream, _))) => {
                        VERSION.write(&mut stream).await.ok();
                        Packet::SaveInit(Save::default())
                            .write(&mut stream)
                            .await
                            .ok();
                        Packet::Goodbye.write(&mut stream).await.ok();
                        *server_count.lock().await += 1;
                    }
                    _ => break,
                }
            }
        });

        // Spawn multiple client connections concurrently
        let client_tasks: Vec<_> = (0..5)
            .map(|_| {
                tokio::spawn(async move {
                    if let Ok(stream) = TcpStream::connect((Ipv4Addr::LOCALHOST, port)).await {
                        let packet_stream = proto::read(stream);
                        pin_mut!(packet_stream);
                        while let Some(Ok(_)) = packet_stream.next().await {}
                    }
                })
            })
            .collect();

        // Wait for all clients
        for task in client_tasks {
            let _ = task.await;
        }

        // Allow server to finish
        let _ = timeout(Duration::from_millis(500), server_task).await;

        // At least some connections should have succeeded
        let count = *success_count.lock().await;
        assert!(count > 0, "At least one connection should succeed");
    });
}
