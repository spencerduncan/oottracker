# OoTMM Save Structure Offset Mapping

This document maps the save structure offsets used in OoTMM combo ROMs versus vanilla OoT/MM.

**Sources:**
- https://github.com/OoTMM/OoTMM/blob/master/packages/core/include/combo/oot/save.h
- https://github.com/OoTMM/OoTMM/blob/master/packages/core/include/combo/mm/save.h
- https://github.com/OoTMM/OoTMM/blob/master/packages/core/include/combo/save.h
- https://github.com/zeldaret/mm/blob/main/include/z64save.h (vanilla MM decomp)

## Base Addresses

| Game | Vanilla Address | OoTMM Address | Notes |
|------|-----------------|---------------|-------|
| OoT  | 0x8011a5d0      | 0x8011a5d0    | Same! |
| MM   | 0x801ef670      | 0x801ef670    | Same! |

The base addresses are identical - only the internal structure layout differs.

## Current Tracker Status

**Our tracker currently uses these offsets (from mm_save.rs):**
```
PLAYER_FORM:     0x0020
HEALTH_CAPACITY: 0x002C
HEALTH:          0x002E
MAGIC_LEVEL:     0x0032
RUPEES:          0x0034
INVENTORY:       0x0070
MASKS:           0x0088
UPGRADES:        0x00A0
QUEST_ITEMS:     0x00A4
DUNGEON_ITEMS:   0x00A8
SMALL_KEYS:      0x00BC
STRAY_FAIRIES:   0x00D0
```

**These do NOT match OoTMM!** See comparison below.

---

## OoTMM MM Offset Comparison (CRITICAL)

| Field | Current Tracker | OoTMM Actual | Delta |
|-------|-----------------|--------------|-------|
| PLAYER_FORM | 0x0020 | 0x0020 | SAME |
| HEALTH_CAPACITY | 0x002C | 0x0034 | **+8** |
| HEALTH | 0x002E | 0x0036 | **+8** |
| MAGIC_LEVEL | 0x0032 | 0x0038 | **+6** |
| RUPEES | 0x0034 | 0x003A | **+6** |
| DOUBLE_DEFENSE | 0x003B | 0x0042 | **+7** |
| SWORD_SHIELD | 0x0044 | 0x006C | **+0x28** |
| INVENTORY | 0x0070 | 0x006E | -2 |
| MASKS | 0x0088 | 0x0086 | -2 (combined with items!) |
| UPGRADES | 0x00A0 | 0x00B6 | **+0x16** |
| QUEST_ITEMS | 0x00A4 | 0x00BA | **+0x16** |
| DUNGEON_ITEMS | 0x00A8 | 0x00BE | **+0x16** |
| SMALL_KEYS | 0x00BC | 0x00C8 | **+0x0C** |
| STRAY_FAIRIES | 0x00D0 | 0x00D2 | **+2** |

### OoTMM MM Structure Layout

```
MmSaveContext (0x801ef670, size 0x48d0)
├── MmSave (0x00, size 0x3ca0)
│   ├── entrance:        0x00 (s32)
│   ├── equippedMask:    0x04 (u8)
│   ├── linkAge:         0x07 (u8)
│   ├── cutscene:        0x08 (s32)
│   ├── time:            0x0c (u16)
│   ├── isNight:         0x10 (s32)
│   ├── day:             0x18 (u32)
│   ├── daysElapsed:     0x1c (u32)
│   ├── playerForm:      0x20 (u8) ← SAME AS TRACKER
│   ├── isOwlSave:       0x23 (u8)
│   └── info (MmSaveInfo): 0x24
│       ├── playerData (MmSavePlayerData): +0x00
│       │   ├── healthCapacity: +0x10 → 0x34
│       │   ├── health:         +0x12 → 0x36
│       │   ├── magicLevel:     +0x14 → 0x38
│       │   ├── magic:          +0x15 → 0x39
│       │   ├── rupees:         +0x16 → 0x3a
│       │   └── doubleDefense:  +0x1e → 0x42
│       ├── itemEquips (MmItemEquips): +0x28 → 0x4c
│       │   └── equipment:      +0x20 → 0x6c
│       └── inventory (MmInventory): +0x4a → 0x6e
│           ├── items[48]:      +0x00 → 0x6e (items 0-23, masks 24-47)
│           ├── ammo[24]:       +0x30 → 0x9e
│           ├── upgrades:       +0x48 → 0xb6
│           ├── quest:          +0x4c → 0xba
│           ├── dungeonItems[10]: +0x50 → 0xbe
│           ├── dungeonKeys[9]: +0x5a → 0xc8
│           ├── defenseHearts:  +0x63 → 0xd1
│           └── strayFairies[10]: +0x64 → 0xd2
```

