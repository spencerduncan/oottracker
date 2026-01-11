//! Serialization: to_save_data and Protocol implementation.

use crate::mm_save::{
    constants::{MM_PERM_SCENE_SIZE, MM_SIZE},
    offsets::{vanilla_offsets, MmRomType},
    save::MmSave,
};

impl MmSave {
    /// Convert the save state back to raw bytes
    pub fn to_save_data(&self) -> Vec<u8> {
        use vanilla_offsets::*;

        let mut buf = vec![0u8; MM_SIZE];

        // Write player form
        buf[PLAYER_FORM] = self.player_form as u8;

        // Write health
        buf[HEALTH_CAPACITY..HEALTH_CAPACITY + 2]
            .copy_from_slice(&self.health_capacity.to_be_bytes());
        buf[HEALTH..HEALTH + 2].copy_from_slice(&self.health.to_be_bytes());

        // Write magic
        buf[MAGIC_LEVEL] = self.magic as u8;

        // Write double defense
        buf[DOUBLE_DEFENSE] = if self.double_defense { 1 } else { 0 };

        // Write rupees
        buf[RUPEES..RUPEES + 2].copy_from_slice(&self.rupees.to_be_bytes());

        // Write sword and shield
        buf[SWORD_SHIELD] = (self.sword as u8) | ((self.shield as u8) << 4);

        // Write quest items
        buf[QUEST_ITEMS..QUEST_ITEMS + 4].copy_from_slice(&self.quest_items.bits().to_be_bytes());

        // Write upgrades
        buf[UPGRADES..UPGRADES + 4].copy_from_slice(&self.upgrades.bits().to_be_bytes());

        // Write dungeon items
        buf[DUNGEON_ITEMS] = self.dungeon_items.woodfall.bits();
        buf[DUNGEON_ITEMS + 1] = self.dungeon_items.snowhead.bits();
        buf[DUNGEON_ITEMS + 2] = self.dungeon_items.great_bay.bits();
        buf[DUNGEON_ITEMS + 3] = self.dungeon_items.stone_tower.bits();

        // Write small keys
        buf[SMALL_KEYS] = self.small_keys.woodfall;
        buf[SMALL_KEYS + 1] = self.small_keys.snowhead;
        buf[SMALL_KEYS + 2] = self.small_keys.great_bay;
        buf[SMALL_KEYS + 3] = self.small_keys.stone_tower;

        // Write stray fairies
        buf[STRAY_FAIRIES] = self.stray_fairies.clock_town;
        buf[STRAY_FAIRIES + 1] = self.stray_fairies.woodfall;
        buf[STRAY_FAIRIES + 2] = self.stray_fairies.snowhead;
        buf[STRAY_FAIRIES + 3] = self.stray_fairies.great_bay;
        buf[STRAY_FAIRIES + 4] = self.stray_fairies.stone_tower;

        // Write skulltula tokens
        buf[SKULL_SWAMP..SKULL_SWAMP + 2].copy_from_slice(&self.skull_tokens_swamp.to_be_bytes());
        buf[SKULL_OCEAN..SKULL_OCEAN + 2].copy_from_slice(&self.skull_tokens_ocean.to_be_bytes());

        // Write time state
        buf[DAY..DAY + 4].copy_from_slice(&self.day.to_be_bytes());
        buf[TIME..TIME + 2].copy_from_slice(&self.time.to_be_bytes());
        buf[IS_NIGHT] = if self.is_night { 1 } else { 0 };

        // Write permanent scene flags
        for (i, scene) in self.permanent_scene_flags.iter().enumerate() {
            let base = PERM_SCENE_FLAGS + (i * MM_PERM_SCENE_SIZE);
            buf[base..base + 4].copy_from_slice(&scene.chest.to_be_bytes());
            buf[base + 4..base + 8].copy_from_slice(&scene.switch0.to_be_bytes());
            buf[base + 8..base + 12].copy_from_slice(&scene.switch1.to_be_bytes());
            buf[base + 12..base + 16].copy_from_slice(&scene.cleared_room.to_be_bytes());
            buf[base + 16..base + 20].copy_from_slice(&scene.collectible.to_be_bytes());
            buf[base + 20..base + 24].copy_from_slice(&scene.cleared_floors.to_be_bytes());
            buf[base + 24..base + 28].copy_from_slice(&scene.rooms.to_be_bytes());
        }

        buf
    }
}

// ============================================================================
// Protocol Implementation for MmSave
// ============================================================================

impl async_proto::Protocol for MmSave {
    fn read<'a, R: tokio::io::AsyncRead + Unpin + Send + 'a>(
        stream: &'a mut R,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self, async_proto::ReadError>> + Send + 'a>,
    > {
        Box::pin(async move {
            use tokio::io::AsyncReadExt;
            let mut buf = vec![0u8; MM_SIZE];
            stream.read_exact(&mut buf).await?;
            let rom_type = MmRomType::from_env();
            MmSave::from_save_data_with_type(&buf, rom_type)
                .map_err(|e| async_proto::ReadError::Custom(format!("MM decode error: {:?}", e)))
        })
    }

    fn write<'a, W: tokio::io::AsyncWrite + Unpin + Send + 'a>(
        &'a self,
        sink: &'a mut W,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<(), async_proto::WriteError>> + Send + 'a>,
    > {
        Box::pin(async move {
            use tokio::io::AsyncWriteExt;
            let data = self.to_save_data();
            sink.write_all(&data).await?;
            Ok(())
        })
    }

    fn read_sync(stream: &mut impl std::io::Read) -> Result<Self, async_proto::ReadError> {
        let mut buf = vec![0u8; MM_SIZE];
        stream.read_exact(&mut buf)?;
        let rom_type = MmRomType::from_env();
        MmSave::from_save_data_with_type(&buf, rom_type)
            .map_err(|e| async_proto::ReadError::Custom(format!("MM decode error: {:?}", e)))
    }

    fn write_sync(&self, sink: &mut impl std::io::Write) -> Result<(), async_proto::WriteError> {
        let data = self.to_save_data();
        sink.write_all(&data)?;
        Ok(())
    }
}
