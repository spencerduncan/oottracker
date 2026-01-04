#!/usr/bin/env python3
"""Convert OoTMM world data to oottracker YAML format."""

import os
import re
import yaml
from pathlib import Path


def to_snake_case(name: str) -> str:
    """Convert a name to snake_case ID."""
    # Remove special characters and replace spaces/dashes with underscores
    s = re.sub(r"['\"]", "", name)
    s = re.sub(r"[^a-zA-Z0-9]+", "_", s)
    s = re.sub(r"_+", "_", s)
    s = s.strip("_").lower()
    return s


def guess_location_type(name: str) -> str:
    """Guess the location type from the location name."""
    name_lower = name.lower()

    if "gs " in name_lower or " gs" in name_lower or "skulltula" in name_lower:
        return "collectible"
    if " hp" in name_lower or "heart piece" in name_lower or "piece of heart" in name_lower:
        return "collectible"
    if "chest" in name_lower:
        return "chest"
    if "pot " in name_lower or " pot" in name_lower or name_lower.endswith(" pot"):
        return "freestanding"
    if "grass " in name_lower or " grass" in name_lower:
        return "freestanding"
    if "rupee" in name_lower:
        return "freestanding"
    if "heart " in name_lower and "piece" not in name_lower:
        return "freestanding"
    if "rock " in name_lower or " rock" in name_lower:
        return "freestanding"
    if "item " in name_lower or "shop item" in name_lower:
        return "shop"
    if "scrub" in name_lower:
        return "scrub"
    if "cow" in name_lower:
        return "cow"
    if "fairy" in name_lower or "great fairy" in name_lower:
        return "fairy"
    if "fish" in name_lower:
        return "fishing"
    if "gossip" in name_lower:
        return "gossipStone"
    if "song" in name_lower:
        return "song"
    if "boss" in name_lower or "reward" in name_lower:
        return "boss"
    if "soil" in name_lower:
        return "freestanding"
    if "wonder item" in name_lower:
        return "freestanding"
    if "hive" in name_lower or "butterfly" in name_lower:
        return "freestanding"
    if "small crate" in name_lower or "crate" in name_lower:
        return "freestanding"
    if "bush" in name_lower:
        return "freestanding"
    if "tree" in name_lower and ("forked" in name_lower or "reward" in name_lower):
        return "freestanding"
    if "mask" in name_lower:
        return "npc"
    if "keaton" in name_lower or "postman" in name_lower:
        return "npc"
    if "owl statue" in name_lower:
        return "event"
    if "map" in name_lower and "tingle" in name_lower:
        return "npc"
    if "initial" in name_lower:
        return "event"
    if "bomber" in name_lower or "notebook" in name_lower:
        return "npc"
    if "blast" in name_lower:
        return "npc"
    if "bank" in name_lower:
        return "npc"
    if "archery" in name_lower or "shooting" in name_lower:
        return "event"
    if "game" in name_lower:
        return "event"
    if "letter" in name_lower or "pendant" in name_lower:
        return "npc"
    if "bottle" in name_lower:
        return "npc"
    if "purchase" in name_lower:
        return "shop"
    if "deku playground" in name_lower:
        return "event"
    if "honey" in name_lower or "darling" in name_lower:
        return "event"
    if "grandma" in name_lower:
        return "npc"
    if "???" in name_lower:
        return "npc"

    # Default to npc for named rewards/items
    return "npc"


def guess_exit_type(source_area: str, target_area: str, logic: str) -> str:
    """Guess the exit type from source/target areas and logic."""
    source_lower = source_area.lower()
    target_lower = target_area.lower()

    # Check for dungeon entrances
    dungeons = [
        "deku tree", "dodongo", "jabu", "forest temple", "fire temple",
        "water temple", "shadow temple", "spirit temple", "ganon",
        "bottom of the well", "ice cavern", "gerudo training",
        "woodfall temple", "snowhead temple", "great bay temple",
        "stone tower temple", "ancient castle", "beneath the well",
        "pirate fortress", "ocean spider", "swamp spider", "secret shrine"
    ]

    for dungeon in dungeons:
        if dungeon in target_lower and dungeon not in source_lower:
            return "dungeon"

    if "grotto" in target_lower:
        return "grotto"
    if "house" in target_lower or "shop" in target_lower or "inn" in target_lower:
        return "interior"
    if "warp" in logic.lower() or "owl" in target_lower:
        return "warp"

    return "overworld"


