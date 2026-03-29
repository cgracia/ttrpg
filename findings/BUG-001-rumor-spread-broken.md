---
id: BUG-001
type: bug
severity: major
status: fixed
session: 2026-03-29
found_by: playtest
---

# Rumor spread system non-functional

## Observed
All 10 NPCs have `Rumors: 0` at turn 29. The `spread_rumors` system fires every tick
but never writes to any NPC's `Knowledge` component.

## Root Cause
`src/systems/npc_ai.rs` lines 164–174:

```rust
let snapshot: Vec<(Entity, Entity, Vec<String>)> = npcs
    .iter()
    .map(|(_, at, know)| {
        (Entity::PLACEHOLDER, at.0, know.0.iter()...)  // wrong — PLACEHOLDER, not real entity
    })
    .collect();

let entities: Vec<Entity> = npcs.iter().map(|(_, at, _)| at.0).collect();
let _ = (snapshot, entities);  // entire snapshot thrown away
```

The co-location matching loop was never written. Only the fallback 15%-chance log
event runs, which posts "(Rumor) ..." to the event log but writes nothing to Knowledge.

## Impact
- Player cannot learn rumors from NPCs (NPCs have none to share)
- The partial-information pillar is broken at the data layer
- Rumor spread is a core simulation system per the design doc

## Fix Needed
Implement the co-location loop: for each pair of NPCs at the same location, with some
probability, copy a rumor from one's Knowledge to the other's. Requires iterating with
real entity IDs (use `npcs.iter().map(... entity ...)` pattern or split query).

## Verified Fixed — 2026-03-29 (Playtest Session 2)
At Turn 92: all 10 NPCs have 3–7 rumors. Several NPCs are at the spread cap (7).
The two-phase snapshot approach in npc_ai.rs works correctly. Credibility
degradation confirmed (rumors passed between NPCs degrade by 15% per hop).
AskRumor interaction now surfaces real NPC knowledge.
