---
id: TASK-003
assigned_to: balance
priority: low
status: open
created_by: planner
created_date: 2026-03-29
related_finding: BAL-002
---

# Differentiate faction starting tension and front escalation targeting

## What
1. Set distinct starting tension values per faction in `data.rs` to reflect their nature
2. Make front stage escalation add tension only to the factions involved in that front,
   rather than applying +10 to all factions globally

## Why
Currently all factions start at identical tension (60/60/60) and all receive equal
tension increases from every front advance. This makes the "instability coin-flip"
mechanically identical across factions and loses per-faction flavor.

See BAL-002 for full context and observed values.

## Acceptance Criteria
- Merchant Guild starts at lower tension (~40) — established power, stable
- Order of Accord starts at low tension (~25) — law-keepers, controlled
- The Shadows start at higher tension (~70) — inherently volatile, hunted
- Front stage escalation targets specific factions (defined in FrontTemplate)
- After turn 30, faction tensions should show meaningful divergence in state dump
