---
id: BUG-002
type: bug
severity: major
status: open
session: 2026-03-29
found_by: playtest
---

# Screenshot (F12) produces a black image

## Observed
F12 creates `screenshots/latest.png` (18KB file), but the image is solid near-black.
No UI, map, or text is visible.

## Context
- Game window is running and rendering (window title correct, F11 dump works)
- Bevy's `Screenshot::primary_window()` + `save_to_disk` is the mechanism
- This was tested in a remote X11 environment (no physical GPU display)

## Hypothesis
GPU framebuffer capture may not work correctly when the window isn't presented to a
physical display. Bevy's screenshot system reads from the GPU render target, which may
be blank if compositing isn't active or if the window hasn't been presented.

## Impact
- AI skills (playtest, worldbuild) cannot visually inspect the game
- F11 state dump is the only reliable remote observation tool

## Playtest Session 2 Update — 2026-03-29

The architect's scrot fix does not resolve this on the dev machine. Root cause confirmed:
the machine runs **Sway** (Wayland compositor). Bevy launches as a native Wayland window
(`XDG_SESSION_TYPE=wayland`, `WAYLAND_DISPLAY=wayland-1`). `scrot` captures via
XWayland (`:0`), which has no content from the Wayland-native Bevy window — so scrot
also produces a black image. The `screenshots/latest.png` from the previous session
(18KB) shows a uniform near-black background with no UI.

The fix path is to use a Wayland-native capture tool (`grim`) instead of `scrot`, or
to fix Bevy's own screenshot API. The current code falls back to Bevy's API when scrot
fails, but scrot succeeds (exit 0) while still capturing black — so the fallback is
never reached and Bevy's API is never attempted.

**Specific code problem**: `scrot -d 0` exits successfully even when capturing an empty
XWayland display. The game treats that as a successful capture and never tries the Bevy API.

## Fix Needed
Option A (recommended): Try `grim` (Wayland) before `scrot` (X11) in debug.rs.
Option B: Check the captured file's pixel content, not just exit code, before
  treating the capture as successful.
Option C: Accept environment limitation — F11 dump is the canonical AI observation tool.
