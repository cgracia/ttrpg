# Session Log

## 2026-03-29 — Playtest Session 1

**Skill**: playtest | **Turn reached**: 29 | **Mode**: Exploration

**Environment notes**: Game requires `nix-shell shell.nix` to launch (X11 libs).
Use `nix-shell -p xdotool` to send F11/F12 keypresses. Screenshot (F12) produces
a black image in this environment — use F11 state dump as primary observation tool.

**Findings opened**:
- BUG-001: Rumor spread non-functional (major)
- BUG-002: Screenshot black (major)
- BAL-001: Fronts resolve too fast (major)
- BAL-002: Faction tension identical across factions (minor)
- UX-001: Market District empty (minor)
- DESIGN-001: Watchtower dead-end (minor)

**What's working**: Build clean, game launches, time advances, NPC movement,
wealth accumulation, faction power divergence, front advancement mechanics.
