---
id: TASK-010
assigned_to: worldbuild
priority: low
status: open
created_by: designer
created_date: 2026-03-29
related_finding: DESIGN-001
---

# Add Scout action definition to Watchtower location

## What
Update the North Watchtower location definition in `src/data.rs` to include a "Scout"
action. This is a content/data task — the mechanical implementation is in TASK-011.

For now, add the action as a defined option on the location so it appears in the
interaction panel. The description should be evocative:

> "From this height, you can see who moves through the open streets below."

## Design Spec (from designer)
- Action name: "Scout" (or "Survey" — worldbuild can choose what fits the tone)
- Available only at the Watchtower
- Reveals current location of NPCs in outdoor/transit locations
- Does NOT reveal what they're doing — just where
- Flavor: Brega may comment on what you observe (optional, gives her a voice)

## Why
Addresses DESIGN-001: Watchtower currently has no distinct value. The scout action
gives players a reason to make the round trip from Town Square.

## Acceptance Criteria
- Watchtower location has a Scout action entry with appropriate description
- TASK-011 (architect) can reference this definition for implementation
