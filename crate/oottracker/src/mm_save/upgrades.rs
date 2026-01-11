//! Inventory upgrades (wallet, bags, etc.)

use bitflags::bitflags;

bitflags! {
    /// Inventory upgrades (wallet, bags, etc.)
    #[derive(Default)]
    pub struct MmUpgrades: u32 {
        // Quiver (bits 0-2)
        const QUIVER_30 = 0x1;
        const QUIVER_40 = 0x2;
        const QUIVER_50 = 0x3;
        const QUIVER_MASK = 0x7;

        // Bomb bag (bits 3-5)
        const BOMB_BAG_20 = 0x8;
        const BOMB_BAG_30 = 0x10;
        const BOMB_BAG_40 = 0x18;
        const BOMB_BAG_MASK = 0x38;

        // Strength (bits 6-8) - unused in MM
        const STRENGTH_MASK = 0x1C0;

        // Scale (bits 9-11) - unused in MM
        const SCALE_MASK = 0xE00;

        // Wallet (bits 12-13)
        const ADULTS_WALLET = 0x1000;
        const GIANTS_WALLET = 0x2000;
        const WALLET_MASK = 0x3000;

        // Deku stick capacity (bits 17-19)
        const DEKU_STICK_10 = 0x20000;
        const DEKU_STICK_20 = 0x40000;
        const DEKU_STICK_30 = 0x60000;
        const DEKU_STICK_MASK = 0xE0000;

        // Deku nut capacity (bits 20-22)
        const DEKU_NUT_20 = 0x100000;
        const DEKU_NUT_30 = 0x200000;
        const DEKU_NUT_40 = 0x300000;
        const DEKU_NUT_MASK = 0x700000;
    }
}

impl MmUpgrades {
    pub fn wallet(&self) -> MmUpgrades {
        *self & MmUpgrades::WALLET_MASK
    }

    pub fn set_wallet(&mut self, wallet: MmUpgrades) {
        self.remove(MmUpgrades::WALLET_MASK);
        self.insert(wallet & MmUpgrades::WALLET_MASK);
    }

    pub fn bomb_bag(&self) -> MmUpgrades {
        *self & MmUpgrades::BOMB_BAG_MASK
    }

    pub fn quiver(&self) -> MmUpgrades {
        *self & MmUpgrades::QUIVER_MASK
    }
}