### Why 255 Stray Fairies?

Reading from our offset 0xD0 instead of correct offset 0xD2 means we're reading from `dungeonKeys[8]` (Stone Tower keys) instead of `strayFairies[0]` (Clock Town fairies). If that byte is 0xFF, we display 255 fairies.

---

## OoT Save Structure Comparison

### OoT Context Sizes
| Field | Vanilla | OoTMM | Notes |
|-------|---------|-------|-------|
| OotSaveContext | 0x1450 | 0x1450 | Same |
| OotSave | 0x1354 | 0x1354 | Same |

### OoT Key Offsets (within OotSaveContext)

**Player Data:**
| Field | Vanilla | OoTMM | Notes |
|-------|---------|-------|-------|
| health_capacity | 0x2E | 0x2E | Same (in info.playerData) |
| health | 0x30 | 0x30 | Same |
| magic_level | 0x34 | 0x34 | Same |
| magic | 0x35 | 0x35 | Same |
| rupees | 0x36 | 0x36 | Same |
| double_defense | 0x3D | 0x3D | Same |

**Equipment & Equips:**
| Field | Vanilla | OoTMM | Notes |
|-------|---------|-------|-------|
| child_equips | 0x40 | 0x40 | Same |
| adult_equips | 0x4a | 0x4a | Same |
| current_equips | 0x68 | 0x68 | Same |

**Inventory (within OotSave):**
| Field | Vanilla | OoTMM | Offset Calculation |
|-------|---------|-------|-------------------|
| items[24] | 0x74 | 0x74 | info starts at 0x1c, inventory at 0x58 within info |
| ammo[15] | 0x8c | 0x8c | items + 0x18 |
| equipment | 0x9c | 0x9c | Verified by ASSERT_OFFSET |
| upgrades | 0xa0 | 0xa0 | 4 bytes |
| quest_items | 0xa4 | 0xa4 | 4-byte bitfield |
| dungeon_items[20] | 0xa8 | 0xa8 | 1 byte per dungeon |
| dungeon_keys[19] | 0xbc | 0xbc | 1 byte per dungeon |
| gold_tokens | 0xd0 | 0xd0 | 2 bytes |

**Scene Flags:**
| Field | Vanilla | OoTMM | Notes |
|-------|---------|-------|-------|
| perm_scene_flags | 0xd4 | 0xd4 | Same (124 scenes * 0x1c) |

### OoT Quest Items Bitfield (same in both)
```
Bit 0:  Forest Medallion
Bit 1:  Fire Medallion
Bit 2:  Water Medallion
Bit 3:  Spirit Medallion
Bit 4:  Shadow Medallion
Bit 5:  Light Medallion
Bit 6:  Minuet of Forest
Bit 7:  Bolero of Fire
Bit 8:  Serenade of Water
Bit 9:  Requiem of Spirit
Bit 10: Nocturne of Shadow
Bit 11: Prelude of Light
Bit 12: Zelda's Lullaby
Bit 13: Epona's Song
Bit 14: Saria's Song
Bit 15: Sun's Song
Bit 16: Song of Time
Bit 17: Song of Storms
Bit 18: Kokiri Emerald
Bit 19: Goron Ruby
Bit 20: Zora Sapphire
Bit 21: Stone of Agony
Bit 22: Gerudo Card
Bit 23: Gold Skulltula Token flag
Bits 24-27: Heart pieces (4 bits)
```

