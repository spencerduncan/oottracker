#!/bin/bash
# Download and organize tracker icons from EmoTracker packs
# Sources:
#   OoT: https://github.com/Hamsda/EmoTrackerPacks
#   MM:  https://github.com/jupiter0fire/OoTMMR_tracker_pack

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
ASSETS_DIR="$PROJECT_ROOT/assets/img"

# Temp directories for downloads
TMP_DIR=$(mktemp -d)
trap "rm -rf $TMP_DIR" EXIT

echo "=== Downloading EmoTracker Icon Packs ==="
echo "Temp directory: $TMP_DIR"

# Download Hamsda OoT pack
echo ""
echo ">>> Downloading Hamsda OoT pack..."
curl -sL "https://github.com/Hamsda/EmoTrackerPacks/archive/refs/heads/master.zip" -o "$TMP_DIR/hamsda.zip"
unzip -q "$TMP_DIR/hamsda.zip" -d "$TMP_DIR"
HAMSDA_IMAGES="$TMP_DIR/EmoTrackerPacks-master/ootrando_overworldmap_hamsda/images"

# Download jupiter0fire OoTMM pack (has MM items)
echo ">>> Downloading jupiter0fire OoTMM pack..."
curl -sL "https://github.com/jupiter0fire/OoTMMR_tracker_pack/archive/refs/heads/develop.zip" -o "$TMP_DIR/jupiter.zip"
unzip -q "$TMP_DIR/jupiter.zip" -d "$TMP_DIR"
JUPITER_IMAGES="$TMP_DIR/OoTMMR_tracker_pack-develop/images/items"

# Create MM image directories
echo ""
echo ">>> Creating MM image directories..."
mkdir -p "$ASSETS_DIR/mm-images"
mkdir -p "$ASSETS_DIR/mm-images-dimmed"

# ============================================================
# OoT Image Mapping (Hamsda -> xopar-images naming)
# ============================================================
echo ""
echo ">>> Copying OoT images from Hamsda pack..."

declare -A OOT_RENAME=(
    # Equipment
    ["bomb.png"]="bomb_bag.png"
    ["bow.png"]="bow.png"
    ["boomerang.png"]="boomerang.png"
    ["hookshot.png"]="hookshot.png"
    ["longshot.png"]="longshot.png"
    ["lens.png"]="lens.png"
    ["hammer.png"]="hammer.png"
    ["slingshot.png"]="slingshot.png"
    ["ocarina.png"]="ocarina.png"
    ["beans.png"]="beans.png"
    ["bottle.png"]="bottle.png"

    # Upgrades
    ["scale.png"]="gold_scale.png"
    ["strength.png"]="gold_gauntlets.png"
    ["wallet.png"]="wallet.png"

    # Songs
    ["lullaby.png"]="lullaby.png"
    ["epona.png"]="epona.png"
    ["saria.png"]="saria.png"
    ["sun.png"]="sun.png"
    ["time.png"]="time.png"
    ["storms.png"]="storms.png"
    ["minuet.png"]="minuet.png"
    ["bolero.png"]="bolero.png"
    ["serenade.png"]="serenade.png"
    ["nocturne.png"]="nocturne.png"
    ["requiem.png"]="requiem.png"
    ["prelude.png"]="prelude.png"

    # Medallions
    ["forest_medallion.png"]="forest_medallion.png"
    ["fire_medallion.png"]="fire_medallion.png"
    ["water_medallion.png"]="water_medallion.png"
    ["spirit_medallion.png"]="spirit_medallion.png"
    ["shadow_medallion.png"]="shadow_medallion.png"
    ["light_medallion.png"]="light_medallion.png"

    # Stones
    ["kokiri_emerald.png"]="kokiri_emerald.png"
    ["goron_ruby.png"]="goron_ruby.png"
    ["zora_sapphire.png"]="zora_sapphire.png"

    # Tunics/Boots
    ["redtunic.png"]="goron_tunic.png"
    ["bluetunic.png"]="zora_tunic.png"
    ["ironboots.png"]="iron_boots.png"
    ["hoverboots.png"]="hover_boots.png"

    # Spells
    ["din.png"]="dins_fire.png"
    ["farore.png"]="faores_wind.png"
    ["nayru.png"]="nayrus_love.png"

    # Child trade
    ["egg.png"]="blue_egg.png"
    ["cojiro.png"]="cojiro.png"
    ["mushroom.png"]="mushroom.png"
    ["saw.png"]="saw.png"
    ["sword_broken.png"]="broken_sword.png"
    ["prescription.png"]="prescription.png"
    ["frog.png"]="eyeball_frog.png"
    ["eyedrops.png"]="eye_drops.png"
    ["claim.png"]="claim_check.png"

    # Other
    ["skulltula.png"]="golden_skulltula.png"
    ["triforce.png"]="triforce.png"
    ["gerudo.png"]="gerudo_card.png"
    ["mirror.png"]="mirror_shield.png"
    ["fire_arrow.png"]="fire_arrows.png"
    ["ice_arrow.png"]="ice_arrows.png"
    ["light_arrow.png"]="light_arrows.png"
)

