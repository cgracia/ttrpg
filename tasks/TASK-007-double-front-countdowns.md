---
id: TASK-007
assigned_to: balance
priority: high
status: open
created_by: designer
created_date: 2026-03-29
related_finding: BAL-001
---

# Double all front countdown values in data.rs

## What
In `src/data.rs`, double all `countdown_turns` and `starting_countdown` values for
both existing fronts.

Current totals:
- "The Guild's Gambit": starting=4, stages=5+5+4+4 → total ~22 turns
- "Whispers from the Mine": starting=7, stages=6+5 → total ~18 turns

Target: both fronts should sustain active pressure for ~40-50 turns minimum.

## Why
Addresses BAL-001: both fronts currently resolve before turn 25, leaving the world
static. This is a stopgap while front chaining (TASK-008) is implemented.

## Acceptance Criteria
- Countdown values doubled in data.rs
- Playtester verifies fronts are still active at turn 40+
- BAL-001 remains open until TASK-008 (chaining) is also complete
