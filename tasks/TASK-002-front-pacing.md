---
id: TASK-002
assigned_to: designer
priority: high
status: done
created_by: playtest
created_date: 2026-03-29
related_finding: BAL-001
---

# Decide on front pacing and lifecycle strategy

## Decision (2026-03-29)

**Two-part fix: extended countdowns + front chaining.**

### Part 1: Double countdown values (immediate, data-only fix)
Current countdowns sum to ~22 turns total. Double all stage countdowns in data.rs.
This buys ~50 turns of active narrative pressure and unblocks playtest immediately.
This is a stopgap — it buys time but doesn't solve "what happens after all fronts resolve."

### Part 2: Implement front chaining (structural fix)
Each front can optionally name a `successor_front`. When a front resolves, if a
successor is set, that front activates immediately. This means the world never goes
static — it changes. A resolved front represents something that *happened*, and
things that happen have consequences that become new fronts.

**What was NOT chosen:**
- A third parallel front: scope bloat before the core loop is validated
- A separate "world events" system: a front with a short countdown IS a world event —
  don't add a new system that duplicates what fronts already do

## Follow-up Tasks Created
- TASK-007 (balance): Double all front countdown values in data.rs
- TASK-008 (architect): Implement optional successor_front field and front chaining logic
- TASK-009 (worldbuild): Write successor front content for both existing fronts

## Why
Without active fronts, the simulation runs but produces no escalating consequences.
The "living world" pillar degrades after turn 22.