for src in "${!OOT_RENAME[@]}"; do
    dst="${OOT_RENAME[$src]}"
    if [ -f "$HAMSDA_IMAGES/$src" ]; then
        cp "$HAMSDA_IMAGES/$src" "$ASSETS_DIR/xopar-images/$dst"
        echo "  Copied $src -> xopar-images/$dst"
    fi
done

# ============================================================
# MM Image Mapping (jupiter0fire -> mm-images naming)
# ============================================================
echo ""
echo ">>> Copying MM images from jupiter0fire pack..."

declare -A MM_RENAME=(
    # Transformation Masks
    ["mm_deku.png"]="deku_mask.png"
    ["mm_goron.png"]="goron_mask.png"
    ["mm_zora.png"]="zora_mask.png"
    ["mm_fiercedeity.png"]="fierce_deity_mask.png"

    # Collectible Masks
    ["mm_postman.png"]="postman_hat.png"
    ["mm_allnight.png"]="all_night_mask.png"
    ["mm_blast.png"]="blast_mask.png"
    ["mm_stone.png"]="stone_mask.png"
    ["mm_greatfairy.png"]="great_fairy_mask.png"
    ["mm_keaton.png"]="keaton_mask.png"
    ["mm_bremen.png"]="bremen_mask.png"
    ["mm_bunny.png"]="bunny_hood.png"
    ["mm_dongero.png"]="don_gero_mask.png"
    ["mm_scents.png"]="mask_of_scents.png"
    ["mm_romanimask.png"]="romani_mask.png"
    ["mm_troupe.png"]="circus_leader_mask.png"
    ["mm_kafeimask.png"]="kafei_mask.png"
    ["mm_couple.png"]="couples_mask.png"
    ["mm_maskoftruth.png"]="mask_of_truth.png"
    ["mm_kamaro.png"]="kamaro_mask.png"
    ["mm_gibdo.png"]="gibdo_mask.png"
    ["mm_garo.png"]="garo_mask.png"
    ["mm_captain.png"]="captain_hat.png"
    ["mm_giant.png"]="giant_mask.png"

    # Boss Remains
    ["mm_odolwa.png"]="odolwa_remains.png"
    ["mm_goht.png"]="goht_remains.png"
    ["mm_gyorg.png"]="gyorg_remains.png"
    ["mm_twinmold.png"]="twinmold_remains.png"

    # Stray Fairies
    ["mm_clocktown_stray_fairy.png"]="stray_fairy_clock_town.png"
    ["mm_woodfall_stray_fairy.png"]="stray_fairy_woodfall.png"
    ["mm_snowhead_stray_fairy.png"]="stray_fairy_snowhead.png"
    ["mm_greatbay_stray_fairy.png"]="stray_fairy_great_bay.png"
    ["mm_stonetower_stray_fairy.png"]="stray_fairy_stone_tower.png"

    # Songs
    ["mm_songoftime.png"]="song_of_time.png"
    ["mm_healing.png"]="song_of_healing.png"
    ["mm_epona.png"]="eponas_song.png"
    ["mm_soaring.png"]="song_of_soaring.png"
    ["mm_songofstorms.png"]="song_of_storms.png"
    ["mm_sonata.png"]="sonata_of_awakening.png"
    ["mm_lullaby.png"]="goron_lullaby.png"
    ["mm_bossanova.png"]="new_wave_bossa_nova.png"
    ["mm_elegy.png"]="elegy_of_emptiness.png"
    ["mm_oath.png"]="oath_to_order.png"

    # Equipment
    ["mm_bomber.png"]="bomber_notebook.png"
    ["mm_ocarina.png"]="ocarina.png"
    ["mm_bow.png"]="heros_bow.png"
    ["mm_firearrow.png"]="fire_arrow.png"
    ["mm_icearrow.png"]="ice_arrow.png"
    ["mm_lightarrow.png"]="light_arrow.png"
    ["mm_hookshot.png"]="hookshot.png"
    ["mm_bomb.png"]="bombs.png"
    ["mm_bombchu.png"]="bombchu.png"
    ["mm_keg.png"]="powder_keg.png"
    ["mm_lens.png"]="lens_of_truth.png"
    ["mm_box.png"]="pictograph_box.png"
    ["mm_fairysword.png"]="great_fairy_sword.png"
    ["mm_bean.png"]="magic_bean.png"
    ["mm_magic1.png"]="magic.png"
    ["mm_magic2.png"]="double_magic.png"

    # Swords
    ["mm_kokiri.png"]="kokiri_sword.png"
    ["mm_razor.png"]="razor_sword.png"
    ["mm_gilded.png"]="gilded_sword.png"

    # Shield
    ["mm_shield.png"]="heros_shield.png"
    ["mm_mirror.png"]="mirror_shield.png"

    # Bottles & Wallet
    ["mm_bottle.png"]="bottle.png"
    ["mm_wallet.png"]="wallet.png"
    ["mm_giantwallet.png"]="giant_wallet.png"
)