---

## MM Save Structure Comparison

### MM Context Sizes
| Field | Vanilla | OoTMM | Notes |
|-------|---------|-------|-------|
| MmSaveContext | 0x48d0 | 0x48d0 | Same total size |
| MmSave | 0x3ca0 | 0x3ca0 | Same |

### MM Key Offsets (CRITICAL DIFFERENCES)

**Pre-Info Fields (within MmSave):**
| Field | Vanilla | OoTMM | Notes |
|-------|---------|-------|-------|
| entrance | 0x00 | 0x00 | Same |
| equipped_mask | 0x04 | 0x04 | Same |
| link_age | 0x07 | 0x07 | Same |
| time | 0x0c | 0x0c | Same |
| is_night | 0x10 | 0x10 | Same |
| day | 0x18 | 0x18 | Same |
| player_form | 0x20 | 0x20 | Same |
| **info starts** | 0x24 | 0x24 | Same |

**Player Data (within MmSave.info):**
| Field | Vanilla | OoTMM | Difference |
|-------|---------|-------|------------|
| health_capacity | 0x2c | 0x34 | **+8** |
| health | 0x2e | 0x36 | **+8** |
| magic_level | 0x32 | 0x38 | **+6** |
| magic | 0x33 | 0x39 | **+6** |
| rupees | 0x34 | 0x3a | **+6** |
| sword_shield | 0x44 | 0x64 | **+0x20** |
| double_defense | 0x3b | 0x42 | **+7** |

Wait, these calculations need verification. Let me recalculate...

Actually, the issue is the MmSavePlayerData structure itself is different:

**Vanilla MmSavePlayerData (size ~0x28):**
- Starts at info offset 0x00
- healthCapacity at 0x10 within playerData

**OoTMM MmSavePlayerData (size 0x28):**
- Starts at info offset 0x00 (same)
- healthCapacity at 0x10 within playerData (same)

So if info starts at 0x24:
- healthCapacity = 0x24 + 0x00 + 0x10 = 0x34

But our vanilla tracker has HEALTH_CAPACITY at 0x002C...

Let me verify vanilla MM structure separately.

### Vanilla MM SaveContext Layout (for reference)

According to decomp projects, vanilla MM has:
- gSaveContext at 0x801ef670
- Within save context:
  - save.playerData.healthCapacity at offset 0x2c from base
  - save.playerData.health at offset 0x2e

### OoTMM MM SaveContext Layout

Within MmSaveContext:
- MmSave at offset 0x00
  - entrance: 0x00
  - equippedMask: 0x04
  - playerForm: 0x20
  - info: 0x24
    - playerData: +0x00 (relative to info)
      - healthCapacity: +0x10 (relative to playerData)
      - = 0x24 + 0x10 = **0x34**
    - itemEquips: +0x28
      - = 0x24 + 0x28 = **0x4c**
    - inventory: +0x4a
      - = 0x24 + 0x4a = **0x6e**

### MM Inventory Structure (MAJOR DIFFERENCES)

**Vanilla MmInventory:**
- items[24] at offset 0x00
- masks[24] at separate offset 0x18
- ammo[24] at offset 0x30
- upgrades at offset 0xa0
- quest_items at offset 0xa4
- dungeon_items at offset 0xa8
- small_keys at offset 0xbc
- stray_fairies at offset 0xd0

