---
id: TASK-006
assigned_to: architect
priority: medium
status: open
created_by: planner
created_date: 2026-03-29
---

# Fix F12 Screenshot Producing Black Image

## What

Investigate why F12 screenshot capture (`debug.rs: screenshot_on_f12`) produces a
black image in the current environment (nix-shell with X11). Fix or document a
reliable workaround so playtesting has a visual observation path.

## Why

Addresses BUG-002. Without screenshots, the playtest skill is limited to F11 state
dumps — no visual feedback on UI layout, rendering, or NPC map positions.

## Acceptance Criteria

- F12 produces a non-black screenshot of the running game, OR
- A documented alternative (e.g., `scrot`, `xwd`, external capture) is confirmed
  working in the nix-shell environment and noted in SESSIONS.md / WORKFLOW.md
