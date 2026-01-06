# Feature Decomposition: Combo Layout Improvements (#467)

## Analysis Summary

**Decision: DECOMPOSE**

Issue #467 should be broken into 5 sub-issues because:

1. **Multiple distinct concerns**: UI organization, responsive design, configuration, and custom features
2. **Varying complexity**: Ranges from ~2 hours (reorganization) to multiple days (custom layouts)
3. **Different file areas**: `ui.rs`, `main.rs`, and potential new config structures
4. **Independent value**: Each improvement provides standalone user benefit
5. **Natural dependency ordering**: Some improvements enable others

## Current State

The Combo layout is defined in `/crate/oottracker/src/ui.rs:4141-4249`:
- Fixed 12-column grid using `columns!` macro
- 84 items across 7 rows (720x480 pixels)
- Hardcoded item positions
- No customization options

```rust
Self::Combo => {
    columns!(
        12,
        [
            // Row 1: OoT Dungeon Rewards + MM Boss Remains
            ForestMedallion, FireMedallion, WaterMedallion, ...
            // ... 84 total items
        ]
    )
}
```

---

## Sub-Issue 1: Reorganize Items by Category

**Priority**: High (foundational for other improvements)
**Complexity**: Low (~2-4 hours)
**Dependencies**: None

### Task
Reorganize items in `TrackerLayout::Combo` to group related items together logically.

### Proposed Organization
```
Row 1: OoT Medallions (6)
Row 2: OoT Stones (3) + MM Boss Remains (4) + spacer
Row 3: Transformation Masks (4) + Core shared items
Row 4: OoT Equipment (12)
Row 5: MM Equipment (12)
Row 6: OoT Songs - Ocarina (6) + Warp (6)
Row 7: MM Songs (12)
Row 8: Trade items / Misc from both games
```

### Files
- `/crate/oottracker/src/ui.rs` - Reorder items in `TrackerLayout::Combo::cells()` match arm

### Tests
- Visual inspection that items are logically grouped
- Verify layout still renders correctly

---

## Sub-Issue 2: Visual Separation Between OoT and MM Sections

**Priority**: Medium
**Complexity**: Low-Medium (~4-8 hours)
**Dependencies**: Benefits from Sub-Issue 1

### Task
Add visual cues to distinguish OoT items from MM items.

### Approach Options
1. Row-based separation (empty row or horizontal line)
2. Color coding (subtle background: OoT gold, MM purple)
3. Section headers (small text labels)
4. Border/frame around each game's items

### Implementation Notes
- May require changes to `CellLayout` struct for section metadata
- Or special "separator" cell types in `TrackerCellId`
- GUI rendering in `main.rs` handles visual distinction

### Files
- `/crate/oottracker/src/ui.rs` - Layout structure
- `/crate/oottracker-gui/src/main.rs` - Rendering

---

## Sub-Issue 3: Responsive Sizing

**Priority**: Medium
**Complexity**: Medium (~1-2 days)
**Dependencies**: None (can parallel)

### Task
Make Combo layout adapt to window size instead of fixed 720x480.

### Current State
```rust
// CellLayout uses fixed u16 positions
CellLayout { idx, id, pos: [col * 60 + 5, row * 60 + 5], size: [50, 50] }
```

### Approach Options
1. **Proportional scaling**: Scale cell sizes based on window dimensions
2. **Breakpoint layouts**: Different column counts at different widths (12 -> 8 -> 6)
3. **Flex-like behavior**: Let iced handle layout with constraints

### Implementation Notes
- `CellLayout::pos` and `size` use fixed `u16` values
- `columns!` macro hardcodes positioning
- May need to pass window dimensions to `TrackerLayout::cells()`
- Or calculate positions at render time in `main.rs`

### Files
- `/crate/oottracker/src/ui.rs` - `CellLayout`, `columns!` macro
- `/crate/oottracker-gui/src/main.rs` - Window size handling

### Tests
- Test at 720x480, 1280x720, 1920x1080, smaller sizes
- Verify click targets remain accurate
- Ensure icons remain readable

---

## Sub-Issue 4: Item Category Filtering

**Priority**: Low-Medium
**Complexity**: Medium (~1-2 days)
**Dependencies**: Sub-Issue 1 (organization makes filtering logical)

### Task
Allow users to show/hide specific item categories.

### Categories
- [ ] OoT Dungeon Rewards (medallions + stones)
- [ ] MM Boss Remains
- [ ] Transformation Masks
- [ ] OoT Equipment
- [ ] MM Equipment
- [ ] OoT Songs
- [ ] MM Songs
- [ ] Trade Items / Misc

### Implementation