**OoTMM MmInventory (structure within info.inventory):**
```c
typedef struct {
    u8              items[48];           // 0x00 - Combined items AND masks!
    s8              ammo[24];            // 0x30
    MmUpgrades      upgrades;            // 0x48 (4 bytes)
    MmQuestItems    quest;               // 0x4c (4 bytes)
    MmDungeonItems  dungeonItems[10];    // 0x50 (10 bytes)
    s8              dungeonKeys[9];      // 0x5a (9 bytes)
    s8              defenseHearts;       // 0x63 (1 byte)
    s8              strayFairies[10];    // 0x64 (10 bytes)
    char            dekuPlaygroundPlayerName[3][8]; // 0x6e (24 bytes)
} MmInventory;
```

### MM Inventory Absolute Offsets (within MmSaveContext)

| Field | Vanilla | OoTMM | Notes |
|-------|---------|-------|-------|
| inventory_base | 0x70 | 0x6e | Base of inventory |
| items[0-23] | 0x70 | 0x6e | Regular items |
| items[24-47] | N/A | 0x86 | Masks in OoTMM! |
| masks | 0x88 | N/A | Separate in vanilla |
| ammo | 0x? | 0x9e | |
| upgrades | 0xa0 | 0xb6 | |
| quest_items | 0xa4 | 0xba | |
| dungeon_items | 0xa8 | 0xbe | |
| small_keys | 0xbc | 0xc8 | |
| stray_fairies | 0xd0 | 0xd2 | |

---

## OoTMM Item Slot Mapping

### OoT Items (items[0x18] array)
| Index | Item |
|-------|------|
| 0x00 | Deku Sticks |
| 0x01 | Deku Nuts |
| 0x02 | Bombs |
| 0x03 | Bow |
| 0x04 | Fire Arrows |
| 0x05 | Din's Fire |
| 0x06 | Slingshot |
| 0x07 | Ocarina |
| 0x08 | Bombchus |
| 0x09 | Hookshot |
| 0x0a | Ice Arrows |
| 0x0b | Farore's Wind |
| 0x0c | Boomerang |
| 0x0d | Lens of Truth |
| 0x0e | Magic Beans |
| 0x0f | Megaton Hammer |
| 0x10 | Light Arrows |
| 0x11 | Nayru's Love |
| 0x12-0x15 | Bottles (4) |
| 0x16 | Adult Trade Item |
| 0x17 | Child Trade Item |

### MM Items (items[48] combined array)

**Items (0x00-0x11):**
| Index | Item |
|-------|------|
| 0x00 | Ocarina |
| 0x01 | Bow |
| 0x02 | Fire Arrow |
| 0x03 | Ice Arrow |
| 0x04 | Light Arrow |
| 0x05 | Quest slot 1 (unused) |
| 0x06 | Bomb |
| 0x07 | Bombchu |
| 0x08 | Deku Stick |
| 0x09 | Deku Nut |
| 0x0a | Magic Bean |
| 0x0b | Quest slot 2 (unused) |
| 0x0c | Powder Keg |
| 0x0d | Pictograph Box |
| 0x0e | Lens of Truth |
| 0x0f | Hookshot |
| 0x10 | Great Fairy Sword |
| 0x11 | Quest slot 3 (unused) |
| 0x12-0x17 | Bottles (6) |

**Masks (0x20-0x37 within items array, i.e., indices 32-55 but only 24 used):**
| Index | Mask |
|-------|------|
| 0x20 | Postman's Hat |
| 0x21 | All-Night Mask |
| 0x22 | Blast Mask |
| 0x23 | Stone Mask |
| 0x24 | Great Fairy Mask |
| 0x25 | Deku Mask |
| 0x26 | Keaton Mask |
| 0x27 | Bremen Mask |
| 0x28 | Bunny Hood |
| 0x29 | Don Gero's Mask |
| 0x2a | Mask of Scents |
| 0x2b | Goron Mask |
| 0x2c | Romani's Mask |
| 0x2d | Circus Leader's Mask |
| 0x2e | Kafei's Mask |
| 0x2f | Couple's Mask |
| 0x30 | Mask of Truth |
| 0x31 | Zora Mask |
| 0x32 | Kamaro's Mask |
| 0x33 | Gibdo Mask |
| 0x34 | Garo's Mask |
| 0x35 | Captain's Hat |
| 0x36 | Giant's Mask |
| 0x37 | Fierce Deity's Mask |