for src in "${!MM_RENAME[@]}"; do
    dst="${MM_RENAME[$src]}"
    if [ -f "$JUPITER_IMAGES/$src" ]; then
        cp "$JUPITER_IMAGES/$src" "$ASSETS_DIR/mm-images/$dst"
        echo "  Copied $src -> mm-images/$dst"
    else
        echo "  WARNING: Missing $src"
    fi
done

# ============================================================
# Create dimmed versions
# ============================================================
echo ""
echo ">>> Creating dimmed versions of MM images..."

if command -v convert &> /dev/null; then
    for img in "$ASSETS_DIR/mm-images/"*.png; do
        if [ -f "$img" ]; then
            basename=$(basename "$img")
            convert "$img" -modulate 50,50,100 "$ASSETS_DIR/mm-images-dimmed/$basename"
        fi
    done
    echo "  Created dimmed versions using ImageMagick"
else
    echo "  WARNING: ImageMagick not installed, skipping dimmed versions"
    echo "  Install with: sudo apt install imagemagick"
fi

# ============================================================
# Summary
# ============================================================
echo ""
echo "=== Download Complete ==="
echo "OoT images: $(ls -1 "$ASSETS_DIR/xopar-images/"*.png 2>/dev/null | wc -l) files"
echo "MM images:  $(ls -1 "$ASSETS_DIR/mm-images/"*.png 2>/dev/null | wc -l) files"
echo "MM dimmed:  $(ls -1 "$ASSETS_DIR/mm-images-dimmed/"*.png 2>/dev/null | wc -l) files"
echo ""
echo "Note: Some images may need manual adjustment for style consistency."
echo "The script downloaded from community tracker packs (Hamsda, jupiter0fire)."
