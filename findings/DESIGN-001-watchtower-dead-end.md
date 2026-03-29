---
id: DESIGN-001
type: design
severity: minor
status: open
session: 2026-03-29
found_by: playtest
---

# North Watchtower is a dead-end with no distinct value

## Observed
Watchtower connects only to Town Square. Contains only Brega Halm (Order of Accord).
No unique actions or mechanical hooks are present.

## Impact
No incentive to travel there unless specifically seeking Brega. One connection means
it's a round-trip from Town Square every time.

## Decision (2026-03-29, designer)
Keep the dead-end topology. Add a unique "Scout" action.
- The isolation communicates Brega's character (Order of Accord, watches from a distance)
- Scout action reveals location of outdoor NPCs — costs a round trip, pays off for tracking
- Second connection rejected: it dilutes what the topology communicates
- See TASK-010 (worldbuild) and TASK-011 (architect) for implementation