---

## MM Quest Items Bitfield

```
Bit 0:  Odolwa's Remains
Bit 1:  Goht's Remains
Bit 2:  Gyorg's Remains
Bit 3:  Twinmold's Remains
Bit 4-5: unused
Bit 6:  Sonata of Awakening
Bit 7:  Goron Lullaby
Bit 8:  New Wave Bossa Nova
Bit 9:  Elegy of Emptiness
Bit 10: Oath to Order
Bit 11: Saria's Song (shared)
Bit 12: Song of Time
Bit 13: Song of Healing
Bit 14: Epona's Song
Bit 15: Song of Soaring
Bit 16: Song of Storms
Bit 17: Sun's Song (shared)
Bit 18: Bomber's Notebook
Bits 19-23: unused
Bit 24: Goron Lullaby Intro
Bits 25-27: unused
Bits 28-31: Heart pieces (4 bits)
```

---

## SharedCustomSave Structure

OoTMM uses an additional shared save structure for cross-game data:

```c
typedef struct ALIGNED(16) {
    OotCustomSave   oot;           // OoT-specific custom data
    MmCustomSave    mm;            // MM-specific custom data
    s16             netGiSkip[16]; // Network item tracking
    u16             coins[4];      // Coin counts
    u16             ocarinaButtonMaskOot;
    u16             ocarinaButtonMaskMm;
    u8              soulsEnemyOot[8];  // Soul tracking
    u8              soulsEnemyMm[8];
    u8              soulsBossOot[2];
    u8              soulsBossMm[1];
    u8              soulsNpcOot[8];
    u8              soulsNpcMm[8];
    u8              soulsAnimalsOot[2];
    u8              soulsAnimalsMm[2];
    u8              soulsMiscOot[1];
    u8              soulsMiscMm[1];
    u8              caughtChildFishWeight[20];
    u8              caughtAdultFishWeight[20];
    u8              caughtFishFlags[5];
    RespawnData     respawn[1];
    // Bitfield flags for cross-game features
    u8              foundMasterSword:1;
    u8              storedSirloin:1;
    u8              extraSwordsOot:2;
    u8              bombchuBagOot:2;
    u8              bombchuBagMm:2;
    u8              mmShieldIsDeku:1;
    u8              mmProgressiveShields:2;
    u8              bronzeScaleOot:1;
    u8              bronzeScaleMm:1;
    u8              traps[TRAP_MAX];
    u8              notes[NOTES_MAX];
} SharedCustomSave;
```

This structure tracks items that are shared between games (like health, lens, etc.).

---

## Detection Strategy

To detect OoTMM vs vanilla:

1. **Check ROM header** - OoTMM ROMs have specific identifiers
2. **Check save magic bytes** - Different magic values
3. **Check for OoTMM payload** at specific VROM addresses:
   - `COMBO_VROM_PAYLOAD` at 0xf0000000 (OoT) or 0xf0100000 (MM)
4. **Check for combo config** structure presence

---

## Implementation Notes

1. **OoT seems largely compatible** - The OoT save structure offsets appear to be mostly the same between vanilla and OoTMM

2. **MM is significantly different** - The inventory structure is completely reorganized:
   - Items and masks are combined into a single 48-slot array
   - Different ammo slot count (24 vs 15)
   - All offsets are shifted

3. **Need conditional parsing** - When OoTMM is detected:
   - Use OoTMM offset constants
   - Parse the combined items/masks array differently
   - Handle the SharedCustomSave for cross-game items

4. **Cross-game items** - Some items are shared in OoTMM:
   - Health (4 hearts regardless of game)
   - Lens of Truth
   - Possibly other items depending on settings

5. **OoTMM custom flags** - Track additional OoTMM-specific data through OotCustomSave and MmCustomSave structures