def convert_file(input_path: Path, game: str, prefix: str = "") -> dict:
    """Convert a single OoTMM YAML file to our format.

    Args:
        input_path: Path to the input YAML file
        game: Game identifier (oot or mm)
        prefix: Optional prefix for region IDs (e.g., "mq_" for Master Quest)
    """
    with open(input_path) as f:
        data = yaml.safe_load(f)

    if not data:
        return {"regions": []}

    regions = []

    for area_name, area_data in data.items():
        if not isinstance(area_data, dict):
            continue

        # Prefix region ID with game (and any extra prefix like "mq_") for global uniqueness
        game_prefix = game + "_"
        region_id = prefix + game_prefix + to_snake_case(area_name)

        region = {
            "id": region_id,
            "name": area_name,
            "game": game,
        }

        # Convert locations
        locations = []
        if "locations" in area_data and area_data["locations"]:
            for loc_name, loc_logic in area_data["locations"].items():
                # Apply game prefix and any extra prefix to locations
                loc_id = prefix + game_prefix + to_snake_case(loc_name)
                loc_type = guess_location_type(loc_name)
                loc = {
                    "id": loc_id,
                    "name": loc_name,
                    "locationType": loc_type,
                }
                if loc_logic and loc_logic != "true":
                    loc["logic"] = loc_logic
                else:
                    loc["logic"] = "true"
                locations.append(loc)

        if locations:
            region["locations"] = locations

        # Convert exits
        exits = []
        if "exits" in area_data and area_data["exits"]:
            for target_name, exit_logic in area_data["exits"].items():
                # Skip special exits that reference macros
                if target_name.startswith("GENERIC_") or target_name == "GLOBAL":
                    continue

                # Apply game prefix and any extra prefix to exits
                target_id = prefix + game_prefix + to_snake_case(target_name)
                exit_type = guess_exit_type(area_name, target_name, exit_logic or "")
                exit_data = {
                    "target": target_id,
                    "exitType": exit_type,
                }
                if exit_logic and exit_logic != "true":
                    exit_data["logic"] = exit_logic
                exits.append(exit_data)

        if exits:
            region["exits"] = exits

        # Convert events
        events = []
        if "events" in area_data and area_data["events"]:
            for event_name, event_logic in area_data["events"].items():
                # Make event names more readable
                readable_name = event_name.replace("_", " ").title()
                # Prefix event IDs with game (and any region prefix) for global uniqueness
                game_prefix = game.upper() + "_"
                event_id = prefix + game_prefix + event_name
                event_data = {
                    "id": event_id,
                    "name": readable_name,
                }
                if event_logic and event_logic != "true":
                    event_data["logic"] = event_logic
                events.append(event_data)

        if events:
            region["events"] = events

        regions.append(region)

    return {"regions": regions}


