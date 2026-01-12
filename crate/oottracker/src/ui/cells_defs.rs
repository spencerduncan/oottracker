cells! {
    GoMode: Simple {
        img: ImageInfo::extra("go_mode"),
        active: Box::new(|state| match state.knowledge.progression_mode {
            ProgressionMode::Go | ProgressionMode::Done => true,
            ProgressionMode::Bk | ProgressionMode::Normal => false,
        }),
        toggle: Box::new(|state| {
            let new_mode = match state.knowledge.progression_mode {
                ProgressionMode::Done => ProgressionMode::Done, // only the racetime integration may toggle .done for now
                ProgressionMode::Go => ProgressionMode::Normal,
                ProgressionMode::Bk | ProgressionMode::Normal => ProgressionMode::Go,
            };
            state.knowledge.progression_mode = new_mode;
        }),
    },
    GoBk: GoBk,
    LightMedallion: Medallion(Medallion::Light),
    ForestMedallion: Medallion(Medallion::Forest),
    FireMedallion: Medallion(Medallion::Fire),
    WaterMedallion: Medallion(Medallion::Water),
    ShadowMedallion: Medallion(Medallion::Shadow),
    SpiritMedallion: Medallion(Medallion::Spirit),
    LightMedallionLocation: MedallionLocation(Medallion::Light),
    ForestMedallionLocation: MedallionLocation(Medallion::Forest),
    FireMedallionLocation: MedallionLocation(Medallion::Fire),
    WaterMedallionLocation: MedallionLocation(Medallion::Water),
    ShadowMedallionLocation: MedallionLocation(Medallion::Shadow),
    SpiritMedallionLocation: MedallionLocation(Medallion::Spirit),
    LightMedallionWithLocation: MedallionWithLocation(Medallion::Light),
    ForestMedallionWithLocation: MedallionWithLocation(Medallion::Forest),
    FireMedallionWithLocation: MedallionWithLocation(Medallion::Fire),
    WaterMedallionWithLocation: MedallionWithLocation(Medallion::Water),
    ShadowMedallionWithLocation: MedallionWithLocation(Medallion::Shadow),
    SpiritMedallionWithLocation: MedallionWithLocation(Medallion::Spirit),
    AdultTrade: Sequence {
        idx: Box::new(|state| match state.ram.save.inv.adult_trade_item {
            AdultTradeItem::None => 0,
            AdultTradeItem::PocketEgg => 1,
            AdultTradeItem::PocketCucco => 2,
            AdultTradeItem::Cojiro => 3,
            AdultTradeItem::OddMushroom => 4,
            AdultTradeItem::OddPotion => 5,
            AdultTradeItem::PoachersSaw => 6,
            AdultTradeItem::BrokenSword => 7,
            AdultTradeItem::Prescription => 8,
            AdultTradeItem::EyeballFrog => 9,
            AdultTradeItem::Eyedrops => 10,
            AdultTradeItem::ClaimCheck => 11,
        }),
        img: Box::new(|state| match state.ram.save.inv.adult_trade_item {
            AdultTradeItem::None => (false, ImageInfo::new("blue_egg")),
            AdultTradeItem::PocketEgg | AdultTradeItem::PocketCucco => (true, ImageInfo::new("blue_egg")),
            AdultTradeItem::Cojiro => (true, ImageInfo::new("cojiro")),
            AdultTradeItem::OddMushroom => (true, ImageInfo::new("odd_mushroom")),
            AdultTradeItem::OddPotion => (true, ImageInfo::new("odd_poultice")),
            AdultTradeItem::PoachersSaw => (true, ImageInfo::new("poachers_saw")),
            AdultTradeItem::BrokenSword => (true, ImageInfo::new("broken_sword")),
            AdultTradeItem::Prescription => (true, ImageInfo::new("prescription")),
            AdultTradeItem::EyeballFrog => (true, ImageInfo::new("eyeball_frog")),
            AdultTradeItem::Eyedrops => (true, ImageInfo::new("eye_drops")),
            AdultTradeItem::ClaimCheck => (true, ImageInfo::new("claim_check")),
        }),
        increment: Box::new(|state| state.ram.save.inv.adult_trade_item = match state.ram.save.inv.adult_trade_item {
            AdultTradeItem::None => AdultTradeItem::PocketEgg,
            AdultTradeItem::PocketEgg => AdultTradeItem::PocketCucco,
            AdultTradeItem::PocketCucco => AdultTradeItem::Cojiro,
            AdultTradeItem::Cojiro => AdultTradeItem::OddMushroom,
            AdultTradeItem::OddMushroom => AdultTradeItem::OddPotion,
            AdultTradeItem::OddPotion => AdultTradeItem::PoachersSaw,
            AdultTradeItem::PoachersSaw => AdultTradeItem::BrokenSword,
            AdultTradeItem::BrokenSword => AdultTradeItem::Prescription,
            AdultTradeItem::Prescription => AdultTradeItem::EyeballFrog,
            AdultTradeItem::EyeballFrog => AdultTradeItem::Eyedrops,
            AdultTradeItem::Eyedrops => AdultTradeItem::ClaimCheck,
            AdultTradeItem::ClaimCheck => AdultTradeItem::None,
        }),
        decrement: Box::new(|state| state.ram.save.inv.adult_trade_item = match state.ram.save.inv.adult_trade_item {
            AdultTradeItem::None => AdultTradeItem::ClaimCheck,
            AdultTradeItem::PocketEgg => AdultTradeItem::None,
            AdultTradeItem::PocketCucco => AdultTradeItem::PocketEgg,
            AdultTradeItem::Cojiro => AdultTradeItem::PocketEgg,
            AdultTradeItem::OddMushroom => AdultTradeItem::Cojiro,
            AdultTradeItem::OddPotion => AdultTradeItem::OddMushroom,
            AdultTradeItem::PoachersSaw => AdultTradeItem::OddPotion,
            AdultTradeItem::BrokenSword => AdultTradeItem::PoachersSaw,
            AdultTradeItem::Prescription => AdultTradeItem::BrokenSword,
            AdultTradeItem::EyeballFrog => AdultTradeItem::Prescription,
            AdultTradeItem::Eyedrops => AdultTradeItem::EyeballFrog,
            AdultTradeItem::ClaimCheck => AdultTradeItem::Eyedrops,
        }),
    },
    AdultTradeNoChicken: Sequence {
        idx: Box::new(|state| match state.ram.save.inv.adult_trade_item {
            AdultTradeItem::None => 0,
            AdultTradeItem::PocketEgg | AdultTradeItem::PocketCucco => 1,
            AdultTradeItem::Cojiro => 2,
            AdultTradeItem::OddMushroom => 3,
            AdultTradeItem::OddPotion => 4,
            AdultTradeItem::PoachersSaw => 5,
            AdultTradeItem::BrokenSword => 6,
            AdultTradeItem::Prescription => 7,
            AdultTradeItem::EyeballFrog => 8,
            AdultTradeItem::Eyedrops => 9,
            AdultTradeItem::ClaimCheck => 10,
        }),
        img: Box::new(|state| match state.ram.save.inv.adult_trade_item {
            AdultTradeItem::None => (false, ImageInfo::new("blue_egg")),
            AdultTradeItem::PocketEgg | AdultTradeItem::PocketCucco => (true, ImageInfo::new("blue_egg")),
            AdultTradeItem::Cojiro => (true, ImageInfo::new("cojiro")),
            AdultTradeItem::OddMushroom => (true, ImageInfo::new("odd_mushroom")),
            AdultTradeItem::OddPotion => (true, ImageInfo::new("odd_poultice")),
            AdultTradeItem::PoachersSaw => (true, ImageInfo::new("poachers_saw")),
            AdultTradeItem::BrokenSword => (true, ImageInfo::new("broken_sword")),
            AdultTradeItem::Prescription => (true, ImageInfo::new("prescription")),
            AdultTradeItem::EyeballFrog => (true, ImageInfo::new("eyeball_frog")),
            AdultTradeItem::Eyedrops => (true, ImageInfo::new("eye_drops")),
            AdultTradeItem::ClaimCheck => (true, ImageInfo::new("claim_check")),
        }),
        increment: Box::new(|state| state.ram.save.inv.adult_trade_item = match state.ram.save.inv.adult_trade_item {
            AdultTradeItem::None => AdultTradeItem::PocketEgg,
            AdultTradeItem::PocketEgg | AdultTradeItem::PocketCucco => AdultTradeItem::Cojiro,
            AdultTradeItem::Cojiro => AdultTradeItem::OddMushroom,
            AdultTradeItem::OddMushroom => AdultTradeItem::OddPotion,
            AdultTradeItem::OddPotion => AdultTradeItem::PoachersSaw,
            AdultTradeItem::PoachersSaw => AdultTradeItem::BrokenSword,
            AdultTradeItem::BrokenSword => AdultTradeItem::Prescription,
            AdultTradeItem::Prescription => AdultTradeItem::EyeballFrog,
            AdultTradeItem::EyeballFrog => AdultTradeItem::Eyedrops,
            AdultTradeItem::Eyedrops => AdultTradeItem::ClaimCheck,
            AdultTradeItem::ClaimCheck => AdultTradeItem::None,
        }),
        decrement: Box::new(|state| state.ram.save.inv.adult_trade_item = match state.ram.save.inv.adult_trade_item {
            AdultTradeItem::None => AdultTradeItem::ClaimCheck,
            AdultTradeItem::PocketEgg | AdultTradeItem::PocketCucco => AdultTradeItem::None,
            AdultTradeItem::Cojiro => AdultTradeItem::PocketEgg,
            AdultTradeItem::OddMushroom => AdultTradeItem::Cojiro,
            AdultTradeItem::OddPotion => AdultTradeItem::OddMushroom,
            AdultTradeItem::PoachersSaw => AdultTradeItem::OddPotion,
            AdultTradeItem::BrokenSword => AdultTradeItem::PoachersSaw,
            AdultTradeItem::Prescription => AdultTradeItem::BrokenSword,
            AdultTradeItem::EyeballFrog => AdultTradeItem::Prescription,
            AdultTradeItem::Eyedrops => AdultTradeItem::EyeballFrog,
            AdultTradeItem::ClaimCheck => AdultTradeItem::Eyedrops,
        }),
    },
    Skulltula: Count {
        dimmed_img: ImageInfo::new("golden_skulltula"),
        img: ImageInfo::new("skulls"),
        get: Box::new(|state| state.ram.save.skull_tokens),
        set: Box::new(|state, value| state.ram.save.skull_tokens = value),
        max: 100,
        step: 1,
    },
    SkulltulaTens: Count {
        dimmed_img: ImageInfo::new("golden_skulltula"),
        img: ImageInfo::new("skulls"),
        get: Box::new(|state| state.ram.save.skull_tokens),
        set: Box::new(|state, value| state.ram.save.skull_tokens = value),
        max: 50,
        step: 10,
    },
    KokiriEmerald: Stone(Stone::KokiriEmerald),
    GoronRuby: Stone(Stone::GoronRuby),
    ZoraSapphire: Stone(Stone::ZoraSapphire),
    KokiriEmeraldLocation: StoneLocation(Stone::KokiriEmerald),
    GoronRubyLocation: StoneLocation(Stone::GoronRuby),
    ZoraSapphireLocation: StoneLocation(Stone::ZoraSapphire),
    KokiriEmeraldWithLocation: StoneWithLocation(Stone::KokiriEmerald),
    GoronRubyWithLocation: StoneWithLocation(Stone::GoronRuby),
    ZoraSapphireWithLocation: StoneWithLocation(Stone::ZoraSapphire),
    Bottle: OptionalOverlay {
        main_img: ImageInfo::new("bottle"),
        overlay_img: ImageInfo::new("letter"),
        active: Box::new(|state| (state.ram.save.inv.emptiable_bottles() > 0, state.ram.save.inv.has_rutos_letter())), //TODO also show Ruto's letter as active if it has been delivered or Open Fountain is known (https://github.com/fenhl/oottracker/issues/21)
        toggle_main: Box::new(|state| {
            let new_val = if state.ram.save.inv.emptiable_bottles() > 0 { 0 } else { 1 };
            state.ram.save.inv.set_emptiable_bottles(new_val);
        }),
        toggle_overlay: Box::new(|state| state.ram.save.inv.toggle_rutos_letter()),
    },
    NumBottles: DynamicCount {
        dimmed_img: ImageInfo::new("bottle"),
        img: ImageInfo::new("UNIMPLEMENTED"), //TODO make images for 1–4 bottles
        get: Box::new(|state| state.ram.save.inv.emptiable_bottles()),
        set: Box::new(|state, value| state.ram.save.inv.set_emptiable_bottles(value.min(state.max_bottles))),
        max_fn: Box::new(|state| state.max_bottles),
        step: 1,
    },
    RutosLetter: Simple {
        img: ImageInfo::new("UNIMPLEMENTED"),
        active: Box::new(|state| state.ram.save.inv.has_rutos_letter()), //TODO also show Ruto's letter as active if it has been delivered or Open Fountain is known (https://github.com/fenhl/oottracker/issues/21)
        toggle: Box::new(|state| state.ram.save.inv.toggle_rutos_letter()),
    },
    Scale: Sequence {
        idx: Box::new(|state| match state.ram.save.upgrades.scale() {
            Upgrades::SILVER_SCALE => 1,
            Upgrades::GOLD_SCALE => 2,
            _ => 0,
        }),
        img: Box::new(|state| match state.ram.save.upgrades.scale() {
            Upgrades::SILVER_SCALE => (true, ImageInfo::new("silver_scale")),
            Upgrades::GOLD_SCALE => (true, ImageInfo::new("gold_scale")),
            _ => (false, ImageInfo::new("silver_scale")),
        }),
        increment: Box::new(|state| {
            let new_val = match state.ram.save.upgrades.scale() {
                Upgrades::SILVER_SCALE => Upgrades::GOLD_SCALE,
                Upgrades::GOLD_SCALE => Upgrades::NONE,
                _ => Upgrades::SILVER_SCALE,
            };
            state.ram.save.upgrades.set_scale(new_val);
        }),
        decrement: Box::new(|state| {
            let new_val = match state.ram.save.upgrades.scale() {
                Upgrades::SILVER_SCALE => Upgrades::NONE,
                Upgrades::GOLD_SCALE => Upgrades::SILVER_SCALE,
                _ => Upgrades::GOLD_SCALE,
            };
            state.ram.save.upgrades.set_scale(new_val);
        }),
    },
    Slingshot: Simple {
        img: ImageInfo::new("slingshot"),
        active: Box::new(|state| state.ram.save.inv.slingshot),
        toggle: Box::new(|state| {
            state.ram.save.inv.slingshot = !state.ram.save.inv.slingshot;
            let new_bullet_bag = if state.ram.save.inv.slingshot { Upgrades::BULLET_BAG_30 } else { Upgrades::NONE };
            state.ram.save.upgrades.set_bullet_bag(new_bullet_bag);
        }),
    },
    BulletBag: Sequence {
        idx: Box::new(|state| match state.ram.save.upgrades.bullet_bag() {
            Upgrades::BULLET_BAG_30 => 1,
            Upgrades::BULLET_BAG_40 => 2,
            Upgrades::BULLET_BAG_50 => 3,
            _ => 0,
        }),
        img: Box::new(|state| (state.ram.save.inv.slingshot, ImageInfo::new("slingshot"))),
        increment: Box::new(|state| {
            let new_bullet_bag = match state.ram.save.upgrades.bullet_bag() {
                Upgrades::BULLET_BAG_30 => Upgrades::BULLET_BAG_40,
                Upgrades::BULLET_BAG_40 => Upgrades::BULLET_BAG_50,
                Upgrades::BULLET_BAG_50 => Upgrades::NONE,
                _ => Upgrades::BULLET_BAG_30,
            };
            state.ram.save.upgrades.set_bullet_bag(new_bullet_bag);
            state.ram.save.inv.slingshot = state.ram.save.upgrades.bullet_bag() != Upgrades::NONE;
        }),
        decrement: Box::new(|state| {
            let new_bullet_bag = match state.ram.save.upgrades.bullet_bag() {
                Upgrades::BULLET_BAG_30 => Upgrades::NONE,
                Upgrades::BULLET_BAG_40 => Upgrades::BULLET_BAG_30,
                Upgrades::BULLET_BAG_50 => Upgrades::BULLET_BAG_40,
                _ => Upgrades::BULLET_BAG_50,
            };
            state.ram.save.upgrades.set_bullet_bag(new_bullet_bag);
            state.ram.save.inv.slingshot = state.ram.save.upgrades.bullet_bag() != Upgrades::NONE;
        }),
    },
    Bombs: Overlay {
        main_img: ImageInfo::new("bomb_bag"),
        overlay_img: ImageInfo::new("bombchu"),
        active: Box::new(|state| (state.ram.save.upgrades.bomb_bag() != Upgrades::NONE, state.ram.save.inv.bombchus)),
        toggle_main: Box::new(|state| if state.ram.save.upgrades.bomb_bag() == Upgrades::NONE {
            state.ram.save.upgrades.set_bomb_bag(Upgrades::BOMB_BAG_20);
        } else {
            state.ram.save.upgrades.set_bomb_bag(Upgrades::NONE);
        }),
        toggle_overlay: Box::new(|state| state.ram.save.inv.bombchus = !state.ram.save.inv.bombchus),
    },
    BombBag: Sequence {
        idx: Box::new(|state| match state.ram.save.upgrades.bomb_bag() {
            Upgrades::BOMB_BAG_20 => 1,
            Upgrades::BOMB_BAG_30 => 2,
            Upgrades::BOMB_BAG_40 => 3,
            _ => 0,
        }),
        img: Box::new(|state| (state.ram.save.upgrades.bomb_bag() != Upgrades::NONE, ImageInfo::new("bomb_bag"))),
        increment: Box::new(|state| {
            let new_val = match state.ram.save.upgrades.bomb_bag() {
                Upgrades::BOMB_BAG_20 => Upgrades::BOMB_BAG_30,
                Upgrades::BOMB_BAG_30 => Upgrades::BOMB_BAG_40,
                Upgrades::BOMB_BAG_40 => Upgrades::NONE,
                _ => Upgrades::BOMB_BAG_20,
            };
            state.ram.save.upgrades.set_bomb_bag(new_val);
        }),
        decrement: Box::new(|state| {
            let new_val = match state.ram.save.upgrades.bomb_bag() {
                Upgrades::BOMB_BAG_20 => Upgrades::NONE,
                Upgrades::BOMB_BAG_30 => Upgrades::BOMB_BAG_20,
                Upgrades::BOMB_BAG_40 => Upgrades::BOMB_BAG_30,
                _ => Upgrades::BOMB_BAG_40,
            };
            state.ram.save.upgrades.set_bomb_bag(new_val);
        }),
    },
    Bombchus: Simple {
        img: ImageInfo::new("UNIMPLEMENTED"),
        active: Box::new(|state| state.ram.save.inv.bombchus),
        toggle: Box::new(|state| state.ram.save.inv.bombchus = !state.ram.save.inv.bombchus),
    },
    Boomerang: Simple {
        img: ImageInfo::new("boomerang"),
        active: Box::new(|state| state.ram.save.inv.boomerang),
        toggle: Box::new(|state| state.ram.save.inv.boomerang = !state.ram.save.inv.boomerang),
    },
    Strength: Sequence {
        idx: Box::new(|state| match state.ram.save.upgrades.strength() {
            Upgrades::GORON_BRACELET => 1,
            Upgrades::SILVER_GAUNTLETS => 2,
            Upgrades::GOLD_GAUNTLETS => 3,
            _ => 0,
        }),
        img: Box::new(|state| match state.ram.save.upgrades.strength() {
            Upgrades::GORON_BRACELET => (true, ImageInfo::new("goron_bracelet")),
            Upgrades::SILVER_GAUNTLETS => (true, ImageInfo::new("silver_gauntlets")),
            Upgrades::GOLD_GAUNTLETS => (true, ImageInfo::new("gold_gauntlets")),
            _ => (false, ImageInfo::new("goron_bracelet")),
        }),
        increment: Box::new(|state| {
            let new_val = match state.ram.save.upgrades.strength() {
                Upgrades::GORON_BRACELET => Upgrades::SILVER_GAUNTLETS,
                Upgrades::SILVER_GAUNTLETS => Upgrades::GOLD_GAUNTLETS,
                Upgrades::GOLD_GAUNTLETS => Upgrades::NONE,
                _ => Upgrades::GORON_BRACELET,
            };
            state.ram.save.upgrades.set_strength(new_val);
        }),
        decrement: Box::new(|state| {
            let new_val = match state.ram.save.upgrades.strength() {
                Upgrades::GORON_BRACELET => Upgrades::NONE,
                Upgrades::SILVER_GAUNTLETS => Upgrades::GORON_BRACELET,
                Upgrades::GOLD_GAUNTLETS => Upgrades::SILVER_GAUNTLETS,
                _ => Upgrades::GOLD_GAUNTLETS,
            };
            state.ram.save.upgrades.set_strength(new_val);
        }),
    },
    Magic: Simple {
        img: ImageInfo::new("magic"),
        active: Box::new(|state| state.ram.save.magic != MagicCapacity::None),
        toggle: Box::new(|state| if state.ram.save.magic == MagicCapacity::None {
            state.ram.save.magic = MagicCapacity::Small;
        } else {
            state.ram.save.magic = MagicCapacity::None;
        }),
    },
    MagicLens: MagicLens,
    MagicCapacity: Sequence {
        idx: Box::new(|state| match state.ram.save.magic {
            MagicCapacity::None => 0,
            MagicCapacity::Small => 1,
            MagicCapacity::Large => 2,
        }),
        img: Box::new(|state| (state.ram.save.magic != MagicCapacity::None, ImageInfo::new("magic"))),
        increment: Box::new(|state| state.ram.save.magic = match state.ram.save.magic {
            MagicCapacity::None => MagicCapacity::Small,
            MagicCapacity::Small => MagicCapacity::Large,
            MagicCapacity::Large => MagicCapacity::None,
        }),
        decrement: Box::new(|state| state.ram.save.magic = match state.ram.save.magic {
            MagicCapacity::None => MagicCapacity::Large,
            MagicCapacity::Small => MagicCapacity::None,
            MagicCapacity::Large => MagicCapacity::Small,
        }),
    },
    Lens: Simple {
        img: ImageInfo::new("lens"),
        active: Box::new(|state| state.ram.save.inv.lens),
        toggle: Box::new(|state| state.ram.save.inv.lens = !state.ram.save.inv.lens),
    },
    DinsFarores: Composite {
        left_img: ImageInfo::new("dins_fire"),
        right_img: ImageInfo::new("faores_wind"),
        both_img: ImageInfo::new("composite_magic"),
        active: Box::new(|state| (state.ram.save.inv.dins_fire, state.ram.save.inv.farores_wind)),
        toggle_left: Box::new(|state| state.ram.save.inv.dins_fire = !state.ram.save.inv.dins_fire),
        toggle_right: Box::new(|state| state.ram.save.inv.farores_wind = !state.ram.save.inv.farores_wind),
    },
    Spells: Spells,
    DinsFire: Simple {
        img: ImageInfo::new("dins_fire"),
        active: Box::new(|state| state.ram.save.inv.dins_fire),
        toggle: Box::new(|state| state.ram.save.inv.dins_fire = !state.ram.save.inv.dins_fire),
    },
    FaroresWind: Simple {
        img: ImageInfo::new("faores_wind"),
        active: Box::new(|state| state.ram.save.inv.farores_wind),
        toggle: Box::new(|state| state.ram.save.inv.farores_wind = !state.ram.save.inv.farores_wind),
    },
    NayrusLove: Simple {
        img: ImageInfo::extra("nayrus_love"),
        active: Box::new(|state| state.ram.save.inv.nayrus_love),
        toggle: Box::new(|state| state.ram.save.inv.nayrus_love = !state.ram.save.inv.nayrus_love),
    },
    Hookshot: Sequence {
        idx: Box::new(|state| match state.ram.save.inv.hookshot {
            Hookshot::None => 0,
            Hookshot::Hookshot => 1,
            Hookshot::Longshot => 2,
        }),
        img: Box::new(|state| match state.ram.save.inv.hookshot {
            Hookshot::None => (false, ImageInfo::new("hookshot")),
            Hookshot::Hookshot => (true, ImageInfo::new("hookshot_accessible")),
            Hookshot::Longshot => (true, ImageInfo::new("longshot_accessible")),
        }),
        increment: Box::new(|state| state.ram.save.inv.hookshot = match state.ram.save.inv.hookshot {
            Hookshot::None => Hookshot::Hookshot,
            Hookshot::Hookshot => Hookshot::Longshot,
            Hookshot::Longshot => Hookshot::None,
        }),
        decrement: Box::new(|state| state.ram.save.inv.hookshot = match state.ram.save.inv.hookshot {
            Hookshot::None => Hookshot::Longshot,
            Hookshot::Hookshot => Hookshot::None,
            Hookshot::Longshot => Hookshot::Hookshot,
        }),
    },
    Bow: OptionalOverlay {
        main_img: ImageInfo::new("bow"),
        overlay_img: ImageInfo::new("ice_arrows"),
        active: Box::new(|state| (state.ram.save.inv.bow, state.ram.save.inv.ice_arrows)),
        toggle_main: Box::new(|state| {
            state.ram.save.inv.bow = !state.ram.save.inv.bow;
            let new_quiver = if state.ram.save.inv.bow { Upgrades::QUIVER_30 } else { Upgrades::NONE };
            state.ram.save.upgrades.set_quiver(new_quiver);
        }),
        toggle_overlay: Box::new(|state| state.ram.save.inv.ice_arrows = !state.ram.save.inv.ice_arrows),
    },
    IceArrows: Simple {
        img: ImageInfo::new("ice_trap"),
        active: Box::new(|state| state.ram.save.inv.ice_arrows),
        toggle: Box::new(|state| state.ram.save.inv.ice_arrows = !state.ram.save.inv.ice_arrows),
    },
    Quiver: Sequence {
        idx: Box::new(|state| match state.ram.save.upgrades.quiver() {
            Upgrades::QUIVER_30 => 1,
            Upgrades::QUIVER_40 => 2,
            Upgrades::QUIVER_50 => 3,
            _ => 0,
        }),
        img: Box::new(|state| (state.ram.save.inv.bow, ImageInfo::new("bow"))),
        increment: Box::new(|state| {
            let new_quiver = match state.ram.save.upgrades.quiver() {
                Upgrades::QUIVER_30 => Upgrades::QUIVER_40,
                Upgrades::QUIVER_40 => Upgrades::QUIVER_50,
                Upgrades::QUIVER_50 => Upgrades::NONE,
                _ => Upgrades::QUIVER_30,
            };
            state.ram.save.upgrades.set_quiver(new_quiver);
            state.ram.save.inv.bow = state.ram.save.upgrades.quiver() != Upgrades::NONE;
        }),
        decrement: Box::new(|state| {
            let new_quiver = match state.ram.save.upgrades.quiver() {
                Upgrades::QUIVER_30 => Upgrades::NONE,
                Upgrades::QUIVER_40 => Upgrades::QUIVER_30,
                Upgrades::QUIVER_50 => Upgrades::QUIVER_40,
                _ => Upgrades::QUIVER_50,
            };
            state.ram.save.upgrades.set_quiver(new_quiver);
            state.ram.save.inv.bow = state.ram.save.upgrades.quiver() != Upgrades::NONE;
        }),
    },
    Arrows: Composite {
        left_img: ImageInfo::new("fire_arrows"),
        right_img: ImageInfo::new("light_arrows"),
        both_img: ImageInfo::new("composite_arrows"),
        active: Box::new(|state| (state.ram.save.inv.fire_arrows, state.ram.save.inv.light_arrows)),
        toggle_left: Box::new(|state| state.ram.save.inv.fire_arrows = !state.ram.save.inv.fire_arrows),
        toggle_right: Box::new(|state| state.ram.save.inv.light_arrows = !state.ram.save.inv.light_arrows),
    },
    FireArrows: Simple {
        img: ImageInfo::new("fire_arrows"),
        active: Box::new(|state| state.ram.save.inv.fire_arrows),
        toggle: Box::new(|state| state.ram.save.inv.fire_arrows = !state.ram.save.inv.fire_arrows),
    },
    LightArrows: Simple {
        img: ImageInfo::new("light_arrows"),
        active: Box::new(|state| state.ram.save.inv.light_arrows),
        toggle: Box::new(|state| state.ram.save.inv.light_arrows = !state.ram.save.inv.light_arrows),
    },
    Hammer: Simple {
        img: ImageInfo::new("hammer"),
        active: Box::new(|state| state.ram.save.inv.hammer),
        toggle: Box::new(|state| state.ram.save.inv.hammer = !state.ram.save.inv.hammer),
    },
    Boots: Composite {
        left_img: ImageInfo::new("iron_boots"),
        right_img: ImageInfo::new("hover_boots"),
        both_img: ImageInfo::new("composite_boots"),
        active: Box::new(|state| (state.ram.save.equipment.contains(Equipment::IRON_BOOTS), state.ram.save.equipment.contains(Equipment::HOVER_BOOTS))),
        toggle_left: Box::new(|state| state.ram.save.equipment.toggle(Equipment::IRON_BOOTS)),
        toggle_right: Box::new(|state| state.ram.save.equipment.toggle(Equipment::HOVER_BOOTS)),
    },
    IronBoots: Simple {
        img: ImageInfo::new("iron_boots"),
        active: Box::new(|state| state.ram.save.equipment.contains(Equipment::IRON_BOOTS)),
        toggle: Box::new(|state| state.ram.save.equipment.toggle(Equipment::IRON_BOOTS)),
    },
    HoverBoots: Simple {
        img: ImageInfo::new("hover_boots"),
        active: Box::new(|state| state.ram.save.equipment.contains(Equipment::HOVER_BOOTS)),
        toggle: Box::new(|state| state.ram.save.equipment.toggle(Equipment::HOVER_BOOTS)),
    },
    MirrorShield: Simple {
        img: ImageInfo::new("mirror_shield"),
        active: Box::new(|state| state.ram.save.equipment.contains(Equipment::MIRROR_SHIELD)),
        toggle: Box::new(|state| state.ram.save.equipment.toggle(Equipment::MIRROR_SHIELD)),
    },
    ChildTrade: Sequence {
        idx: Box::new(|state| match state.ram.save.inv.child_trade_item {
            ChildTradeItem::None => 0,
            ChildTradeItem::WeirdEgg => 1,
            ChildTradeItem::Chicken => 2,
            ChildTradeItem::ZeldasLetter | ChildTradeItem::GoronMask | ChildTradeItem::ZoraMask | ChildTradeItem::GerudoMask | ChildTradeItem::SoldOut => 3, //TODO for SOLD OUT, check trade quest progress
            ChildTradeItem::KeatonMask => 4,
            ChildTradeItem::SkullMask => 5,
            ChildTradeItem::SpookyMask => 6,
            ChildTradeItem::BunnyHood => 7,
            ChildTradeItem::MaskOfTruth => 8,
        }),
        img: Box::new(|state| match state.ram.save.inv.child_trade_item {
            ChildTradeItem::None => (false, ImageInfo::new("white_egg")),
            ChildTradeItem::WeirdEgg => (true, ImageInfo::new("white_egg")),
            ChildTradeItem::Chicken => (true, ImageInfo::new("white_chicken")),
            ChildTradeItem::ZeldasLetter | ChildTradeItem::GoronMask | ChildTradeItem::ZoraMask | ChildTradeItem::GerudoMask | ChildTradeItem::SoldOut => (true, ImageInfo::new("zelda_letter")), //TODO for SOLD OUT, check trade quest progress
            ChildTradeItem::KeatonMask => (true, ImageInfo::new("keaton_mask")),
            ChildTradeItem::SkullMask => (true, ImageInfo::new("skull_mask")),
            ChildTradeItem::SpookyMask => (true, ImageInfo::new("spooky_mask")),
            ChildTradeItem::BunnyHood => (true, ImageInfo::new("bunny_hood")),
            ChildTradeItem::MaskOfTruth => (true, ImageInfo::new("mask_of_truth")),
        }),
        increment: Box::new(|state| state.ram.save.inv.child_trade_item = match state.ram.save.inv.child_trade_item {
            ChildTradeItem::None => ChildTradeItem::WeirdEgg,
            ChildTradeItem::WeirdEgg => ChildTradeItem::Chicken,
            ChildTradeItem::Chicken => ChildTradeItem::ZeldasLetter,
            ChildTradeItem::ZeldasLetter | ChildTradeItem::GoronMask | ChildTradeItem::ZoraMask | ChildTradeItem::GerudoMask | ChildTradeItem::SoldOut => ChildTradeItem::KeatonMask, //TODO for SOLD OUT, check trade quest progress
            ChildTradeItem::KeatonMask => ChildTradeItem::SkullMask,
            ChildTradeItem::SkullMask => ChildTradeItem::SpookyMask,
            ChildTradeItem::SpookyMask => ChildTradeItem::BunnyHood,
            ChildTradeItem::BunnyHood => ChildTradeItem::MaskOfTruth,
            ChildTradeItem::MaskOfTruth => ChildTradeItem::None,
        }),
        decrement: Box::new(|state| state.ram.save.inv.child_trade_item = match state.ram.save.inv.child_trade_item {
            ChildTradeItem::None => ChildTradeItem::MaskOfTruth,
            ChildTradeItem::WeirdEgg => ChildTradeItem::None,
            ChildTradeItem::Chicken => ChildTradeItem::WeirdEgg,
            ChildTradeItem::ZeldasLetter | ChildTradeItem::GoronMask | ChildTradeItem::ZoraMask | ChildTradeItem::GerudoMask | ChildTradeItem::SoldOut => ChildTradeItem::Chicken, //TODO for SOLD OUT, check trade quest progress
            ChildTradeItem::KeatonMask => ChildTradeItem::ZeldasLetter,
            ChildTradeItem::SkullMask => ChildTradeItem::KeatonMask,
            ChildTradeItem::SpookyMask => ChildTradeItem::SkullMask,
            ChildTradeItem::BunnyHood => ChildTradeItem::SpookyMask,
            ChildTradeItem::MaskOfTruth => ChildTradeItem::BunnyHood,
        }),
    },
    ChildTradeNoChicken: Sequence {
        idx: Box::new(|state| match state.ram.save.inv.child_trade_item {
            ChildTradeItem::None => 0,
            ChildTradeItem::WeirdEgg | ChildTradeItem::Chicken => 1,
            ChildTradeItem::ZeldasLetter | ChildTradeItem::GoronMask | ChildTradeItem::ZoraMask | ChildTradeItem::GerudoMask | ChildTradeItem::SoldOut => 2, //TODO for SOLD OUT, check trade quest progress
            ChildTradeItem::KeatonMask => 3,
            ChildTradeItem::SkullMask => 4,
            ChildTradeItem::SpookyMask => 5,
            ChildTradeItem::BunnyHood => 6,
            ChildTradeItem::MaskOfTruth => 7,
        }),
        img: Box::new(|state| match state.ram.save.inv.child_trade_item {
            ChildTradeItem::None => (false, ImageInfo::new("white_egg")),
            ChildTradeItem::WeirdEgg | ChildTradeItem::Chicken => (true, ImageInfo::new("white_egg")),
            ChildTradeItem::ZeldasLetter | ChildTradeItem::GoronMask | ChildTradeItem::ZoraMask | ChildTradeItem::GerudoMask | ChildTradeItem::SoldOut => (true, ImageInfo::new("zelda_letter")), //TODO for SOLD OUT, check trade quest progress
            ChildTradeItem::KeatonMask => (true, ImageInfo::new("keaton_mask")),
            ChildTradeItem::SkullMask => (true, ImageInfo::new("skull_mask")),
            ChildTradeItem::SpookyMask => (true, ImageInfo::new("spooky_mask")),
            ChildTradeItem::BunnyHood => (true, ImageInfo::new("bunny_hood")),
            ChildTradeItem::MaskOfTruth => (true, ImageInfo::new("mask_of_truth")),
        }),
        increment: Box::new(|state| state.ram.save.inv.child_trade_item = match state.ram.save.inv.child_trade_item {
            ChildTradeItem::None => ChildTradeItem::WeirdEgg,
            ChildTradeItem::WeirdEgg | ChildTradeItem::Chicken => ChildTradeItem::ZeldasLetter,
            ChildTradeItem::ZeldasLetter | ChildTradeItem::GoronMask | ChildTradeItem::ZoraMask | ChildTradeItem::GerudoMask | ChildTradeItem::SoldOut => ChildTradeItem::KeatonMask, //TODO for SOLD OUT, check trade quest progress
            ChildTradeItem::KeatonMask => ChildTradeItem::SkullMask,
            ChildTradeItem::SkullMask => ChildTradeItem::SpookyMask,
            ChildTradeItem::SpookyMask => ChildTradeItem::BunnyHood,
            ChildTradeItem::BunnyHood => ChildTradeItem::MaskOfTruth,
            ChildTradeItem::MaskOfTruth => ChildTradeItem::None,
        }),
        decrement: Box::new(|state| state.ram.save.inv.child_trade_item = match state.ram.save.inv.child_trade_item {
            ChildTradeItem::None => ChildTradeItem::MaskOfTruth,
            ChildTradeItem::WeirdEgg | ChildTradeItem::Chicken => ChildTradeItem::None,
            ChildTradeItem::ZeldasLetter | ChildTradeItem::GoronMask | ChildTradeItem::ZoraMask | ChildTradeItem::GerudoMask | ChildTradeItem::SoldOut => ChildTradeItem::WeirdEgg, //TODO for SOLD OUT, check trade quest progress
            ChildTradeItem::KeatonMask => ChildTradeItem::ZeldasLetter,
            ChildTradeItem::SkullMask => ChildTradeItem::KeatonMask,
            ChildTradeItem::SpookyMask => ChildTradeItem::SkullMask,
            ChildTradeItem::BunnyHood => ChildTradeItem::SpookyMask,
            ChildTradeItem::MaskOfTruth => ChildTradeItem::BunnyHood,
        }),
    },
    ChildTradeSoldOut: Sequence {
        idx: Box::new(|state| match state.ram.save.inv.child_trade_item {
            ChildTradeItem::None => 0,
            ChildTradeItem::WeirdEgg => 1,
            ChildTradeItem::Chicken => 2,
            ChildTradeItem::ZeldasLetter | ChildTradeItem::GoronMask | ChildTradeItem::ZoraMask | ChildTradeItem::GerudoMask | ChildTradeItem::SoldOut => 3, //TODO for SOLD OUT, check trade quest progress
            //TODO Zelda's letter turned in => 4
            ChildTradeItem::KeatonMask => 5,
            //TODO Keaton mask sold => 6
            ChildTradeItem::SkullMask => 7,
            //TODO skull mask sold => 8
            ChildTradeItem::SpookyMask => 9,
            //TODO spooky mask sold => 10
            ChildTradeItem::BunnyHood => 11,
            //TODO bunny hood sold => 12
            ChildTradeItem::MaskOfTruth => 13,
        }),
        img: Box::new(|state| match state.ram.save.inv.child_trade_item {
            ChildTradeItem::None => (false, ImageInfo::new("white_egg")),
            ChildTradeItem::WeirdEgg => (true, ImageInfo::new("white_egg")),
            ChildTradeItem::Chicken => (true, ImageInfo::new("white_chicken")),
            ChildTradeItem::ZeldasLetter | ChildTradeItem::GoronMask | ChildTradeItem::ZoraMask | ChildTradeItem::GerudoMask | ChildTradeItem::SoldOut => (true, ImageInfo::new("zelda_letter")), //TODO for SOLD OUT, check trade quest progress
            //TODO Zelda's letter turned in => SOLD OUT
            ChildTradeItem::KeatonMask => (true, ImageInfo::new("keaton_mask")),
            //TODO Keaton mask sold => SOLD OUT
            ChildTradeItem::SkullMask => (true, ImageInfo::new("skull_mask")),
            //TODO skull mask sold => SOLD OUT
            ChildTradeItem::SpookyMask => (true, ImageInfo::new("spooky_mask")),
            //TODO spooky mask sold => SOLD OUT
            ChildTradeItem::BunnyHood => (true, ImageInfo::new("bunny_hood")),
            //TODO bunny hood sold => SOLD OUT
            ChildTradeItem::MaskOfTruth => (true, ImageInfo::new("mask_of_truth")),
        }),
        increment: Box::new(|state| state.ram.save.inv.child_trade_item = match state.ram.save.inv.child_trade_item {
            //TODO consider sold-out states
            ChildTradeItem::None => ChildTradeItem::WeirdEgg,
            ChildTradeItem::WeirdEgg => ChildTradeItem::Chicken,
            ChildTradeItem::Chicken => ChildTradeItem::ZeldasLetter,
            ChildTradeItem::ZeldasLetter | ChildTradeItem::GoronMask | ChildTradeItem::ZoraMask | ChildTradeItem::GerudoMask | ChildTradeItem::SoldOut => ChildTradeItem::KeatonMask, //TODO for SOLD OUT, check trade quest progress
            ChildTradeItem::KeatonMask => ChildTradeItem::SkullMask,
            ChildTradeItem::SkullMask => ChildTradeItem::SpookyMask,
            ChildTradeItem::SpookyMask => ChildTradeItem::BunnyHood,
            ChildTradeItem::BunnyHood => ChildTradeItem::MaskOfTruth,
            ChildTradeItem::MaskOfTruth => ChildTradeItem::None,
        }),
        decrement: Box::new(|state| state.ram.save.inv.child_trade_item = match state.ram.save.inv.child_trade_item {
            //TODO consider sold-out states
            ChildTradeItem::None => ChildTradeItem::MaskOfTruth,
            ChildTradeItem::WeirdEgg => ChildTradeItem::None,
            ChildTradeItem::Chicken => ChildTradeItem::WeirdEgg,
            ChildTradeItem::ZeldasLetter | ChildTradeItem::GoronMask | ChildTradeItem::ZoraMask | ChildTradeItem::GerudoMask | ChildTradeItem::SoldOut => ChildTradeItem::Chicken, //TODO for SOLD OUT, check trade quest progress
            ChildTradeItem::KeatonMask => ChildTradeItem::ZeldasLetter,
            ChildTradeItem::SkullMask => ChildTradeItem::KeatonMask,
            ChildTradeItem::SpookyMask => ChildTradeItem::SkullMask,
            ChildTradeItem::BunnyHood => ChildTradeItem::SpookyMask,
            ChildTradeItem::MaskOfTruth => ChildTradeItem::BunnyHood,
        }),
    },
    Ocarina: Overlay {
        main_img: ImageInfo::new("ocarina"),
        overlay_img: ImageInfo::new("scarecrow"),
        //TODO this has multiple issues:
        // * it leaks the info that the free scarecrow setting is active as soon as the scarecrow song has been set as child
        // * it doesn't display free scarecrow song known from settings input
        // see also https://github.com/fenhl/oottracker/issues/21
        active: Box::new(|state| (state.ram.save.inv.ocarina != Ocarina::None, state.ram.save.scarecrow_song_child && state.ram.save.event_chk_inf.9.contains(EventChkInf9::SCARECROW_SONG))),
        toggle_main: Box::new(|state| state.ram.save.inv.ocarina = match state.ram.save.inv.ocarina {
            Ocarina::None => Ocarina::FairyOcarina,
            Ocarina::FairyOcarina | Ocarina::OcarinaOfTime => Ocarina::None,
        }),
        toggle_overlay: Box::new(|state| if state.ram.save.scarecrow_song_child && state.ram.save.event_chk_inf.9.contains(EventChkInf9::SCARECROW_SONG) {
            state.ram.save.event_chk_inf.9.remove(EventChkInf9::SCARECROW_SONG);
        } else {
            state.ram.save.scarecrow_song_child = true;
            state.ram.save.event_chk_inf.9.insert(EventChkInf9::SCARECROW_SONG);
        }), //TODO make sure free scarecrow knowledge is toggled properly
    },
    Beans: Simple { //TODO overlay with number bought if auto-tracking is on & shuffle beans is off
        img: ImageInfo::new("beans"),
        active: Box::new(|state| state.ram.save.inv.beans),
        toggle: Box::new(|state| state.ram.save.inv.beans = !state.ram.save.inv.beans),
    },
    SwordCard: Composite {
        left_img: ImageInfo::new("kokiri_sword"),
        right_img: ImageInfo::new("gerudo_card"),
        both_img: ImageInfo::extra("composite_ksword_gcard"),
        active: Box::new(|state| (state.ram.save.equipment.contains(Equipment::KOKIRI_SWORD), state.ram.save.quest_items.contains(QuestItems::GERUDO_CARD))),
        toggle_left: Box::new(|state| state.ram.save.equipment.toggle(Equipment::KOKIRI_SWORD)),
        toggle_right: Box::new(|state| state.ram.save.quest_items.toggle(QuestItems::GERUDO_CARD)),
    },
    SwordShield: Overlay {
        main_img: ImageInfo::new("kokiri_sword"),
        overlay_img: ImageInfo::extra("deku_shield_badge"),
        active: Box::new(|state| (state.ram.save.equipment.contains(Equipment::KOKIRI_SWORD), state.ram.save.equipment.contains(Equipment::DEKU_SHIELD))),
        toggle_main: Box::new(|state| state.ram.save.equipment.toggle(Equipment::KOKIRI_SWORD)),
        toggle_overlay: Box::new(|state| state.ram.save.equipment.toggle(Equipment::DEKU_SHIELD)),
    },
    KokiriSword: Simple {
        img: ImageInfo::new("kokiri_sword"),
        active: Box::new(|state| state.ram.save.equipment.contains(Equipment::KOKIRI_SWORD)),
        toggle: Box::new(|state| state.ram.save.equipment.toggle(Equipment::KOKIRI_SWORD)),
    },
    Tunics: Composite {
        left_img: ImageInfo::new("goron_tunic"),
        right_img: ImageInfo::new("zora_tunic"),
        both_img: ImageInfo::new("composite_tunics"),
        active: Box::new(|state| (state.ram.save.equipment.contains(Equipment::GORON_TUNIC), state.ram.save.equipment.contains(Equipment::ZORA_TUNIC))),
        toggle_left: Box::new(|state| state.ram.save.equipment.toggle(Equipment::GORON_TUNIC)),
        toggle_right: Box::new(|state| state.ram.save.equipment.toggle(Equipment::ZORA_TUNIC)),
    },
    GoronTunic: Simple {
        img: ImageInfo::new("goron_tunic"),
        active: Box::new(|state| state.ram.save.equipment.contains(Equipment::GORON_TUNIC)),
        toggle: Box::new(|state| state.ram.save.equipment.toggle(Equipment::GORON_TUNIC)),
    },
    ZoraTunic: Simple {
        img: ImageInfo::new("zora_tunic"),
        active: Box::new(|state| state.ram.save.equipment.contains(Equipment::ZORA_TUNIC)),
        toggle: Box::new(|state| state.ram.save.equipment.toggle(Equipment::ZORA_TUNIC)),
    },
    Triforce: Count {
        dimmed_img: ImageInfo::new("triforce"),
        img: ImageInfo::new("force"),
        get: Box::new(|state| state.ram.save.triforce_pieces()),
        set: Box::new(|state, value| state.ram.save.set_triforce_pieces(value)),
        max: 100,
        step: 1,
    },
    BigPoeTriforce: BigPoeTriforce,
    TriforceOneAndFives: Sequence {
        idx: Box::new(|state| match state.ram.save.triforce_pieces() {
            0 => 0,
            1..=4 => 1,
            5..=9 => 2,
            10..=14 => 3,
            15..=19 => 4,
            20..=24 => 5,
            25..=29 => 6,
            30..=34 => 7,
            35..=39 => 8,
            40..=44 => 9,
            45..=49 => 10,
            50..=54 => 11,
            55..=59 => 12,
            _ => 13,
        }),
        img: Box::new(|state| (state.ram.save.triforce_pieces() > 0, ImageInfo::new("triforce"))), //TODO images from count?
        increment: Box::new(|state| {
            let new_val = match state.ram.save.triforce_pieces() {
                0 => 1,
                1..=4 => 5,
                5..=9 => 10,
                10..=14 => 15,
                15..=19 => 20,
                20..=24 => 25,
                25..=29 => 30,
                30..=34 => 35,
                35..=39 => 40,
                40..=44 => 45,
                45..=49 => 50,
                50..=54 => 55,
                55..=59 => 60,
                _ => 0,
            };
            state.ram.save.set_triforce_pieces(new_val);
        }),
        decrement: Box::new(|state| {
            let new_val = match state.ram.save.triforce_pieces() {
                0 => 60,
                1..=4 => 0,
                5..=9 => 1,
                10..=14 => 5,
                15..=19 => 10,
                20..=24 => 15,
                25..=29 => 20,
                30..=34 => 25,
                35..=39 => 30,
                40..=44 => 35,
                45..=49 => 40,
                50..=54 => 45,
                55..=59 => 50,
                _ => 55,
            };
            state.ram.save.set_triforce_pieces(new_val);
        }),
    },
    ZeldasLullaby: Song {
        song: QuestItems::ZELDAS_LULLABY,
        check: "Song from Impa",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SONG_FROM_IMPA)),
    },
    ZeldasLullabyCheck: SongCheck {
        check: "Song from Impa",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SONG_FROM_IMPA)),
    },
    EponasSong: Song {
        song: QuestItems::EPONAS_SONG,
        check: "Song from Malon",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SONG_FROM_MALON)),
    },
    EponasSongCheck: SongCheck {
        check: "Song from Malon",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SONG_FROM_MALON)),
    },
    SariasSong: Song {
        song: QuestItems::SARIAS_SONG,
        check: "Song from Saria",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SONG_FROM_SARIA)),
    },
    SariasSongCheck: SongCheck {
        check: "Song from Saria",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SONG_FROM_SARIA)),
    },
    SunsSong: Song {
        song: QuestItems::SUNS_SONG,
        check: "Song from Royal Familys Tomb",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SONG_FROM_ROYAL_FAMILYS_TOMB)),
    },
    SunsSongCheck: SongCheck {
        check: "Song from Royal Familys Tomb",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SONG_FROM_ROYAL_FAMILYS_TOMB)),
    },
    SongOfTime: Song {
        song: QuestItems::SONG_OF_TIME,
        check: "Song from Ocarina of Time",
        toggle_overlay: Box::new(|eci| eci.10.toggle(EventChkInf10::SONG_FROM_OCARINA_OF_TIME)),
    },
    SongOfTimeCheck: SongCheck {
        check: "Song from Ocarina of Time",
        toggle_overlay: Box::new(|eci| eci.10.toggle(EventChkInf10::SONG_FROM_OCARINA_OF_TIME)),
    },
    SongOfStorms: Song {
        song: QuestItems::SONG_OF_STORMS,
        check: "Song from Windmill",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SONG_FROM_WINDMILL)),
    },
    SongOfStormsCheck: SongCheck {
        check: "Song from Windmill",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SONG_FROM_WINDMILL)),
    },
    Minuet: Song {
        song: QuestItems::MINUET_OF_FOREST,
        check: "Sheik in Forest",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SHEIK_IN_FOREST)),
    },
    MinuetCheck: SongCheck {
        check: "Sheik in Forest",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SHEIK_IN_FOREST)),
    },
    Bolero: Song {
        song: QuestItems::BOLERO_OF_FIRE,
        check: "Sheik in Crater",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SHEIK_IN_CRATER)),
    },
    BoleroCheck: SongCheck {
        check: "Sheik in Crater",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SHEIK_IN_CRATER)),
    },
    Serenade: Song {
        song: QuestItems::SERENADE_OF_WATER,
        check: "Sheik in Ice Cavern",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SHEIK_IN_ICE_CAVERN)),
    },
    SerenadeCheck: SongCheck {
        check: "Sheik in Ice Cavern",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SHEIK_IN_ICE_CAVERN)),
    },
    Requiem: Song {
        song: QuestItems::REQUIEM_OF_SPIRIT,
        check: "Sheik at Colossus",
        toggle_overlay: Box::new(|eci| eci.10.toggle(EventChkInf10::SHEIK_AT_COLOSSUS)),
    },
    RequiemCheck: SongCheck {
        check: "Sheik at Colossus",
        toggle_overlay: Box::new(|eci| eci.10.toggle(EventChkInf10::SHEIK_AT_COLOSSUS)),
    },
    Nocturne: Song {
        song: QuestItems::NOCTURNE_OF_SHADOW,
        check: "Sheik in Kakariko",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SHEIK_IN_KAKARIKO)),
    },
    NocturneCheck: SongCheck {
        check: "Sheik in Kakariko",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SHEIK_IN_KAKARIKO)),
    },
    Prelude: Song {
        song: QuestItems::PRELUDE_OF_LIGHT,
        check: "Sheik at Temple",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SHEIK_AT_TEMPLE)),
    },
    PreludeCheck: SongCheck {
        check: "Sheik at Temple",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SHEIK_AT_TEMPLE)),
    },
    FreeReward: FreeReward,
    DekuMq: Mq(Dungeon::Main(MainDungeon::DekuTree)),
    DcMq: Mq(Dungeon::Main(MainDungeon::DodongosCavern)),
    JabuMq: Mq(Dungeon::Main(MainDungeon::JabuJabu)),
    ForestMq: Mq(Dungeon::Main(MainDungeon::ForestTemple)),
    ForestSmallKeys: TrackerCellKind::SmallKeys {
        get: Box::new(|keys| keys.forest_temple),
        set: Box::new(|keys, value| keys.forest_temple = value),
        max_vanilla: 5,
        max_mq: 6,
        label: "Frst",
    },
    ForestBossKey: BossKey {
        active: Box::new(|keys| keys.forest_temple.contains(DungeonItems::BOSS_KEY)),
        toggle: Box::new(|keys| keys.forest_temple.toggle(DungeonItems::BOSS_KEY)),
        label: "Frst",
    },
    ForestKeys: CompositeKeys {
        small: TrackerCellId::ForestSmallKeys,
        boss: TrackerCellId::ForestBossKey,
    },
    FireMq: Mq(Dungeon::Main(MainDungeon::FireTemple)),
    FireSmallKeys: TrackerCellKind::SmallKeys {
        get: Box::new(|keys| keys.fire_temple),
        set: Box::new(|keys, value| keys.fire_temple = value),
        max_vanilla: 8,
        max_mq: 5,
        label: "Fire",
    },
    FireBossKey: BossKey {
        active: Box::new(|keys| keys.fire_temple.contains(DungeonItems::BOSS_KEY)),
        toggle: Box::new(|keys| keys.fire_temple.toggle(DungeonItems::BOSS_KEY)),
        label: "Fire",
    },
    FireKeys: CompositeKeys {
        small: TrackerCellId::FireSmallKeys,
        boss: TrackerCellId::FireBossKey,
    },
    WaterMq: Mq(Dungeon::Main(MainDungeon::WaterTemple)),
    WaterSmallKeys: TrackerCellKind::SmallKeys {
        get: Box::new(|keys| keys.water_temple),
        set: Box::new(|keys, value| keys.water_temple = value),
        max_vanilla: 6,
        max_mq: 2,
        label: "Watr",
    },
    WaterBossKey: BossKey {
        active: Box::new(|keys| keys.water_temple.contains(DungeonItems::BOSS_KEY)),
        toggle: Box::new(|keys| keys.water_temple.toggle(DungeonItems::BOSS_KEY)),
        label: "Watr",
    },
    WaterKeys: CompositeKeys {
        small: TrackerCellId::WaterSmallKeys,
        boss: TrackerCellId::WaterBossKey,
    },
    ShadowMq: Mq(Dungeon::Main(MainDungeon::ShadowTemple)),
    ShadowSmallKeys: TrackerCellKind::SmallKeys {
        get: Box::new(|keys| keys.shadow_temple),
        set: Box::new(|keys, value| keys.shadow_temple = value),
        max_vanilla: 5,
        max_mq: 6,
        label: "Shdw",
    },
    ShadowBossKey: BossKey {
        active: Box::new(|keys| keys.shadow_temple.contains(DungeonItems::BOSS_KEY)),
        toggle: Box::new(|keys| keys.shadow_temple.toggle(DungeonItems::BOSS_KEY)),
        label: "Shdw",
    },
    ShadowKeys: CompositeKeys {
        small: TrackerCellId::ShadowSmallKeys,
        boss: TrackerCellId::ShadowBossKey,
    },
    SpiritMq: Mq(Dungeon::Main(MainDungeon::SpiritTemple)),
    SpiritSmallKeys: TrackerCellKind::SmallKeys {
        get: Box::new(|keys| keys.spirit_temple),
        set: Box::new(|keys, value| keys.spirit_temple = value),
        max_vanilla: 5,
        max_mq: 7,
        label: "Sprt",
    },
    SpiritBossKey: BossKey {
        active: Box::new(|keys| keys.spirit_temple.contains(DungeonItems::BOSS_KEY)),
        toggle: Box::new(|keys| keys.spirit_temple.toggle(DungeonItems::BOSS_KEY)),
        label: "Sprt",
    },
    SpiritKeys: CompositeKeys {
        small: TrackerCellId::SpiritSmallKeys,
        boss: TrackerCellId::SpiritBossKey,
    },
    IceMq: Mq(Dungeon::IceCavern),
    WellMq: Mq(Dungeon::BottomOfTheWell),
    WellSmallKeys: TrackerCellKind::SmallKeys {
        get: Box::new(|keys| keys.bottom_of_the_well),
        set: Box::new(|keys, value| keys.bottom_of_the_well = value),
        max_vanilla: 3,
        max_mq: 2,
        label: "Well",
    },
    FortressMq: FortressMq,
    FortressSmallKeys: TrackerCellKind::SmallKeys {
        get: Box::new(|keys| keys.thieves_hideout),
        set: Box::new(|keys, value| keys.thieves_hideout = value),
        max_vanilla: 4,
        max_mq: 4,
        label: "Fort",
    },
    GtgMq: Mq(Dungeon::GerudoTrainingGround),
    GtgSmallKeys: TrackerCellKind::SmallKeys {
        get: Box::new(|keys| keys.gerudo_training_ground),
        set: Box::new(|keys, value| keys.gerudo_training_ground = value),
        max_vanilla: 9,
        max_mq: 3,
        label: "GTG",
    },
    GanonMq: Mq(Dungeon::GanonsCastle),
    GanonSmallKeys: TrackerCellKind::SmallKeys {
        get: Box::new(|keys| keys.ganons_castle),
        set: Box::new(|keys, value| keys.ganons_castle = value),
        max_vanilla: 2,
        max_mq: 3,
        label: "Ganon",
    },
    GanonBossKey: BossKey {
        active: Box::new(|keys| keys.ganons_castle.contains(DungeonItems::BOSS_KEY)),
        toggle: Box::new(|keys| keys.ganons_castle.toggle(DungeonItems::BOSS_KEY)),
        label: "Ganon",
    },
    GanonKeys: CompositeKeys {
        small: TrackerCellId::GanonSmallKeys,
        boss: TrackerCellId::GanonBossKey,
    },

    // ============================================================================
    // OoT Dungeon Maps
    // ============================================================================
    DekuMap: OotMap {
        active: Box::new(|keys| keys.deku_tree.contains(DungeonItems::MAP)),
        toggle: Box::new(|keys| keys.deku_tree.toggle(DungeonItems::MAP)),
        label: "Deku",
    },
    DcMap: OotMap {
        active: Box::new(|keys| keys.dodongos_cavern.contains(DungeonItems::MAP)),
        toggle: Box::new(|keys| keys.dodongos_cavern.toggle(DungeonItems::MAP)),
        label: "DC",
    },
    JabuMap: OotMap {
        active: Box::new(|keys| keys.jabu_jabu.contains(DungeonItems::MAP)),
        toggle: Box::new(|keys| keys.jabu_jabu.toggle(DungeonItems::MAP)),
        label: "Jabu",
    },
    ForestMap: OotMap {
        active: Box::new(|keys| keys.forest_temple.contains(DungeonItems::MAP)),
        toggle: Box::new(|keys| keys.forest_temple.toggle(DungeonItems::MAP)),
        label: "Frst",
    },
    FireMap: OotMap {
        active: Box::new(|keys| keys.fire_temple.contains(DungeonItems::MAP)),
        toggle: Box::new(|keys| keys.fire_temple.toggle(DungeonItems::MAP)),
        label: "Fire",
    },
    WaterMap: OotMap {
        active: Box::new(|keys| keys.water_temple.contains(DungeonItems::MAP)),
        toggle: Box::new(|keys| keys.water_temple.toggle(DungeonItems::MAP)),
        label: "Watr",
    },
    ShadowMap: OotMap {
        active: Box::new(|keys| keys.shadow_temple.contains(DungeonItems::MAP)),
        toggle: Box::new(|keys| keys.shadow_temple.toggle(DungeonItems::MAP)),
        label: "Shdw",
    },
    SpiritMap: OotMap {
        active: Box::new(|keys| keys.spirit_temple.contains(DungeonItems::MAP)),
        toggle: Box::new(|keys| keys.spirit_temple.toggle(DungeonItems::MAP)),
        label: "Sprt",
    },
    WellMap: OotMap {
        active: Box::new(|keys| keys.bottom_of_the_well.contains(DungeonItems::MAP)),
        toggle: Box::new(|keys| keys.bottom_of_the_well.toggle(DungeonItems::MAP)),
        label: "Well",
    },
    IceMap: OotMap {
        active: Box::new(|keys| keys.ice_cavern.contains(DungeonItems::MAP)),
        toggle: Box::new(|keys| keys.ice_cavern.toggle(DungeonItems::MAP)),
        label: "Ice",
    },
    GanonMap: OotMap {
        active: Box::new(|keys| keys.ganons_castle.contains(DungeonItems::MAP)),
        toggle: Box::new(|keys| keys.ganons_castle.toggle(DungeonItems::MAP)),
        label: "Ganon",
    },

    // ============================================================================
    // OoT Dungeon Compasses
    // ============================================================================
    DekuCompass: OotCompass {
        active: Box::new(|keys| keys.deku_tree.contains(DungeonItems::COMPASS)),
        toggle: Box::new(|keys| keys.deku_tree.toggle(DungeonItems::COMPASS)),
        label: "Deku",
    },
    DcCompass: OotCompass {
        active: Box::new(|keys| keys.dodongos_cavern.contains(DungeonItems::COMPASS)),
        toggle: Box::new(|keys| keys.dodongos_cavern.toggle(DungeonItems::COMPASS)),
        label: "DC",
    },
    JabuCompass: OotCompass {
        active: Box::new(|keys| keys.jabu_jabu.contains(DungeonItems::COMPASS)),
        toggle: Box::new(|keys| keys.jabu_jabu.toggle(DungeonItems::COMPASS)),
        label: "Jabu",
    },
    ForestCompass: OotCompass {
        active: Box::new(|keys| keys.forest_temple.contains(DungeonItems::COMPASS)),
        toggle: Box::new(|keys| keys.forest_temple.toggle(DungeonItems::COMPASS)),
        label: "Frst",
    },
    FireCompass: OotCompass {
        active: Box::new(|keys| keys.fire_temple.contains(DungeonItems::COMPASS)),
        toggle: Box::new(|keys| keys.fire_temple.toggle(DungeonItems::COMPASS)),
        label: "Fire",
    },
    WaterCompass: OotCompass {
        active: Box::new(|keys| keys.water_temple.contains(DungeonItems::COMPASS)),
        toggle: Box::new(|keys| keys.water_temple.toggle(DungeonItems::COMPASS)),
        label: "Watr",
    },
    ShadowCompass: OotCompass {
        active: Box::new(|keys| keys.shadow_temple.contains(DungeonItems::COMPASS)),
        toggle: Box::new(|keys| keys.shadow_temple.toggle(DungeonItems::COMPASS)),
        label: "Shdw",
    },
    SpiritCompass: OotCompass {
        active: Box::new(|keys| keys.spirit_temple.contains(DungeonItems::COMPASS)),
        toggle: Box::new(|keys| keys.spirit_temple.toggle(DungeonItems::COMPASS)),
        label: "Sprt",
    },
    WellCompass: OotCompass {
        active: Box::new(|keys| keys.bottom_of_the_well.contains(DungeonItems::COMPASS)),
        toggle: Box::new(|keys| keys.bottom_of_the_well.toggle(DungeonItems::COMPASS)),
        label: "Well",
    },
    IceCompass: OotCompass {
        active: Box::new(|keys| keys.ice_cavern.contains(DungeonItems::COMPASS)),
        toggle: Box::new(|keys| keys.ice_cavern.toggle(DungeonItems::COMPASS)),
        label: "Ice",
    },

    // ============================================================================
    // MM Dungeon Boss Keys
    // ============================================================================
    MmWoodfallBossKey: MmBossKey {
        active: Box::new(|keys| keys.woodfall.contains(crate::mm_save::MmDungeonItems::BOSS_KEY)),
        toggle: Box::new(|keys| keys.woodfall.toggle(crate::mm_save::MmDungeonItems::BOSS_KEY)),
        label: "WF",
    },
    MmSnowheadBossKey: MmBossKey {
        active: Box::new(|keys| keys.snowhead.contains(crate::mm_save::MmDungeonItems::BOSS_KEY)),
        toggle: Box::new(|keys| keys.snowhead.toggle(crate::mm_save::MmDungeonItems::BOSS_KEY)),
        label: "SH",
    },
    MmGreatBayBossKey: MmBossKey {
        active: Box::new(|keys| keys.great_bay.contains(crate::mm_save::MmDungeonItems::BOSS_KEY)),
        toggle: Box::new(|keys| keys.great_bay.toggle(crate::mm_save::MmDungeonItems::BOSS_KEY)),
        label: "GB",
    },
    MmStoneTowerBossKey: MmBossKey {
        active: Box::new(|keys| keys.stone_tower.contains(crate::mm_save::MmDungeonItems::BOSS_KEY)),
        toggle: Box::new(|keys| keys.stone_tower.toggle(crate::mm_save::MmDungeonItems::BOSS_KEY)),
        label: "ST",
    },

    // ============================================================================
    // MM Dungeon Maps
    // ============================================================================
    MmWoodfallMap: MmMap {
        active: Box::new(|keys| keys.woodfall.contains(crate::mm_save::MmDungeonItems::MAP)),
        toggle: Box::new(|keys| keys.woodfall.toggle(crate::mm_save::MmDungeonItems::MAP)),
        label: "WF",
    },
    MmSnowheadMap: MmMap {
        active: Box::new(|keys| keys.snowhead.contains(crate::mm_save::MmDungeonItems::MAP)),
        toggle: Box::new(|keys| keys.snowhead.toggle(crate::mm_save::MmDungeonItems::MAP)),
        label: "SH",
    },
    MmGreatBayMap: MmMap {
        active: Box::new(|keys| keys.great_bay.contains(crate::mm_save::MmDungeonItems::MAP)),
        toggle: Box::new(|keys| keys.great_bay.toggle(crate::mm_save::MmDungeonItems::MAP)),
        label: "GB",
    },
    MmStoneTowerMap: MmMap {
        active: Box::new(|keys| keys.stone_tower.contains(crate::mm_save::MmDungeonItems::MAP)),
        toggle: Box::new(|keys| keys.stone_tower.toggle(crate::mm_save::MmDungeonItems::MAP)),
        label: "ST",
    },

    // ============================================================================
    // MM Dungeon Compasses
    // ============================================================================
    MmWoodfallCompass: MmCompass {
        active: Box::new(|keys| keys.woodfall.contains(crate::mm_save::MmDungeonItems::COMPASS)),
        toggle: Box::new(|keys| keys.woodfall.toggle(crate::mm_save::MmDungeonItems::COMPASS)),
        label: "WF",
    },
    MmSnowheadCompass: MmCompass {
        active: Box::new(|keys| keys.snowhead.contains(crate::mm_save::MmDungeonItems::COMPASS)),
        toggle: Box::new(|keys| keys.snowhead.toggle(crate::mm_save::MmDungeonItems::COMPASS)),
        label: "SH",
    },
    MmGreatBayCompass: MmCompass {
        active: Box::new(|keys| keys.great_bay.contains(crate::mm_save::MmDungeonItems::COMPASS)),
        toggle: Box::new(|keys| keys.great_bay.toggle(crate::mm_save::MmDungeonItems::COMPASS)),
        label: "GB",
    },
    MmStoneTowerCompass: MmCompass {
        active: Box::new(|keys| keys.stone_tower.contains(crate::mm_save::MmDungeonItems::COMPASS)),
        toggle: Box::new(|keys| keys.stone_tower.toggle(crate::mm_save::MmDungeonItems::COMPASS)),
        label: "ST",
    },

    BiggoronSword: Simple {
        img: ImageInfo::new("UNIMPLEMENTED"),
        active: Box::new(|state| state.ram.save.biggoron_sword && state.ram.save.equipment.contains(Equipment::GIANTS_KNIFE)),
        toggle: Box::new(|state| if state.ram.save.biggoron_sword && state.ram.save.equipment.contains(Equipment::GIANTS_KNIFE) {
            state.ram.save.biggoron_sword = false;
            state.ram.save.equipment.remove(Equipment::GIANTS_KNIFE);
        } else {
            state.ram.save.biggoron_sword = true;
            state.ram.save.equipment.insert(Equipment::GIANTS_KNIFE);
        }),
    },
    WalletNoTycoon: Sequence {
        idx: Box::new(|state| match state.ram.save.upgrades.wallet() {
            Upgrades::ADULTS_WALLET => 1,
            Upgrades::GIANTS_WALLET | Upgrades::TYCOONS_WALLET => 2,
            _ => 0,
        }),
        img: Box::new(|state| (state.ram.save.upgrades.wallet() != Upgrades::NONE, ImageInfo::new("UNIMPLEMENTED"))),
        increment: Box::new(|state| {
            let new_val = match state.ram.save.upgrades.wallet() {
                Upgrades::ADULTS_WALLET => Upgrades::GIANTS_WALLET,
                Upgrades::GIANTS_WALLET | Upgrades::TYCOONS_WALLET => Upgrades::NONE,
                _ => Upgrades::ADULTS_WALLET,
            };
            state.ram.save.upgrades.set_wallet(new_val);
        }),
        decrement: Box::new(|state| {
            let new_val = match state.ram.save.upgrades.wallet() {
                Upgrades::ADULTS_WALLET => Upgrades::NONE,
                Upgrades::GIANTS_WALLET | Upgrades::TYCOONS_WALLET => Upgrades::ADULTS_WALLET,
                _ => Upgrades::GIANTS_WALLET,
            };
            state.ram.save.upgrades.set_wallet(new_val);
        }),
    },
    StoneOfAgony: Simple {
        img: ImageInfo::new("UNIMPLEMENTED"),
        active: Box::new(|state| state.ram.save.quest_items.contains(QuestItems::STONE_OF_AGONY)),
        toggle: Box::new(|state| state.ram.save.quest_items.toggle(QuestItems::STONE_OF_AGONY)),
    },
    Blank: Simple {
        img: ImageInfo::extra("blank"),
        active: Box::new(|_| false),
        toggle: Box::new(|_| ()),
    },

    // ============================================================================
    // MM Items - Transformation Masks
    // ============================================================================
    MmDekuMask: Simple {
        img: ImageInfo::mm("deku_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|mm| mm.has_deku_mask())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.masks.transformation.toggle(crate::mm_save::MmTransformationMasks::DEKU);
        }),
    },
    MmGoronMask: Simple {
        img: ImageInfo::mm("goron_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|mm| mm.has_goron_mask())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.masks.transformation.toggle(crate::mm_save::MmTransformationMasks::GORON);
        }),
    },
    MmZoraMask: Simple {
        img: ImageInfo::mm("zora_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|mm| mm.has_zora_mask())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.masks.transformation.toggle(crate::mm_save::MmTransformationMasks::ZORA);
        }),
    },
    MmFierceDeityMask: Simple {
        img: ImageInfo::mm("fierce_deity_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|mm| mm.has_fierce_deity_mask())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.masks.transformation.toggle(crate::mm_save::MmTransformationMasks::FIERCE_DEITY);
        }),
    },

    // ============================================================================
    // MM Items - Collectible Masks (24 unique)
    // ============================================================================
    MmPostmanHat: Simple {
        img: ImageInfo::mm("postman_hat"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_postman_hat())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.masks.masks_low.toggle(crate::mm_save::MmMasksLow::POSTMAN);
        }),
    },
    MmAllNightMask: Simple {
        img: ImageInfo::mm("all_night_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_all_night_mask())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.masks.masks_low.toggle(crate::mm_save::MmMasksLow::ALL_NIGHT);
        }),
    },
    MmBlastMask: Simple {
        img: ImageInfo::mm("blast_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_blast_mask())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.masks.masks_low.toggle(crate::mm_save::MmMasksLow::BLAST);
        }),
    },
    MmStoneMask: Simple {
        img: ImageInfo::mm("stone_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_stone_mask())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.masks.masks_low.toggle(crate::mm_save::MmMasksLow::STONE);
        }),
    },
    MmGreatFairyMask: Simple {
        img: ImageInfo::mm("great_fairy_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_great_fairy_mask())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.masks.masks_low.toggle(crate::mm_save::MmMasksLow::GREAT_FAIRY);
        }),
    },
    MmKeatonMask: Simple {
        img: ImageInfo::mm("keaton_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_keaton_mask())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.masks.masks_low.toggle(crate::mm_save::MmMasksLow::KEATON);
        }),
    },
    MmBremenMask: Simple {
        img: ImageInfo::mm("bremen_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_bremen_mask())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.masks.masks_low.toggle(crate::mm_save::MmMasksLow::BREMEN);
        }),
    },
    MmBunnyHood: Simple {
        img: ImageInfo::mm("bunny_hood"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_bunny_hood())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.masks.masks_low.toggle(crate::mm_save::MmMasksLow::BUNNY);
        }),
    },
    MmDonGeroMask: Simple {
        img: ImageInfo::mm("don_gero_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_don_gero_mask())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.masks.masks_low.toggle(crate::mm_save::MmMasksLow::DON_GERO);
        }),
    },
    MmMaskOfScents: Simple {
        img: ImageInfo::mm("mask_of_scents"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_mask_of_scents())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.masks.masks_low.toggle(crate::mm_save::MmMasksLow::SCENTS);
        }),
    },
    MmRomaniMask: Simple {
        img: ImageInfo::mm("romani_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_romani_mask())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.masks.masks_low.toggle(crate::mm_save::MmMasksLow::ROMANI);
        }),
    },
    MmCircusLeaderMask: Simple {
        img: ImageInfo::mm("circus_leader_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_circus_leader_mask())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.masks.masks_low.toggle(crate::mm_save::MmMasksLow::CIRCUS_LEADER);
        }),
    },
    MmKafeiMask: Simple {
        img: ImageInfo::mm("kafei_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_kafei_mask())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.masks.masks_low.toggle(crate::mm_save::MmMasksLow::KAFEI);
        }),
    },
    MmCouplesMask: Simple {
        img: ImageInfo::mm("couples_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_couples_mask())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.masks.masks_low.toggle(crate::mm_save::MmMasksLow::COUPLES);
        }),
    },
    MmMaskOfTruth: Simple {
        img: ImageInfo::mm("mask_of_truth"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_mask_of_truth())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.masks.masks_low.toggle(crate::mm_save::MmMasksLow::TRUTH);
        }),
    },
    MmKamaroMask: Simple {
        img: ImageInfo::mm("kamaro_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_kamaro_mask())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.masks.masks_low.toggle(crate::mm_save::MmMasksLow::KAMARO);
        }),
    },
    MmGibdoMask: Simple {
        img: ImageInfo::mm("gibdo_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_gibdo_mask())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.masks.masks_high.toggle(crate::mm_save::MmMasksHigh::GIBDO);
        }),
    },
    MmGaroMask: Simple {
        img: ImageInfo::mm("garo_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_garo_mask())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.masks.masks_high.toggle(crate::mm_save::MmMasksHigh::GARO);
        }),
    },
    MmCaptainHat: Simple {
        img: ImageInfo::mm("captain_hat"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_captain_hat())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.masks.masks_high.toggle(crate::mm_save::MmMasksHigh::CAPTAIN);
        }),
    },
    MmGiantMask: Simple {
        img: ImageInfo::mm("giant_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_giant_mask())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.masks.masks_high.toggle(crate::mm_save::MmMasksHigh::GIANT);
        }),
    },

    // ============================================================================
    // MM Items - Boss Remains
    // ============================================================================
    MmOdolwaRemains: Simple {
        img: ImageInfo::mm("odolwa_remains"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_odolwa_remains())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.quest_items.toggle(crate::mm_save::MmQuestItems::REMAINS_ODOLWA);
        }),
    },
    MmGohtRemains: Simple {
        img: ImageInfo::mm("goht_remains"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_goht_remains())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.quest_items.toggle(crate::mm_save::MmQuestItems::REMAINS_GOHT);
        }),
    },
    MmGyorgRemains: Simple {
        img: ImageInfo::mm("gyorg_remains"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_gyorg_remains())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.quest_items.toggle(crate::mm_save::MmQuestItems::REMAINS_GYORG);
        }),
    },
    MmTwinmoldRemains: Simple {
        img: ImageInfo::mm("twinmold_remains"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_twinmold_remains())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.quest_items.toggle(crate::mm_save::MmQuestItems::REMAINS_TWINMOLD);
        }),
    },

    // ============================================================================
    // MM Items - Stray Fairies (per dungeon)
    // ============================================================================
    MmStrayFairyWoodfall: Count {
        dimmed_img: ImageInfo::mm("stray_fairy_woodfall"),
        img: ImageInfo::mm("stray_fairy_woodfall"),
        get: Box::new(|state| state.ram.mm_save.as_ref().map_or(0, |mm| mm.stray_fairies.woodfall)),
        set: Box::new(|state, value| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.stray_fairies.woodfall = value;
        }),
        max: 15,
        step: 1,
    },
    MmStrayFairySnowhead: Count {
        dimmed_img: ImageInfo::mm("stray_fairy_snowhead"),
        img: ImageInfo::mm("stray_fairy_snowhead"),
        get: Box::new(|state| state.ram.mm_save.as_ref().map_or(0, |mm| mm.stray_fairies.snowhead)),
        set: Box::new(|state, value| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.stray_fairies.snowhead = value;
        }),
        max: 15,
        step: 1,
    },
    MmStrayFairyGreatBay: Count {
        dimmed_img: ImageInfo::mm("stray_fairy_great_bay"),
        img: ImageInfo::mm("stray_fairy_great_bay"),
        get: Box::new(|state| state.ram.mm_save.as_ref().map_or(0, |mm| mm.stray_fairies.great_bay)),
        set: Box::new(|state, value| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.stray_fairies.great_bay = value;
        }),
        max: 15,
        step: 1,
    },
    MmStrayFairyStoneTower: Count {
        dimmed_img: ImageInfo::mm("stray_fairy_stone_tower"),
        img: ImageInfo::mm("stray_fairy_stone_tower"),
        get: Box::new(|state| state.ram.mm_save.as_ref().map_or(0, |mm| mm.stray_fairies.stone_tower)),
        set: Box::new(|state, value| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.stray_fairies.stone_tower = value;
        }),
        max: 15,
        step: 1,
    },
    MmStrayFairyClockTown: Simple {
        img: ImageInfo::mm("stray_fairy_clock_town"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|mm| mm.stray_fairies.clock_town > 0)),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.stray_fairies.clock_town = if mm_save.stray_fairies.clock_town > 0 { 0 } else { 1 };
        }),
    },

    // ============================================================================
    // MM Items - Songs
    // ============================================================================
    MmSongOfTime: Simple {
        img: ImageInfo::mm("song_of_time"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|mm| mm.has_song_of_time())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.quest_items.toggle(crate::mm_save::MmQuestItems::SONG_TIME);
        }),
    },
    MmSongOfHealing: Simple {
        img: ImageInfo::mm("song_of_healing"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|mm| mm.has_song_of_healing())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.quest_items.toggle(crate::mm_save::MmQuestItems::SONG_HEALING);
        }),
    },
    MmEponasSong: Simple {
        img: ImageInfo::mm("eponas_song"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|mm| mm.has_eponas_song())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.quest_items.toggle(crate::mm_save::MmQuestItems::SONG_EPONA);
        }),
    },
    MmSongOfSoaring: Simple {
        img: ImageInfo::mm("song_of_soaring"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|mm| mm.has_song_of_soaring())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.quest_items.toggle(crate::mm_save::MmQuestItems::SONG_SOARING);
        }),
    },
    MmSongOfStorms: Simple {
        img: ImageInfo::mm("song_of_storms"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|mm| mm.has_song_of_storms())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.quest_items.toggle(crate::mm_save::MmQuestItems::SONG_STORMS);
        }),
    },
    MmSonataOfAwakening: Simple {
        img: ImageInfo::mm("sonata_of_awakening"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|mm| mm.has_sonata_of_awakening())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.quest_items.toggle(crate::mm_save::MmQuestItems::SONG_AWAKENING);
        }),
    },
    MmGoronLullaby: Simple {
        img: ImageInfo::mm("goron_lullaby"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|mm| mm.has_goron_lullaby())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.quest_items.toggle(crate::mm_save::MmQuestItems::SONG_GORON);
        }),
    },
    MmNewWaveBossaNova: Simple {
        img: ImageInfo::mm("new_wave_bossa_nova"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|mm| mm.has_new_wave_bossa_nova())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.quest_items.toggle(crate::mm_save::MmQuestItems::SONG_ZORA);
        }),
    },
    MmElegyOfEmptiness: Simple {
        img: ImageInfo::mm("elegy_of_emptiness"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|mm| mm.has_elegy_of_emptiness())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.quest_items.toggle(crate::mm_save::MmQuestItems::SONG_EMPTINESS);
        }),
    },
    MmOathToOrder: Simple {
        img: ImageInfo::mm("oath_to_order"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|mm| mm.has_oath_to_order())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.quest_items.toggle(crate::mm_save::MmQuestItems::SONG_ORDER);
        }),
    },

    // ============================================================================
    // MM Items - Bomber's Notebook
    // ============================================================================
    MmBomberNotebook: Simple {
        img: ImageInfo::mm("bomber_notebook"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_bombers_notebook())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.quest_items.toggle(crate::mm_save::MmQuestItems::NOTEBOOK);
        }),
    },

    // ============================================================================
    // MM Items - Equipment
    // ============================================================================
    MmOcarina: Simple {
        img: ImageInfo::mm("ocarina"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_ocarina())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.inventory.ocarina = !mm_save.inventory.ocarina;
        }),
    },
    MmHerosBow: Simple {
        img: ImageInfo::mm("heros_bow"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_heros_bow())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.inventory.bow = !mm_save.inventory.bow;
        }),
    },
    MmFireArrow: Simple {
        img: ImageInfo::mm("fire_arrow"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_fire_arrow())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.inventory.fire_arrows = !mm_save.inventory.fire_arrows;
        }),
    },
    MmIceArrow: Simple {
        img: ImageInfo::mm("ice_arrow"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_ice_arrow())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.inventory.ice_arrows = !mm_save.inventory.ice_arrows;
        }),
    },
    MmLightArrow: Simple {
        img: ImageInfo::mm("light_arrow"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_light_arrow())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.inventory.light_arrows = !mm_save.inventory.light_arrows;
        }),
    },
    MmHookshot: Simple {
        img: ImageInfo::mm("hookshot"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_hookshot())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.inventory.hookshot = !mm_save.inventory.hookshot;
        }),
    },
    MmBombs: Simple {
        img: ImageInfo::mm("bombs"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_bombs())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.inventory.bombs = !mm_save.inventory.bombs;
        }),
    },
    MmBombchu: Simple {
        img: ImageInfo::mm("bombchu"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_bombchu())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.inventory.bombchus = !mm_save.inventory.bombchus;
        }),
    },
    MmPowderKeg: Simple {
        img: ImageInfo::mm("powder_keg"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_powder_keg())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.inventory.powder_keg = !mm_save.inventory.powder_keg;
        }),
    },
    MmLensOfTruth: Simple {
        img: ImageInfo::mm("lens_of_truth"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_lens_of_truth())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.inventory.lens = !mm_save.inventory.lens;
        }),
    },
    MmPictographBox: Simple {
        img: ImageInfo::mm("pictograph_box"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_pictograph_box())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.inventory.pictograph_box = !mm_save.inventory.pictograph_box;
        }),
    },
    MmGreatFairySword: Simple {
        img: ImageInfo::mm("great_fairy_sword"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_great_fairy_sword())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.inventory.great_fairy_sword = !mm_save.inventory.great_fairy_sword;
        }),
    },
    MmMagicBean: Simple {
        img: ImageInfo::mm("magic_bean"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_magic_bean())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.inventory.magic_beans = !mm_save.inventory.magic_beans;
        }),
    },

    // ============================================================================
    // MM Items - Swords
    // ============================================================================
    MmSword: Sequence {
        idx: Box::new(|state| {
            state.ram.mm_save.as_ref().map_or(0, |mm| match mm.sword {
                crate::mm_save::MmSword::None => 0,
                crate::mm_save::MmSword::KokiriSword => 1,
                crate::mm_save::MmSword::RazorSword => 2,
                crate::mm_save::MmSword::GildedSword => 3,
            })
        }),
        img: Box::new(|state| {
            state.ram.mm_save.as_ref().map_or((false, ImageInfo::mm("kokiri_sword")), |mm| match mm.sword {
                crate::mm_save::MmSword::None => (false, ImageInfo::mm("kokiri_sword")),
                crate::mm_save::MmSword::KokiriSword => (true, ImageInfo::mm("kokiri_sword")),
                crate::mm_save::MmSword::RazorSword => (true, ImageInfo::mm("razor_sword")),
                crate::mm_save::MmSword::GildedSword => (true, ImageInfo::mm("gilded_sword")),
            })
        }),
        increment: Box::new(|state| {
            if let Some(mm) = state.ram.mm_save.as_mut() {
                mm.sword = match mm.sword {
                    crate::mm_save::MmSword::None => crate::mm_save::MmSword::KokiriSword,
                    crate::mm_save::MmSword::KokiriSword => crate::mm_save::MmSword::RazorSword,
                    crate::mm_save::MmSword::RazorSword => crate::mm_save::MmSword::GildedSword,
                    crate::mm_save::MmSword::GildedSword => crate::mm_save::MmSword::None,
                };
            }
        }),
        decrement: Box::new(|state| {
            if let Some(mm) = state.ram.mm_save.as_mut() {
                mm.sword = match mm.sword {
                    crate::mm_save::MmSword::None => crate::mm_save::MmSword::GildedSword,
                    crate::mm_save::MmSword::KokiriSword => crate::mm_save::MmSword::None,
                    crate::mm_save::MmSword::RazorSword => crate::mm_save::MmSword::KokiriSword,
                    crate::mm_save::MmSword::GildedSword => crate::mm_save::MmSword::RazorSword,
                };
            }
        }),
    },

    // ============================================================================
    // MM Items - Shields
    // ============================================================================
    MmShield: Sequence {
        idx: Box::new(|state| {
            state.ram.mm_save.as_ref().map_or(0, |mm| match mm.shield {
                crate::mm_save::MmShield::None => 0,
                crate::mm_save::MmShield::HeroShield => 1,
                crate::mm_save::MmShield::HylianShield => 2,
                crate::mm_save::MmShield::MirrorShield => 3,
            })
        }),
        img: Box::new(|state| {
            state.ram.mm_save.as_ref().map_or((false, ImageInfo::mm("hero_shield")), |mm| match mm.shield {
                crate::mm_save::MmShield::None => (false, ImageInfo::mm("hero_shield")),
                crate::mm_save::MmShield::HeroShield => (true, ImageInfo::mm("hero_shield")),
                crate::mm_save::MmShield::HylianShield => (true, ImageInfo::mm("hero_shield")), // TODO: add hylian shield image
                crate::mm_save::MmShield::MirrorShield => (true, ImageInfo::mm("mirror_shield")),
            })
        }),
        increment: Box::new(|state| {
            if let Some(mm) = state.ram.mm_save.as_mut() {
                mm.shield = match mm.shield {
                    crate::mm_save::MmShield::None => crate::mm_save::MmShield::HeroShield,
                    crate::mm_save::MmShield::HeroShield => crate::mm_save::MmShield::HylianShield,
                    crate::mm_save::MmShield::HylianShield => crate::mm_save::MmShield::MirrorShield,
                    crate::mm_save::MmShield::MirrorShield => crate::mm_save::MmShield::None,
                };
            }
        }),
        decrement: Box::new(|state| {
            if let Some(mm) = state.ram.mm_save.as_mut() {
                mm.shield = match mm.shield {
                    crate::mm_save::MmShield::None => crate::mm_save::MmShield::MirrorShield,
                    crate::mm_save::MmShield::HeroShield => crate::mm_save::MmShield::None,
                    crate::mm_save::MmShield::HylianShield => crate::mm_save::MmShield::HeroShield,
                    crate::mm_save::MmShield::MirrorShield => crate::mm_save::MmShield::HylianShield,
                };
            }
        }),
    },

    // ============================================================================
    // MM Items - Bottles
    // ============================================================================
    MmBottle: Count {
        dimmed_img: ImageInfo::mm("bottle"),
        img: ImageInfo::mm("bottle"),
        get: Box::new(|state| {
            state.ram.mm_save.as_ref().map_or(0, |mm| {
                mm.inventory.bottles.iter().filter(|&&b| b != crate::mm_save::MmBottle::None).count() as u8
            })
        }),
        set: Box::new(|state, value| {
            if let Some(mm) = state.ram.mm_save.as_mut() {
                // Set bottles to Empty up to the value, then None for the rest
                for (i, bottle) in mm.inventory.bottles.iter_mut().enumerate() {
                    *bottle = if (i as u8) < value {
                        // Preserve existing bottle content, or set to Empty if was None
                        if *bottle == crate::mm_save::MmBottle::None {
                            crate::mm_save::MmBottle::Empty
                        } else {
                            *bottle
                        }
                    } else {
                        crate::mm_save::MmBottle::None
                    };
                }
            }
        }),
        max: 6,
        step: 1,
    },

    // ============================================================================
    // MM Items - Wallet/Upgrades
    // ============================================================================
    MmWallet: Sequence {
        idx: Box::new(|state| {
            state.ram.mm_save.as_ref().map_or(0, |mm| {
                let wallet = mm.upgrades.wallet();
                if wallet == crate::mm_save::MmUpgrades::GIANTS_WALLET {
                    2
                } else if wallet == crate::mm_save::MmUpgrades::ADULTS_WALLET {
                    1
                } else {
                    0
                }
            })
        }),
        img: Box::new(|state| {
            state.ram.mm_save.as_ref().map_or((false, ImageInfo::mm("wallet")), |mm| {
                let wallet = mm.upgrades.wallet();
                if wallet == crate::mm_save::MmUpgrades::GIANTS_WALLET {
                    (true, ImageInfo::mm("giants_wallet"))
                } else if wallet == crate::mm_save::MmUpgrades::ADULTS_WALLET {
                    (true, ImageInfo::mm("adults_wallet"))
                } else {
                    (false, ImageInfo::mm("wallet"))
                }
            })
        }),
        increment: Box::new(|state| {
            if let Some(mm) = state.ram.mm_save.as_mut() {
                let new_val = {
                    let wallet = mm.upgrades.wallet();
                    if wallet == crate::mm_save::MmUpgrades::ADULTS_WALLET {
                        crate::mm_save::MmUpgrades::GIANTS_WALLET
                    } else if wallet == crate::mm_save::MmUpgrades::GIANTS_WALLET {
                        crate::mm_save::MmUpgrades::empty()
                    } else {
                        crate::mm_save::MmUpgrades::ADULTS_WALLET
                    }
                };
                mm.upgrades.set_wallet(new_val);
            }
        }),
        decrement: Box::new(|state| {
            if let Some(mm) = state.ram.mm_save.as_mut() {
                let new_val = {
                    let wallet = mm.upgrades.wallet();
                    if wallet == crate::mm_save::MmUpgrades::ADULTS_WALLET {
                        crate::mm_save::MmUpgrades::empty()
                    } else if wallet == crate::mm_save::MmUpgrades::GIANTS_WALLET {
                        crate::mm_save::MmUpgrades::ADULTS_WALLET
                    } else {
                        crate::mm_save::MmUpgrades::GIANTS_WALLET
                    }
                };
                mm.upgrades.set_wallet(new_val);
            }
        }),
    },
    MmMagic: Simple {
        img: ImageInfo::mm("magic"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_magic())),
        toggle: Box::new(|state| {
            let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
            mm_save.magic = if mm_save.magic == crate::mm_save::MmMagicCapacity::None {
                crate::mm_save::MmMagicCapacity::Single
            } else {
                crate::mm_save::MmMagicCapacity::None
            };
        }),
    },
    MmDoubleDefense: Simple {
        img: ImageInfo::mm("double_defense"),
        active: Box::new(|state| {
            state.ram.mm_save.as_ref().is_some_and(|mm| mm.double_defense)
        }),
        toggle: Box::new(|state| {
            if let Some(mm) = state.ram.mm_save.as_mut() {
                mm.double_defense = !mm.double_defense;
            }
        }),
    },

    // ============================================================================
    // Heart Count Cells
    // ============================================================================

    // OoT heart containers count (3-20)
    OotHearts: Count {
        dimmed_img: ImageInfo::extra("heart_container"),
        img: ImageInfo::extra("heart_container"),
        get: Box::new(|state| state.ram.save.heart_containers()),
        set: Box::new(|state, value| {
            // Set health_capacity based on heart containers (each heart = 0x10)
            state.ram.save.health_capacity = (value as u16) * 0x10;
        }),
        max: 20,
        step: 1,
    },

    // OoT heart pieces count (0-3)
    OotHeartPieces: Count {
        dimmed_img: ImageInfo::extra("heart_piece"),
        img: ImageInfo::extra("heart_piece"),
        get: Box::new(|state| state.ram.save.heart_pieces),
        set: Box::new(|state, value| {
            state.ram.save.heart_pieces = value.min(3);
        }),
        max: 3,
        step: 1,
    },

    // MM heart containers count (3-20)
    MmHearts: Count {
        dimmed_img: ImageInfo::extra("heart_container"),
        img: ImageInfo::extra("heart_container"),
        get: Box::new(|state| {
            state.ram.mm_save.as_ref().map_or(0, |mm| mm.heart_containers())
        }),
        set: Box::new(|state, value| {
            if let Some(mm) = state.ram.mm_save.as_mut() {
                mm.health_capacity = (value as u16) * 0x10;
            }
        }),
        max: 20,
        step: 1,
    },

    // MM heart pieces count (0-3)
    MmHeartPieces: Count {
        dimmed_img: ImageInfo::extra("heart_piece"),
        img: ImageInfo::extra("heart_piece"),
        get: Box::new(|state| {
            state.ram.mm_save.as_ref().map_or(0, |mm| mm.quest_items.heart_pieces())
        }),
        set: Box::new(|_state, _value| {
            // Heart pieces in MM are stored in quest_items bitflags - complex to set
        }),
        max: 3,
        step: 1,
    },

    // ============================================================================
    // MM Items - Dungeon Keys
    // ============================================================================
    MmWoodfallSmallKeys: TrackerCellKind::MmSmallKeys {
        get: Box::new(|keys| keys.woodfall),
        set: Box::new(|keys, value| keys.woodfall = value),
        max: 1,
        label: "WF",
    },
    MmSnowheadSmallKeys: TrackerCellKind::MmSmallKeys {
        get: Box::new(|keys| keys.snowhead()),
        set: Box::new(|keys, value| keys.snowhead = value),
        max: 3,
        label: "SH",
    },
    MmGreatBaySmallKeys: TrackerCellKind::MmSmallKeys {
        get: Box::new(|keys| keys.great_bay()),
        set: Box::new(|keys, value| keys.great_bay = value),
        max: 1,
        label: "GB",
    },
    MmStoneTowerSmallKeys: TrackerCellKind::MmSmallKeys {
        get: Box::new(|keys| keys.stone_tower()),
        set: Box::new(|keys, value| keys.stone_tower = value),
        max: 4,
        label: "ST",
    },

    // ============================================================================
    // MM Items - Item Sharing Indicators (OoTMM combo rando)
    // ============================================================================
    MmSharedOcarina: Overlay {
        main_img: ImageInfo::mm("ocarina"),
        overlay_img: ImageInfo::extra("oot_badge"),
        active: Box::new(|state| (
            state.ram.mm_save.as_ref().is_some_and(|save| save.has_ocarina()),
            state.ram.save.inv.ocarina != Ocarina::None,
        )),
        toggle_main: Box::new(|_| ()),
        toggle_overlay: Box::new(|_| ()),
    },
    MmSharedHookshot: Overlay {
        main_img: ImageInfo::mm("hookshot"),
        overlay_img: ImageInfo::extra("oot_badge"),
        active: Box::new(|state| (
            state.ram.mm_save.as_ref().is_some_and(|save| save.has_hookshot()),
            state.ram.save.inv.hookshot != Hookshot::None,
        )),
        toggle_main: Box::new(|_| ()),
        toggle_overlay: Box::new(|_| ()),
    },
    MmSharedBow: Overlay {
        main_img: ImageInfo::mm("heros_bow"),
        overlay_img: ImageInfo::extra("oot_badge"),
        active: Box::new(|state| (
            state.ram.mm_save.as_ref().is_some_and(|save| save.has_heros_bow()),
            state.ram.save.inv.bow,
        )),
        toggle_main: Box::new(|_| ()),
        toggle_overlay: Box::new(|_| ()),
    },
    MmSharedBombs: Overlay {
        main_img: ImageInfo::mm("bombs"),
        overlay_img: ImageInfo::extra("oot_badge"),
        active: Box::new(|state| (
            state.ram.mm_save.as_ref().is_some_and(|save| save.has_bombs()),
            state.ram.save.upgrades.bomb_bag() != Upgrades::NONE,
        )),
        toggle_main: Box::new(|_| ()),
        toggle_overlay: Box::new(|_| ()),
    },
    MmSharedMagic: Overlay {
        main_img: ImageInfo::mm("magic"),
        overlay_img: ImageInfo::extra("oot_badge"),
        active: Box::new(|state| (
            state.ram.mm_save.as_ref().is_some_and(|save| save.has_magic()),
            state.ram.save.magic != MagicCapacity::None,
        )),
        toggle_main: Box::new(|_| ()),
        toggle_overlay: Box::new(|_| ()),
    },
    MmSharedLens: Overlay {
        main_img: ImageInfo::mm("lens_of_truth"),
        overlay_img: ImageInfo::extra("oot_badge"),
        active: Box::new(|state| (
            state.ram.mm_save.as_ref().is_some_and(|save| save.has_lens_of_truth()),
            state.ram.save.inv.lens,
        )),
        toggle_main: Box::new(|_| ()),
        toggle_overlay: Box::new(|_| ()),
    },
    MmSharedWallet: Overlay {
        main_img: ImageInfo::mm("wallet"),
        overlay_img: ImageInfo::extra("oot_badge"),
        active: Box::new(|state| (
            state.ram.mm_save.as_ref().is_some_and(|save| save.upgrades.wallet() != crate::mm_save::MmUpgrades::empty()),
            state.ram.save.upgrades.wallet() != Upgrades::NONE,
        )),
        toggle_main: Box::new(|_| ()),
        toggle_overlay: Box::new(|_| ()),
    },
}
