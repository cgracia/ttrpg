---
id: TASK-009
assigned_to: worldbuild
priority: medium
status: open
created_by: designer
created_date: 2026-03-29
related_finding: BAL-001
---

# Write successor front content for both existing fronts

## What
For each of the two existing fronts ("The Guild's Gambit" and "Whispers from the Mine"),
design and write a successor front that activates when the original resolves.

The successor should feel like a *consequence* of the original. It's not a fresh
scenario — it emerges from what happened. The opening stage description should connect
explicitly to the resolution of its predecessor.

## Design Constraints (from designer)
- Successor fronts can be shorter (2-3 stages) — the world has already been set in motion
- Successor fronts should involve at least one NPC from the original front
- "Guild's Gambit" resolution = guild dominance → successor = what the guild does with power
- "Whispers from the Mine" resolution = cult/secret exposed (or entrenched) → successor follows that outcome
- Keep tone consistent with existing front flavour

## Depends On
- TASK-008 must be implemented before these fronts can be activated automatically
- But content can be written in parallel

## Acceptance Criteria
- Two new FrontTemplate entries added to data.rs (or a data file)
- Each has meaningful 2-3 stage progression with stage descriptions
- Original fronts reference successors by name via `successor_front` field
- Narrative voice is consistent with existing content
