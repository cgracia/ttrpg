# Session Log

## 2026-03-29 — Architect Session 1

**Skill**: architect | **Tasks completed**: TASK-001, TASK-006

**BUG-001 fix** (`src/systems/npc_ai.rs`, `src/data.rs`, `src/systems/interaction.rs`):
- Rewrote `spread_rumors` with proper two-phase approach: immutable snapshot pass groups
  NPCs by location, then a mutable pass calls `npcs.get_mut()` per recipient. Adds `Entity`
  to the query tuple so entity IDs are available.
- Added `starter_rumors: Vec<String>` field to `NpcTemplate` (`#[serde(default)]`).
  Five NPCs (Finn, Lena, Otto, Vex, Tomas) seeded with 1-2 starting rumors each.
- Updated `AskRumor` handler to surface NPC's actual `Knowledge` when available,
  falling back to `generate_rumor` when empty.
- Credibility degrades 15% on each spread hop — partial information is preserved.

**BUG-002 fix** (`src/systems/debug.rs`):
- F12 now tries `scrot` first (more reliable in X11 environments). Falls back to
  Bevy's screenshot API. Playtest skill needs `scrot` in nix-shell: `nix-shell -p scrot`.
- BUG-002 marked `needs-verify` — requires playtest to confirm scrot path works.

**Build**: Clean (`cargo build` passes with only pre-existing warnings).

---

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
