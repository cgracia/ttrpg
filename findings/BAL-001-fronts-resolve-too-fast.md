---
id: BAL-001
type: balance
severity: major
status: open
session: 2026-03-29
found_by: playtest
---

# Both fronts resolve before turn 25 — world goes static

## Observed
At turn 29, both fronts are `active=false, countdown=0`:
- "Whispers from the Mine" resolves ~turn 16
- "The Guild's Gambit" resolves ~turn 22

From turn 22 onward the world runs with no active narrative pressure. Factions still
fluctuate via tension, but no new events escalate, no stage descriptions change.

## Root Cause
Countdown values in `src/data.rs` sum to:
- Gambit: 4 + 5 + 5 + 4 + 4 = 22 turns total
- Whispers: 7 + 6 + 5 = 18 turns total

With no mechanism to spawn new fronts after resolution, the world becomes static.

## Impact
- Core "living world" pillar breaks down after ~turn 22
- Player has no narrative arc driving them forward
- Simulation runs but produces no meaningful consequences

## Fix Options
1. Double or triple countdown values (simplest, buys time)
2. Add a third front to maintain parallel pressure
3. Implement front chaining: resolving one front can activate another
4. Add a "world event" system that fires independently of fronts
