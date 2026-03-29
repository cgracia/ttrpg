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

## Fix Needed
Verify on a physical display first. If it works there, the issue is environment-specific
and acceptable. If black on physical display too, investigate Bevy screenshot timing
(screenshot entity may need a frame delay before the observe callback fires).