1. **Config additions**:
```rust
#[derive(Default, Serialize, Deserialize)]
pub struct ComboLayoutFilters {
    pub show_oot_rewards: bool,
    pub show_mm_remains: bool,
    pub show_masks: bool,
    // ...
}
```

2. **UI**: Checkboxes in settings menu
3. **Layout**: Filter items in `cells()` based on config
4. **Persistence**: Save with config

### Files
- `/crate/oottracker/src/ui.rs` - Filter config, filtered layout
- `/crate/oottracker-gui/src/main.rs` - Settings UI

---

## Sub-Issue 5: Custom User-Defined Layouts

**Priority**: Low (enhancement)
**Complexity**: High (~1-2 weeks)
**Dependencies**: None, but benefits from Sub-Issue 3

### Task
Allow users to define their own combo layouts.

### Feature Scope
1. **Layout schema**: JSON/TOML definition format
2. **Layout editor UI**: Drag-and-drop or form-based
3. **Import/Export**: Share layouts
4. **Presets**: Bundled alternative layouts

### Implementation Considerations

```rust
// New variant
pub enum TrackerLayout {
    // ... existing variants
    Custom(CustomLayout),
}

#[derive(Serialize, Deserialize)]
pub struct CustomLayout {
    pub name: String,
    pub columns: u16,
    pub cells: Vec<CustomCellDef>,
}

#[derive(Serialize, Deserialize)]
pub struct CustomCellDef {
    pub id: TrackerCellId,
    pub row: u16,
    pub col: u16,
    pub width: Option<u16>,  // defaults to 1
    pub height: Option<u16>, // defaults to 1
}
```

### Files
- `/crate/oottracker/src/ui.rs` - `CustomLayout` type, validation
- `/crate/oottracker-gui/src/main.rs` - Layout editor UI (significant)
- New layout file schema

---

## Implementation Order

```
                    +------------------+
                    | #1 Reorganize    |
                    | (no deps)        |
                    +--------+---------+
                             |
              +--------------+--------------+
              |                             |
              v                             v
    +------------------+          +------------------+
    | #2 Visual Sep    |          | #4 Filtering     |
    | (benefits #1)    |          | (needs #1)       |
    +------------------+          +------------------+

    +------------------+          +------------------+
    | #3 Responsive    |          | #5 Custom        |
    | (independent)    |          | (independent)    |
    +------------------+          +------------------+
```

### Recommended Sequence
1. **Sub-Issue 1** (Reorganization) - Foundation, quick win
2. **Sub-Issue 2** (Visual Separation) - Builds on #1
3. **Sub-Issue 3** (Responsive) - Can parallel with #2
4. **Sub-Issue 4** (Filtering) - After #1 complete
5. **Sub-Issue 5** (Custom) - Major feature, later phase

### Parallel Opportunities
- Sub-Issues 3 and 5 can be developed in parallel with others
- Sub-Issues 2 and 4 both depend on 1 but not each other

---

## Related Issues
- #461 - Combo layout should display both OoT and MM items (base functionality, prerequisite)
- #467 - Parent tracking issue

---

## Commands to Create Sub-Issues

When ready to create the actual GitHub issues:

```bash
# Sub-issue 1: Reorganization
gh issue create --repo spencerduncan/oottracker \
  --title "[Combo Layout] Reorganize items by category" \
  --body "Part of #467. [body from above]"

# Sub-issue 2: Visual separation
gh issue create --repo spencerduncan/oottracker \
  --title "[Combo Layout] Add visual separation between OoT and MM sections" \
  --body "Part of #467. [body from above]"

# Sub-issue 3: Responsive sizing
gh issue create --repo spencerduncan/oottracker \
  --title "[Combo Layout] Responsive sizing to adapt to window size" \
  --body "Part of #467. [body from above]"

# Sub-issue 4: Item filtering
gh issue create --repo spencerduncan/oottracker \
  --title "[Combo Layout] Item category filtering (show/hide)" \
  --body "Part of #467. [body from above]"

# Sub-issue 5: Custom layouts
gh issue create --repo spencerduncan/oottracker \
  --title "[Combo Layout] Custom user-defined layouts" \
  --body "Part of #467. [body from above]"
```

After creating, set up blocked-by relationships:
```bash
# Get node IDs
gh api graphql -f query='query { repository(owner: "spencerduncan", name: "oottracker") {
  issues(first: 10, orderBy: {field: CREATED_AT, direction: DESC}) {
    nodes { id number title }
  }
}}'

# Link sub-issues to block parent #467
gh api graphql -f query='mutation { addBlockedBy(input: {
  issueId: "<#467_node_id>",
  blockingIssueId: "<sub_issue_node_id>"
}) { issue { number } } }'
```
