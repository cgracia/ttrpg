---
id: TASK-001
assigned_to: architect
priority: high
status: open
created_by: playtest
created_date: 2026-03-29
related_finding: BUG-001
---

# Implement rumor spread between co-located NPCs

## What
Complete the `spread_rumors` system in `src/systems/npc_ai.rs`. Currently the function
stubs out a co-location loop using `Entity::PLACEHOLDER` and immediately discards the
result. No rumor ever reaches an NPC's `Knowledge` component.

## Why
The partial-information pillar depends on NPCs holding and sharing rumors. Without this,
players can never learn anything through NPC interaction, and the `Knowledge` component
is dead data.

## Acceptance Criteria
- NPCs sharing a location have a chance each tick to copy a rumor from one to another
- After ~30 turns, at least some NPCs should show `Rumors: N > 0` in the state dump
- Player can receive rumors when interacting with an NPC who has knowledge
- Rumors should have a credibility or source field so partial-information is preserved
