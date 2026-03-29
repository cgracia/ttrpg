---
id: TASK-005
assigned_to: designer
priority: low
status: done
created_by: planner
created_date: 2026-03-29
related_finding: DESIGN-001
---

# Make a design decision on North Watchtower

## Decision (2026-03-29)

**Keep the dead-end topology. Add a unique "Scout" action.**

The Watchtower's isolation is intentional — Brega Halm of the Order of Accord watches
from a distance, doesn't mix. The single connection communicates her separateness.
Do NOT add a second connection; it would dilute what the topology is saying.

**Scout action spec:**
- Available only at the Watchtower
- Costs one turn (travel there + action + travel back = real commitment)
- Reveals current location of 1-2 NPCs who are in outdoor/transit locations
- Does NOT reveal what they're doing or who they're with — just *where*
- Serves players who are tracking someone, planning an intercept, or unsure who is where

This hits the partial information pillar: you can see *where*, not *what*. It makes the
round trip a deliberate tradeoff worth considering, not a waste.

**What was NOT chosen:**
- Second connection to Market District: smooths over topology that communicates intent
- Full front-progress visibility: too much information, breaks partial-info pillar

## Follow-up Tasks Created
- TASK-010 (worldbuild): Add Scout action to Watchtower location definition
- TASK-011 (architect): Implement Scout action logic (reveals outdoor NPC locations)

## Why
Without a mechanical hook, the Watchtower has no pull. The round trip from Town Square
is a real cost that needs a real payoff.
