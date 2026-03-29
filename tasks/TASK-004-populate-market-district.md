---
id: TASK-004
assigned_to: worldbuild
priority: medium
status: open
created_by: planner
created_date: 2026-03-29
related_finding: UX-001
---

# Add NPC presence to Market District

## What
Add at least one NPC whose routine prioritizes the Market District as a frequent
location. Options:
- A new NPC (merchant, trader, fence, information broker) with Market as primary
- Modify an existing NPC's patrol to include Market District as a frequent stop

## Why
Market District is described as the main commercial hub but is consistently empty
at turn 29 (UX-001). Players traveling there find no one to interact with, making
it a dead location that undermines the "living world" pillar.

## Acceptance Criteria
- At least one NPC present in Market District at most simulation turns
- The NPC's faction, goals, and traits make narrative sense for a market setting
- If a new NPC is added, they are internally consistent with the existing world
- State dump at turn 20+ shows an NPC at `market` most of the time
