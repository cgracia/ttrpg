---
id: TASK-011
assigned_to: architect
priority: low
status: open
created_by: designer
created_date: 2026-03-29
related_finding: DESIGN-001
---

# Implement Scout action logic at the Watchtower

## What
Implement the Scout action so that when the player uses it at the North Watchtower,
it reveals the current location of NPCs who are in "outdoor" locations.

## Design Spec (from designer)
- Query all NPCs (not the player), get their `AtLocation` component
- Filter to NPCs at locations tagged as outdoor (or not tagged as enclosed)
- Post results to EventLog: "From above, you can see [NPC name] near [location name]."
- Reveal 1-3 NPCs (cap it — full visibility breaks the partial-info pillar)
- If no outdoor NPCs visible, post a fallback: "The streets are quiet."

## Location tagging
Either:
a) Add an `is_outdoor: bool` or `location_type` field to LocationTemplate and tag locations
b) Hardcode a list of outdoor location names (simpler, acceptable for now)

Option (b) is fine for the vertical slice — don't over-engineer.

## Depends On
- TASK-010: worldbuild writes the action definition/content

## Why
Gives the Watchtower a mechanical reason to visit. Addresses DESIGN-001.

## Acceptance Criteria
- Scout action appears in interaction panel at Watchtower
- Executing it posts location-reveal events to the log
- Results are bounded (not a full NPC census)
- Behavior is correct whether NPCs are indoors or outdoors
