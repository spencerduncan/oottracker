impl TrackerCellKind {
    pub fn render(&self, state: &ModelState) -> CellRender {
        match self {
            BigPoeTriforce => {
                if state.ram.save.triforce_pieces() > 0 {
                    CellRender {
                        img: ImageInfo::new("triforce"),
                        style: CellStyle::Normal,
                        overlay: CellOverlay::Count {
                            count: state.ram.save.triforce_pieces(),
                            max: 0, // Triforce max is configurable and unknown
                            count_img: ImageInfo::new("force"),
                        },
                        accessibility: None,
                        label: None,
                    }
                } else if state.ram.save.big_poes > 0 {
                    //TODO show dimmed Triforce icon if it's known that it's TH
                    CellRender {
                        img: ImageInfo::extra("big_poe"),
                        style: CellStyle::Normal,
                        overlay: CellOverlay::Count {
                            count: state.ram.save.big_poes,
                            max: 10,
                            count_img: ImageInfo::extra("poes"),
                        },
                        accessibility: None,
                        label: None,
                    }
                } else {
                    CellRender {
                        img: ImageInfo::extra("big_poe"),
                        style: CellStyle::Dimmed,
                        overlay: CellOverlay::None,
                        accessibility: None,
                        label: None,
                    }
                }
            }
            BossKey { active, label, .. } => CellRender {
                img: ImageInfo::extra("boss_key"),
                style: if active(&state.ram.save.dungeon_items) {
                    CellStyle::Normal
                } else {
                    CellStyle::Dimmed
                },
                overlay: CellOverlay::None,
                accessibility: None,
                label: Some(label.to_string()),
            },
            OotMap { active, label, .. } => CellRender {
                img: ImageInfo::extra("map"),
                style: if active(&state.ram.save.dungeon_items) {
                    CellStyle::Normal
                } else {
                    CellStyle::Dimmed
                },
                overlay: CellOverlay::None,
                accessibility: None,
                label: Some(label.to_string()),
            },
            OotCompass { active, label, .. } => CellRender {
                img: ImageInfo::extra("compass"),
                style: if active(&state.ram.save.dungeon_items) {
                    CellStyle::Normal
                } else {
                    CellStyle::Dimmed
                },
                overlay: CellOverlay::None,
                accessibility: None,
                label: Some(label.to_string()),
            },
            MmBossKey { active, label, .. } => CellRender {
                img: ImageInfo::extra("boss_key"),
                style: if state
                    .ram
                    .mm_save
                    .as_ref()
                    .is_some_and(|mm| active(&mm.dungeon_items))
                {
                    CellStyle::Normal
                } else {
                    CellStyle::Dimmed
                },
                overlay: CellOverlay::None,
                accessibility: None,
                label: Some(label.to_string()),
            },
            MmMap { active, label, .. } => CellRender {
                img: ImageInfo::extra("map"),
                style: if state
                    .ram
                    .mm_save
                    .as_ref()
                    .is_some_and(|mm| active(&mm.dungeon_items))
                {
                    CellStyle::Normal
                } else {
                    CellStyle::Dimmed
                },
                overlay: CellOverlay::None,
                accessibility: None,
                label: Some(label.to_string()),
            },
            MmCompass { active, label, .. } => CellRender {
                img: ImageInfo::extra("compass"),
                style: if state
                    .ram
                    .mm_save
                    .as_ref()
                    .is_some_and(|mm| active(&mm.dungeon_items))
                {
                    CellStyle::Normal
                } else {
                    CellStyle::Dimmed
                },
                overlay: CellOverlay::None,
                accessibility: None,
                label: Some(label.to_string()),
            },
            Composite {
                left_img,
                right_img,
                both_img,
                active,
                ..
            } => {
                let is_active = active(state);
                let img = match is_active {
                    (false, false) | (true, true) => both_img,
                    (false, true) => right_img,
                    (true, false) => left_img,
                }
                .clone();
                CellRender {
                    img,
                    style: if let (false, false) = is_active {
                        CellStyle::Dimmed
                    } else {
                        CellStyle::Normal
                    },
                    overlay: CellOverlay::None,
                    accessibility: None,
                    label: None,
                }
            }
            CompositeKeys { boss, small } => {
                let (has_boss_key, num_small_keys, max_keys, label) = if let (
                    BossKey { active, label, .. },
                    TrackerCellKind::SmallKeys {
                        get,
                        max_vanilla,
                        max_mq,
                        ..
                    },
                ) =
                    (boss.kind(), small.kind())
                {
                    (
                        active(&state.ram.save.dungeon_items),
                        get(&state.ram.save.small_keys),
                        max_vanilla.max(max_mq),
                        label,
                    )
                } else {
                    unimplemented!("CompositeKeys that aren't SmallKeys + BossKey")
                };
                CellRender {
                    img: ImageInfo::extra("keys"),
                    style: match (has_boss_key, num_small_keys) {
                        (false, 0) => CellStyle::Dimmed,
                        (false, _) => CellStyle::LeftDimmed,
                        (true, 0) => CellStyle::RightDimmed,
                        (true, _) => CellStyle::Normal,
                    },
                    overlay: if num_small_keys > 0 {
                        CellOverlay::Count {
                            count: num_small_keys,
                            max: max_keys,
                            count_img: ImageInfo::new("UNIMPLEMENTED"), //TODO
                        }
                    } else {
                        CellOverlay::None
                    },
                    accessibility: None,
                    label: Some(label.to_string()),
                }
            }
            Count {
                dimmed_img,
                img,
                get,
                max,
                ..
            } => {
                let count = get(state);
                let (style, overlay) = if count == 0 {
                    (CellStyle::Dimmed, CellOverlay::None)
                } else {
                    (
                        CellStyle::Normal,
                        CellOverlay::CountWithMax {
                            count,
                            max: *max,
                            count_img: img.clone(),
                        },
                    )
                };
                CellRender {
                    img: dimmed_img.clone(),
                    style,
                    overlay,
                    accessibility: None,
                    label: None,
                }
            }
            DynamicCount {
                dimmed_img,
                img,
                get,
                max_fn,
                ..
            } => {
                let count = get(state);
                let max = max_fn(state);
                let (style, overlay) = if count == 0 {
                    (CellStyle::Dimmed, CellOverlay::None)
                } else {
                    (
                        CellStyle::Normal,
                        CellOverlay::CountWithMax {
                            count,
                            max,
                            count_img: img.clone(),
                        },
                    )
                };
                CellRender {
                    img: dimmed_img.clone(),
                    style,
                    overlay,
                    accessibility: None,
                    label: None,
                }
            }
            FortressMq => {
                CellRender {
                    img: ImageInfo::extra("blank"),
                    style: CellStyle::Normal,
                    overlay: CellOverlay::Location {
                        loc: ImageInfo::extra("fort_text"),
                        style: if state
                            .knowledge
                            .string_settings
                            .get("gerudo_fortress")
                            .is_some_and(|values| values.iter().eq(iter::once("normal")))
                        {
                            LocationStyle::Mq
                        } else {
                            LocationStyle::Normal
                        }, //TODO dim if unknown?
                    },
                    accessibility: None,
                    label: None,
                }
            }
            FreeReward => {
                let reward = state
                    .knowledge
                    .dungeon_reward_locations
                    .iter()
                    .filter_map(|(reward, &loc)| {
                        if loc == DungeonRewardLocation::LinksPocket {
                            Some(reward)
                        } else {
                            None
                        }
                    })
                    .exactly_one()
                    .ok();
                CellRender {
                    img: ImageInfo {
                        dir: if reward.is_some() {
                            ImageDir::Xopar
                        } else {
                            ImageDir::Extra
                        },
                        name: match reward {
                            Some(DungeonReward::Medallion(med)) => Cow::Owned(format!(
                                "{}_medallion",
                                med.element().to_ascii_lowercase()
                            )),
                            Some(DungeonReward::Stone(Stone::KokiriEmerald)) => {
                                Cow::Borrowed("kokiri_emerald")
                            }
                            Some(DungeonReward::Stone(Stone::GoronRuby)) => {
                                Cow::Borrowed("goron_ruby")
                            }
                            Some(DungeonReward::Stone(Stone::ZoraSapphire)) => {
                                Cow::Borrowed("zora_sapphire")
                            }
                            None => Cow::Borrowed("blank"), //TODO "unknown dungeon reward" image?
                        },
                    },
                    style: CellStyle::Normal,
                    overlay: CellOverlay::Location {
                        loc: ImageInfo::new("free_text"),
                        style: LocationStyle::Normal,
                    },
                    accessibility: None,
                    label: None,
                }
            }
            GoBk => CellRender {
                img: ImageInfo::extra(match state.knowledge.progression_mode {
                    ProgressionMode::Done => "blank",
                    ProgressionMode::Bk => "bk_mode",
                    ProgressionMode::Go | ProgressionMode::Normal => "go_mode",
                }),
                style: if state.knowledge.progression_mode == ProgressionMode::Normal {
                    CellStyle::Dimmed
                } else {
                    CellStyle::Normal
                },
                overlay: CellOverlay::None, //TODO overlay with finish time?
                accessibility: None,
                label: None,
            },
            MagicLens => CellRender {
                img: if state.ram.save.magic == MagicCapacity::Large {
                    ImageInfo::new("magic")
                } else {
                    ImageInfo::extra("small_magic")
                },
                style: if state.ram.save.magic == MagicCapacity::None {
                    CellStyle::Dimmed
                } else {
                    CellStyle::Normal
                },
                overlay: if state.ram.save.inv.lens {
                    CellOverlay::Image(ImageInfo::new("lens"))
                } else {
                    CellOverlay::None
                },
                accessibility: None,
                label: None,
            },
            Medallion(med) => CellRender {
                img: ImageInfo::new(format!("{}_medallion", med.element().to_ascii_lowercase())),
                style: if state.ram.save.quest_items.has(*med) {
                    CellStyle::Normal
                } else {
                    CellStyle::Dimmed
                },
                overlay: CellOverlay::None,
                accessibility: None,
                label: None,
            },
            MedallionLocation(med) => {
                let location = state
                    .knowledge
                    .dungeon_reward_locations
                    .get(&DungeonReward::Medallion(*med));
                CellRender {
                    img: ImageInfo::new(match location {
                        None => "unknown_text",
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::DekuTree)) => "deku_text",
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::DodongosCavern)) => {
                            "dc_text"
                        }
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::JabuJabu)) => "jabu_text",
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::ForestTemple)) => {
                            "forest_text"
                        }
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::FireTemple)) => {
                            "fire_text"
                        }
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::WaterTemple)) => {
                            "water_text"
                        }
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::ShadowTemple)) => {
                            "shadow_text"
                        }
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::SpiritTemple)) => {
                            "spirit_text"
                        }
                        Some(DungeonRewardLocation::LinksPocket) => "free_text",
                    }),
                    style: if location.is_some() {
                        CellStyle::Normal
                    } else {
                        CellStyle::Dimmed
                    },
                    overlay: CellOverlay::None,
                    accessibility: None,
                    label: None,
                }
            }
            MedallionWithLocation(med) => {
                let location = state
                    .knowledge
                    .dungeon_reward_locations
                    .get(&DungeonReward::Medallion(*med));
                let has_medallion = state.ram.save.quest_items.has(*med);
                CellRender {
                    img: ImageInfo::new(format!(
                        "{}_medallion",
                        med.element().to_ascii_lowercase()
                    )),
                    style: if has_medallion {
                        CellStyle::Normal
                    } else {
                        CellStyle::Dimmed
                    },
                    overlay: CellOverlay::Location {
                        loc: ImageInfo::new(match location {
                            None => "unknown_text",
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::DekuTree)) => {
                                "deku_text"
                            }
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::DodongosCavern)) => {
                                "dc_text"
                            }
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::JabuJabu)) => {
                                "jabu_text"
                            }
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::ForestTemple)) => {
                                "forest_text"
                            }
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::FireTemple)) => {
                                "fire_text"
                            }
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::WaterTemple)) => {
                                "water_text"
                            }
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::ShadowTemple)) => {
                                "shadow_text"
                            }
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::SpiritTemple)) => {
                                "spirit_text"
                            }
                            Some(DungeonRewardLocation::LinksPocket) => "free_text",
                        }),
                        style: if location.is_some() {
                            LocationStyle::Normal
                        } else {
                            LocationStyle::Dimmed
                        },
                    },
                    // Show accessibility status: Checked if medallion obtained
                    accessibility: if has_medallion {
                        Some(AccessibilityStatus::Checked)
                    } else {
                        None
                    },
                    label: None,
                }
            }
            Mq(dungeon) => {
                let reward = if let Dungeon::Main(main_dungeon) = *dungeon {
                    state
                        .knowledge
                        .dungeon_reward_locations
                        .iter()
                        .filter_map(|(reward, &loc)| {
                            if loc == DungeonRewardLocation::Dungeon(main_dungeon) {
                                Some(reward)
                            } else {
                                None
                            }
                        })
                        .exactly_one()
                        .ok()
                } else {
                    None
                };
                CellRender {
                    img: ImageInfo {
                        dir: if reward.is_some() {
                            ImageDir::Xopar
                        } else {
                            ImageDir::Extra
                        },
                        name: match reward {
                            Some(DungeonReward::Medallion(med)) => Cow::Owned(format!(
                                "{}_medallion",
                                med.element().to_ascii_lowercase()
                            )),
                            Some(DungeonReward::Stone(Stone::KokiriEmerald)) => {
                                Cow::Borrowed("kokiri_emerald")
                            }
                            Some(DungeonReward::Stone(Stone::GoronRuby)) => {
                                Cow::Borrowed("goron_ruby")
                            }
                            Some(DungeonReward::Stone(Stone::ZoraSapphire)) => {
                                Cow::Borrowed("zora_sapphire")
                            }
                            None => Cow::Borrowed("blank"), //TODO "unknown dungeon reward" image? (only for dungeons that have rewards)
                        },
                    },
                    style: if reward.is_some_and(|&reward| state.ram.save.quest_items.has(reward)) {
                        CellStyle::Normal
                    } else {
                        CellStyle::Dimmed
                    },
                    overlay: CellOverlay::Location {
                        loc: ImageInfo {
                            dir: if let Dungeon::Main(_) = dungeon {
                                ImageDir::Xopar
                            } else {
                                ImageDir::Extra
                            },
                            name: Cow::Borrowed(match dungeon {
                                Dungeon::Main(MainDungeon::DekuTree) => "deku_text",
                                Dungeon::Main(MainDungeon::DodongosCavern) => "dc_text",
                                Dungeon::Main(MainDungeon::JabuJabu) => "jabu_text",
                                Dungeon::Main(MainDungeon::ForestTemple) => "forest_text",
                                Dungeon::Main(MainDungeon::FireTemple) => "fire_text",
                                Dungeon::Main(MainDungeon::WaterTemple) => "water_text",
                                Dungeon::Main(MainDungeon::ShadowTemple) => "shadow_text",
                                Dungeon::Main(MainDungeon::SpiritTemple) => "spirit_text",
                                Dungeon::IceCavern => "ice_text",
                                Dungeon::BottomOfTheWell => "well_text",
                                Dungeon::GerudoTrainingGround => "gtg_text",
                                Dungeon::GanonsCastle => "ganon_text",
                            }),
                        },
                        style: if state.knowledge.mq.get(dungeon) == Some(&Mq::Mq) {
                            LocationStyle::Mq
                        } else {
                            LocationStyle::Normal
                        },
                    },
                    accessibility: None,
                    label: None,
                }
            }
            OptionalOverlay {
                main_img,
                overlay_img,
                active,
                ..
            }
            | Overlay {
                main_img,
                overlay_img,
                active,
                ..
            } => {
                let (main_active, overlay_active) = active(state);
                CellRender {
                    img: main_img.clone(),
                    style: if main_active {
                        CellStyle::Normal
                    } else {
                        CellStyle::Dimmed
                    },
                    overlay: if overlay_active {
                        CellOverlay::Image(overlay_img.clone())
                    } else {
                        CellOverlay::None
                    },
                    accessibility: None,
                    label: None,
                }
            }
            Sequence { img, .. } => {
                let (is_active, img) = img(state);
                CellRender {
                    img,
                    style: if is_active {
                        CellStyle::Normal
                    } else {
                        CellStyle::Dimmed
                    },
                    overlay: CellOverlay::None,
                    accessibility: None,
                    label: None,
                }
            }
            Simple { img, active, .. } => CellRender {
                img: img.clone(),
                style: if active(state) {
                    CellStyle::Normal
                } else {
                    CellStyle::Dimmed
                },
                overlay: CellOverlay::None,
                accessibility: None,
                label: None,
            },
            TrackerCellKind::SmallKeys {
                get,
                max_vanilla,
                max_mq,
                label,
                ..
            } => {
                let num_small_keys = get(&state.ram.save.small_keys);
                let max_keys = *max_vanilla.max(max_mq);
                CellRender {
                    img: ImageInfo::extra("small_key"),
                    style: if num_small_keys > 0 {
                        CellStyle::Normal
                    } else {
                        CellStyle::Dimmed
                    },
                    overlay: if num_small_keys > 0 {
                        CellOverlay::Count {
                            count: num_small_keys,
                            max: max_keys,
                            count_img: ImageInfo::new("UNIMPLEMENTED"), //TODO
                        }
                    } else {
                        CellOverlay::None
                    },
                    accessibility: None,
                    label: Some(label.to_string()),
                }
            }
            TrackerCellKind::MmSmallKeys {
                get, max, label, ..
            } => {
                let num_small_keys = state
                    .ram
                    .mm_save
                    .as_ref()
                    .map(|s| get(&s.small_keys))
                    .unwrap_or(0);
                CellRender {
                    img: ImageInfo::extra("small_key"),
                    style: if num_small_keys > 0 {
                        CellStyle::Normal
                    } else {
                        CellStyle::Dimmed
                    },
                    overlay: if num_small_keys > 0 {
                        CellOverlay::Count {
                            count: num_small_keys,
                            max: *max,
                            count_img: ImageInfo::new("UNIMPLEMENTED"), //TODO
                        }
                    } else {
                        CellOverlay::None
                    },
                    accessibility: None,
                    label: Some(label.to_string()),
                }
            }
            Song { song, check, .. } => {
                let is_check_completed = Check::<ootr_static::Rando>::Location(check.to_string())
                    .checked(state)
                    .unwrap_or(None)
                    .unwrap_or(false);
                CellRender {
                    img: ImageInfo::new(match *song {
                        QuestItems::ZELDAS_LULLABY => "lullaby",
                        QuestItems::EPONAS_SONG => "epona",
                        QuestItems::SARIAS_SONG => "saria",
                        QuestItems::SUNS_SONG => "sun",
                        QuestItems::SONG_OF_TIME => "time",
                        QuestItems::SONG_OF_STORMS => "storms",
                        QuestItems::MINUET_OF_FOREST => "minuet",
                        QuestItems::BOLERO_OF_FIRE => "bolero",
                        QuestItems::SERENADE_OF_WATER => "serenade",
                        QuestItems::NOCTURNE_OF_SHADOW => "nocturne",
                        QuestItems::REQUIEM_OF_SPIRIT => "requiem",
                        QuestItems::PRELUDE_OF_LIGHT => "prelude",
                        _ => unreachable!(),
                    }),
                    style: if state.ram.save.quest_items.contains(*song) {
                        CellStyle::Normal
                    } else {
                        CellStyle::Dimmed
                    },
                    overlay: if is_check_completed {
                        //TODO allow ootr_dynamic::Rando
                        CellOverlay::Image(ImageInfo::new("check"))
                    } else {
                        CellOverlay::None
                    },
                    // Show accessibility status: Checked if song location has been collected
                    accessibility: if is_check_completed {
                        Some(AccessibilityStatus::Checked)
                    } else {
                        None
                    },
                    label: None,
                }
            }
            SongCheck { check, .. } => {
                let is_checked = Check::<ootr_static::Rando>::Location(check.to_string())
                    .checked(state)
                    .unwrap_or(None)
                    .unwrap_or(false);
                CellRender {
                    img: ImageInfo::extra("blank"),
                    style: CellStyle::Normal,
                    overlay: if is_checked {
                        //TODO allow ootr_dynamic::Rando
                        CellOverlay::Image(ImageInfo::new("check"))
                    } else {
                        CellOverlay::None
                    },
                    // Show accessibility status: Checked if song location has been collected
                    accessibility: if is_checked {
                        Some(AccessibilityStatus::Checked)
                    } else {
                        None
                    },
                    label: None,
                }
            }
            Spells => CellRender {
                img: match (
                    state.ram.save.inv.dins_fire,
                    state.ram.save.inv.farores_wind,
                    state.ram.save.inv.nayrus_love,
                ) {
                    (false, false, false) | (true, true, false) => {
                        ImageInfo::new("composite_magic")
                    } //TODO use "spells" for dimmed instead if shift-click is available or auto-tracking?
                    (false, false, true) => ImageInfo::extra("nayrus_love"),
                    (false, true, false) => ImageInfo::new("faores_wind"),
                    (false, true, true) => ImageInfo::extra("farores_nayrus"),
                    (true, false, false) => ImageInfo::new("dins_fire"),
                    (true, false, true) => ImageInfo::extra("dins_nayrus"),
                    (true, true, true) => ImageInfo::extra("spells"),
                },
                style: if !state.ram.save.inv.dins_fire
                    && !state.ram.save.inv.farores_wind
                    && !state.ram.save.inv.nayrus_love
                {
                    CellStyle::Dimmed
                } else {
                    CellStyle::Normal
                },
                overlay: CellOverlay::None,
                accessibility: None,
                label: None,
            },
            Stone(stone) => CellRender {
                img: ImageInfo::new(match *stone {
                    Stone::KokiriEmerald => "kokiri_emerald",
                    Stone::GoronRuby => "goron_ruby",
                    Stone::ZoraSapphire => "zora_sapphire",
                }),
                style: if state.ram.save.quest_items.has(*stone) {
                    CellStyle::Normal
                } else {
                    CellStyle::Dimmed
                },
                overlay: CellOverlay::None,
                accessibility: None,
                label: None,
            },
            StoneLocation(stone) => {
                let location = state
                    .knowledge
                    .dungeon_reward_locations
                    .get(&DungeonReward::Stone(*stone));
                CellRender {
                    img: ImageInfo::new(match location {
                        None => "unknown_text",
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::DekuTree)) => "deku_text",
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::DodongosCavern)) => {
                            "dc_text"
                        }
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::JabuJabu)) => "jabu_text",
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::ForestTemple)) => {
                            "forest_text"
                        }
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::FireTemple)) => {
                            "fire_text"
                        }
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::WaterTemple)) => {
                            "water_text"
                        }
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::ShadowTemple)) => {
                            "shadow_text"
                        }
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::SpiritTemple)) => {
                            "spirit_text"
                        }
                        Some(DungeonRewardLocation::LinksPocket) => "free_text",
                    }),
                    style: if location.is_some() {
                        CellStyle::Normal
                    } else {
                        CellStyle::Dimmed
                    },
                    overlay: CellOverlay::None,
                    accessibility: None,
                    label: None,
                }
            }
            StoneWithLocation(stone) => {
                let location = state
                    .knowledge
                    .dungeon_reward_locations
                    .get(&DungeonReward::Stone(*stone));
                let has_stone = state.ram.save.quest_items.has(*stone);
                CellRender {
                    img: ImageInfo::new(match *stone {
                        Stone::KokiriEmerald => "kokiri_emerald",
                        Stone::GoronRuby => "goron_ruby",
                        Stone::ZoraSapphire => "zora_sapphire",
                    }),
                    style: if has_stone {
                        CellStyle::Normal
                    } else {
                        CellStyle::Dimmed
                    },
                    overlay: CellOverlay::Location {
                        loc: ImageInfo::new(match location {
                            None => "unknown_text",
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::DekuTree)) => {
                                "deku_text"
                            }
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::DodongosCavern)) => {
                                "dc_text"
                            }
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::JabuJabu)) => {
                                "jabu_text"
                            }
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::ForestTemple)) => {
                                "forest_text"
                            }
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::FireTemple)) => {
                                "fire_text"
                            }
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::WaterTemple)) => {
                                "water_text"
                            }
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::ShadowTemple)) => {
                                "shadow_text"
                            }
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::SpiritTemple)) => {
                                "spirit_text"
                            }
                            Some(DungeonRewardLocation::LinksPocket) => "free_text",
                        }),
                        style: if location.is_some() {
                            LocationStyle::Normal
                        } else {
                            LocationStyle::Dimmed
                        },
                    },
                    // Show accessibility status: Checked if stone obtained
                    accessibility: if has_stone {
                        Some(AccessibilityStatus::Checked)
                    } else {
                        None
                    },
                    label: None,
                }
            }
        }
    }

    /// Handle a click action from a frontend that don't distinguish between left and right click.
    pub fn click(&self, state: &mut ModelState) {
        match self {
            Composite {
                active,
                toggle_left,
                toggle_right,
                ..
            }
            | Overlay {
                active,
                toggle_main: toggle_left,
                toggle_overlay: toggle_right,
                ..
            } => {
                let (left, _) = active(state);
                if left {
                    toggle_right(state)
                }
                toggle_left(state);
            }
            OptionalOverlay {
                toggle_main: toggle,
                ..
            }
            | Simple { toggle, .. } => toggle(state),
            CompositeKeys { boss, small } => {
                let (toggle_boss, get_small, set_small, max_small_vanilla, max_small_mq) = if let (
                    BossKey { toggle, .. },
                    TrackerCellKind::SmallKeys {
                        get,
                        set,
                        max_vanilla,
                        max_mq,
                        ..
                    },
                ) =
                    (boss.kind(), small.kind())
                {
                    (toggle, get, set, max_vanilla, max_mq)
                } else {
                    unimplemented!("CompositeKeys that aren't SmallKeys + BossKey")
                };
                let num_small = get_small(&state.ram.save.small_keys);
                if num_small == max_small_vanilla.max(max_small_mq) {
                    //TODO check MQ knowledge? Does plentiful go to +1?
                    set_small(&mut state.ram.save.small_keys, 0);
                    toggle_boss(&mut state.ram.save.dungeon_items);
                } else {
                    set_small(&mut state.ram.save.small_keys, num_small + 1);
                }
            }
            Count {
                get,
                set,
                max,
                step,
                ..
            } => {
                let current = get(state);
                set(
                    state,
                    if current == *max {
                        0
                    } else {
                        current.saturating_add(*step).min(*max)
                    },
                );
            }
            DynamicCount {
                get,
                set,
                max_fn,
                step,
                ..
            } => {
                let current = get(state);
                let max = max_fn(state);
                set(
                    state,
                    if current == max {
                        0
                    } else {
                        current.saturating_add(*step).min(max)
                    },
                );
            }
            FortressMq => {
                if state
                    .knowledge
                    .string_settings
                    .get("gerudo_fortress")
                    .is_some_and(|fort| fort.iter().eq(iter::once("normal")))
                {
                    state.knowledge.string_settings.remove("gerudo_fortress");
                } else {
                    state
                        .knowledge
                        .string_settings
                        .insert("gerudo_fortress".to_string(), collect![format!("normal")]);
                }
            }
            GoBk => {
                state.knowledge.progression_mode = match state.knowledge.progression_mode {
                    ProgressionMode::Normal => ProgressionMode::Go,
                    ProgressionMode::Go => ProgressionMode::Bk,
                    ProgressionMode::Bk => ProgressionMode::Done,
                    ProgressionMode::Done => ProgressionMode::Normal,
                }
            }
            MagicLens => {
                if state.ram.save.magic == MagicCapacity::None {
                    state.ram.save.magic = MagicCapacity::Small;
                } else {
                    state.ram.save.magic = MagicCapacity::None;
                    state.ram.save.inv.lens = !state.ram.save.inv.lens;
                }
            }
            Medallion(med) => state.ram.save.quest_items.toggle(QuestItems::from(med)),
            MedallionLocation(med) => state
                .knowledge
                .dungeon_reward_locations
                .increment(DungeonReward::Medallion(*med)),
            MedallionWithLocation(med) => state
                .knowledge
                .dungeon_reward_locations
                .increment(DungeonReward::Medallion(*med)),
            Mq(dungeon) => {
                if state.knowledge.mq.get(dungeon) == Some(&Mq::Mq) {
                    state.knowledge.mq.remove(dungeon);
                } else {
                    state.knowledge.mq.insert(*dungeon, Mq::Mq);
                }
            }
            Sequence { increment, .. } => increment(state),
            TrackerCellKind::SmallKeys {
                get,
                set,
                max_vanilla,
                max_mq,
                ..
            } => {
                let num = get(&state.ram.save.small_keys);
                if num == *max_vanilla.max(max_mq) {
                    //TODO check MQ knowledge? Does plentiful go to +1?
                    set(&mut state.ram.save.small_keys, 0);
                } else {
                    set(&mut state.ram.save.small_keys, num + 1);
                }
            }
            TrackerCellKind::MmSmallKeys { get, set, max, .. } => {
                let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
                let num = get(&mm_save.small_keys);
                if num == *max {
                    set(&mut mm_save.small_keys, 0);
                } else {
                    set(&mut mm_save.small_keys, num + 1);
                }
            }
            Song {
                song: quest_item, ..
            } => state.ram.save.quest_items.toggle(*quest_item),
            Spells => {
                if state.ram.save.inv.dins_fire {
                    state.ram.save.inv.farores_wind = !state.ram.save.inv.farores_wind
                }
                state.ram.save.inv.dins_fire = !state.ram.save.inv.dins_fire;
            }
            Stone(stone) => state.ram.save.quest_items.toggle(QuestItems::from(stone)),
            StoneLocation(stone) => state
                .knowledge
                .dungeon_reward_locations
                .increment(DungeonReward::Stone(*stone)),
            StoneWithLocation(stone) => state
                .knowledge
                .dungeon_reward_locations
                .increment(DungeonReward::Stone(*stone)),
            FreeReward => {}
            OotMap { toggle, .. } => toggle(&mut state.ram.save.dungeon_items),
            OotCompass { toggle, .. } => toggle(&mut state.ram.save.dungeon_items),
            MmBossKey { toggle, .. } => {
                let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
                toggle(&mut mm_save.dungeon_items);
            }
            MmMap { toggle, .. } => {
                let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
                toggle(&mut mm_save.dungeon_items);
            }
            MmCompass { toggle, .. } => {
                let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
                toggle(&mut mm_save.dungeon_items);
            }
            BossKey { toggle, .. } => toggle(&mut state.ram.save.dungeon_items),
            BigPoeTriforce | SongCheck { .. } => unimplemented!(),
        }
    }

    #[cfg(feature = "iced")]
    /// Returns `true` if the menu should be opened.
    #[must_use]
    pub fn left_click(
        &self,
        can_change_state: bool,
        keyboard_modifiers: KeyboardModifiers,
        state: &mut ModelState,
    ) -> bool {
        //TODO shift-click support
        #[cfg(target_os = "macos")]
        if keyboard_modifiers.control() {
            return self.right_click(can_change_state, keyboard_modifiers, state);
        }
        if can_change_state {
            match self {
                Composite { toggle_left, .. }
                | Overlay {
                    toggle_main: toggle_left,
                    ..
                } => toggle_left(state),
                CompositeKeys { boss, .. } => {
                    if let BossKey { toggle, .. } = boss.kind() {
                        toggle(&mut state.ram.save.dungeon_items);
                    } else {
                        unimplemented!("CompositeKeys that aren't SmallKeys + BossKey")
                    }
                }
                Count {
                    get,
                    set,
                    max,
                    step,
                    ..
                } => {
                    let current = get(state);
                    set(
                        state,
                        if current == *max {
                            0
                        } else {
                            current
                                .saturating_add(
                                    step * if keyboard_modifiers.shift() && *max >= 10 {
                                        10
                                    } else {
                                        1
                                    },
                                )
                                .min(*max)
                        },
                    );
                }
                DynamicCount {
                    get,
                    set,
                    max_fn,
                    step,
                    ..
                } => {
                    let current = get(state);
                    let max = max_fn(state);
                    set(
                        state,
                        if current == max {
                            0
                        } else {
                            current
                                .saturating_add(
                                    step * if keyboard_modifiers.shift() && max >= 10 {
                                        10
                                    } else {
                                        1
                                    },
                                )
                                .min(max)
                        },
                    );
                }
                GoBk => {
                    state.knowledge.progression_mode = match state.knowledge.progression_mode {
                        ProgressionMode::Normal => ProgressionMode::Go,
                        ProgressionMode::Go => ProgressionMode::Normal,
                        ProgressionMode::Bk => ProgressionMode::Done,
                        ProgressionMode::Done => ProgressionMode::Bk,
                    }
                }
                MagicLens => {
                    state.ram.save.magic = match (keyboard_modifiers.shift(), state.ram.save.magic)
                    {
                        (true, MagicCapacity::Large) => MagicCapacity::Small,
                        (true, _) => MagicCapacity::Large,
                        (false, MagicCapacity::None) => MagicCapacity::Small,
                        (false, _) => MagicCapacity::None,
                    }
                }
                Spells => {
                    if keyboard_modifiers.shift() {
                        state.ram.save.inv.nayrus_love = !state.ram.save.inv.nayrus_love;
                    } else {
                        state.ram.save.inv.dins_fire = !state.ram.save.inv.dins_fire;
                    }
                }
                _ => self.click(state),
            }
        }
        false
    }

    #[cfg(feature = "iced")]
    /// Returns `true` if the menu should be opened.
    #[must_use]
    pub fn right_click(
        &self,
        can_change_state: bool,
        keyboard_modifiers: KeyboardModifiers,
        state: &mut ModelState,
    ) -> bool {
        //TODO shift-click support
        if let Medallion(_) = self {
            return true;
        }
        if can_change_state {
            match self {
                Composite { toggle_right, .. }
                | OptionalOverlay {
                    toggle_overlay: toggle_right,
                    ..
                }
                | Overlay {
                    toggle_overlay: toggle_right,
                    ..
                } => toggle_right(state),
                CompositeKeys { small, .. } => {
                    if let TrackerCellKind::SmallKeys {
                        get,
                        set,
                        max_vanilla,
                        max_mq,
                        ..
                    } = small.kind()
                    {
                        let num = get(&state.ram.save.small_keys);
                        if num == max_vanilla.max(max_mq) {
                            //TODO check MQ knowledge? Does plentiful go to +1?
                            set(&mut state.ram.save.small_keys, 0);
                        } else {
                            set(&mut state.ram.save.small_keys, num + 1);
                        }
                    } else {
                        unimplemented!("CompositeKeys that aren't SmallKeys + BossKey")
                    }
                }
                Count {
                    get,
                    set,
                    max,
                    step,
                    ..
                } => {
                    let current = get(state);
                    set(
                        state,
                        if current == 0 {
                            *max
                        } else {
                            current.saturating_sub(
                                step * if keyboard_modifiers.shift() && *max >= 10 {
                                    10
                                } else {
                                    1
                                },
                            )
                        },
                    );
                }
                DynamicCount {
                    get,
                    set,
                    max_fn,
                    step,
                    ..
                } => {
                    let current = get(state);
                    let max = max_fn(state);
                    set(
                        state,
                        if current == 0 {
                            max
                        } else {
                            current.saturating_sub(
                                step * if keyboard_modifiers.shift() && max >= 10 {
                                    10
                                } else {
                                    1
                                },
                            )
                        },
                    );
                }
                GoBk => {
                    state.knowledge.progression_mode = match state.knowledge.progression_mode {
                        ProgressionMode::Normal => ProgressionMode::Bk,
                        ProgressionMode::Bk => ProgressionMode::Normal,
                        ProgressionMode::Go => ProgressionMode::Done,
                        ProgressionMode::Done => ProgressionMode::Go,
                    }
                }
                MagicLens => state.ram.save.inv.lens = !state.ram.save.inv.lens,
                Medallion(_) => unreachable!("already handled above"),
                MedallionLocation(med) => state
                    .knowledge
                    .dungeon_reward_locations
                    .decrement(DungeonReward::Medallion(*med)),
                MedallionWithLocation(med) => {
                    state.ram.save.quest_items.toggle(QuestItems::from(med))
                }
                Sequence { decrement, .. } => decrement(state),
                TrackerCellKind::SmallKeys {
                    get,
                    set,
                    max_vanilla,
                    max_mq,
                    ..
                } => {
                    let num = get(&state.ram.save.small_keys);
                    if num == 0 {
                        set(&mut state.ram.save.small_keys, *max_vanilla.max(max_mq));
                    //TODO check MQ knowledge? Does plentiful go to +1?
                    } else {
                        set(&mut state.ram.save.small_keys, num - 1);
                    }
                }
                TrackerCellKind::MmSmallKeys { get, set, max, .. } => {
                    let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
                    let num = get(&mm_save.small_keys);
                    if num == 0 {
                        set(&mut mm_save.small_keys, *max);
                    } else {
                        set(&mut mm_save.small_keys, num - 1);
                    }
                }
                Song { toggle_overlay, .. } => toggle_overlay(&mut state.ram.save.event_chk_inf),
                Spells => state.ram.save.inv.farores_wind = !state.ram.save.inv.farores_wind,
                StoneLocation(stone) => state
                    .knowledge
                    .dungeon_reward_locations
                    .decrement(DungeonReward::Stone(*stone)),
                StoneWithLocation(stone) => {
                    state.ram.save.quest_items.toggle(QuestItems::from(stone))
                }
                FreeReward | FortressMq | Mq(_) | Simple { .. } | Stone(_) => {}
                OotMap { .. }
                | OotCompass { .. }
                | MmBossKey { .. }
                | MmMap { .. }
                | MmCompass { .. } => {}
                BigPoeTriforce | BossKey { .. } | SongCheck { .. } => unimplemented!(),
            }
        }
        false
    }
}