def convert_directory(input_dir: Path, output_path: Path, game: str, category: str, seen_events: set, seen_locations: set, prefix: str = ""):
    """Convert all YAML files in a directory to a single output file.

    Args:
        input_dir: Directory containing YAML files
        output_path: Output file path
        game: Game identifier (oot or mm)
        category: Category name for the file comment
        seen_events: Set of event IDs already seen (to avoid duplicates)
        seen_locations: Set of location IDs already seen (to avoid duplicates)
        prefix: Optional prefix for region/location IDs (e.g., "mq_" for Master Quest)
    """
    all_regions = []

    yaml_files = sorted(input_dir.glob("*.yml"))
    for yaml_file in yaml_files:
        # Skip system files
        if yaml_file.name.startswith("_"):
            continue

        result = convert_file(yaml_file, game, prefix)

        # Deduplicate events and locations across regions
        for region in result.get("regions", []):
            # Deduplicate events
            if "events" in region:
                unique_events = []
                for event in region["events"]:
                    if event["id"] not in seen_events:
                        seen_events.add(event["id"])
                        unique_events.append(event)
                if unique_events:
                    region["events"] = unique_events
                else:
                    del region["events"]

            # Deduplicate locations
            if "locations" in region:
                unique_locations = []
                for loc in region["locations"]:
                    if loc["id"] not in seen_locations:
                        seen_locations.add(loc["id"])
                        unique_locations.append(loc)
                if unique_locations:
                    region["locations"] = unique_locations
                else:
                    del region["locations"]

            all_regions.append(region)

    if all_regions:
        output_data = {"regions": all_regions}
        output_path.parent.mkdir(parents=True, exist_ok=True)

        with open(output_path, "w") as f:
            f.write(f"# {game.upper()} {category.replace('_', ' ').title()} Regions\n")
            f.write(f"# Imported from OoTMM randomizer project\n\n")
            yaml.dump(output_data, f, default_flow_style=False, allow_unicode=True, sort_keys=False, width=120)

        print(f"Created {output_path} with {len(all_regions)} regions")


def main():
    ootmm_base = Path("/tmp/ootmm-source/packages/data/src/world")
    output_base = Path("/home/user/oottracker/crate/ootmm/data/world")

    # Remove existing files (we'll create fresh comprehensive data)
    for old_file in output_base.glob("*.yaml"):
        old_file.unlink()
        print(f"Removed old file: {old_file}")

    # Track seen events and locations to avoid duplicates across files
    # Events are global flags in the OoTMM randomizer, so we only need one definition
    # Locations may appear in multiple areas due to how OoTMM structures data
    oot_seen_events = set()
    oot_seen_locations = set()
    mm_seen_events = set()
    mm_seen_locations = set()

    # Convert OoT files
    oot_base = ootmm_base / "oot"
    convert_directory(oot_base / "overworld", output_base / "oot_overworld.yaml", "oot", "overworld", oot_seen_events, oot_seen_locations)
    convert_directory(oot_base / "dungeons", output_base / "oot_dungeons.yaml", "oot", "dungeons", oot_seen_events, oot_seen_locations)
    # Master Quest dungeons get "mq_" prefix to avoid ID collisions with regular dungeons
    convert_directory(oot_base / "dungeons_mq", output_base / "oot_dungeons_mq.yaml", "oot", "master_quest_dungeons", oot_seen_events, oot_seen_locations, prefix="mq_")
    convert_directory(oot_base / "boss", output_base / "oot_bosses.yaml", "oot", "boss_rooms", oot_seen_events, oot_seen_locations)

    # Convert MM files
    mm_base = ootmm_base / "mm"
    convert_directory(mm_base / "overworld", output_base / "mm_overworld.yaml", "mm", "overworld", mm_seen_events, mm_seen_locations)
    convert_directory(mm_base / "dungeons", output_base / "mm_dungeons.yaml", "mm", "dungeons", mm_seen_events, mm_seen_locations)
    convert_directory(mm_base / "boss", output_base / "mm_bosses.yaml", "mm", "boss_rooms", mm_seen_events, mm_seen_locations)

    # Convert US-specific MM files (skip JP variant for now)
    us_path = mm_base / "us"
    if us_path.exists():
        convert_directory(us_path, output_base / "mm_us_variant.yaml", "mm", "us_variant", mm_seen_events, mm_seen_locations)

    print(f"\nTotal OoT events: {len(oot_seen_events)}, locations: {len(oot_seen_locations)}")
    print(f"Total MM events: {len(mm_seen_events)}, locations: {len(mm_seen_locations)}")


if __name__ == "__main__":
    main()
