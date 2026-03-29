---
id: TASK-008
assigned_to: architect
priority: medium
status: open
created_by: designer
created_date: 2026-03-29
related_finding: BAL-001
---

# Implement optional successor_front and front chaining logic

## What
Add an optional `successor_front: Option<String>` field to `FrontTemplate` (and the
corresponding `Front` component or its data). When a front resolves (reaches its
final stage and countdown hits zero), check if a successor is named. If so, find the
front entity with that name and set `active = true`, `countdown = starting_countdown`,
`stage = 0`.

## Design Intent
Fronts don't just end — they *change the situation*. A resolved front opens a new one.
This is the mechanism that keeps the world from going static after all fronts resolve.

No new system needed. This is just a field on FrontTemplate and a few lines in the
`advance_fronts` system in `src/systems/fronts.rs`.

## Why
Addresses BAL-001 (world goes static) at the structural level. TASK-007 is the
stopgap; this is the permanent fix.

## Acceptance Criteria
- `FrontTemplate` has optional `successor_front: Option<String>` field
- `advance_fronts` activates successor when a front resolves
- Works correctly when `successor_front` is None (no-op, same as current behavior)
- Playtester can verify chaining fires by watching the event log
