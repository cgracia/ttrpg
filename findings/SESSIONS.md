# Session Log

## 2026-03-29 — Worldbuild Session 1

**Skill**: worldbuild | **Tasks completed**: TASK-009, TASK-010

**TASK-009 — Successor fronts** (`src/data.rs`):
- **The Iron Ledger** (successor to The Guild's Gambit): 3-stage arc tracing what Guild dominance feels like from the inside — tribute extraction, Mira seizing the docks (Lena removed), Canon Thess's public reckoning. Tension escalates hard on guild+order+shadows.
- **What the Mine Swallowed** (successor to Whispers from the Mine): 3-stage knowledge/hunt arc — documents reach the Order (Brega's reaction signals gravity), Aldric hunts Vex through Finn Crowe, truth surfaces or is buried. Involves Brega, Vex, Finn Crowe.
- Wired `successor_front` fields in both parent fronts (previously `None`).

**TASK-010 — Scout action flavor** (`src/systems/interaction.rs`):
- Label: "Survey the roads below"
- Empty result: "The roads below are empty. Whatever's happening in this town, it's happening indoors."
- With sightings: "From this height you can see who moves through the open streets. {name} near the {location}."
- Tone: cautious, partial-information register consistent with Brega's voice and world flavor.

**Build**: Clean (`cargo build` passes, no new errors or warnings).

---

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

---

## 2026-03-29 — Playtest Session 2

**Skill**: playtest | **Turn reached**: 92 | **Mode**: Exploration

**Environment update**: Machine runs **Sway** (Wayland). `xdotool` cannot send
keypresses to a Wayland-native Bevy window. F11 state dump now requires:
1. Start `ydotoold` daemon: `nix-shell -p ydotool --run "ydotoold" &`
2. Focus game: `SWAYSOCK=/run/user/1000/sway-ipc.*.sock swaymsg '[title="Ashenveil"]' focus`
3. Send F11: `YDOTOOL_SOCKET=/run/user/1000/.ydotool_socket nix-shell -p ydotool --run "ydotool key 87:1 87:0"`

**BUG-001 VERIFIED FIXED**: All 10 NPCs have 3–7 rumors at T92 (was 0 at T29).
Rumor spread and credibility degradation confirmed working.

**BUG-002 STILL BROKEN**: scrot fix from architect does not work. `scrot -d 0` exits
successfully on Wayland but captures empty XWayland display (black). The game is a
native Wayland window — XWayland tools cannot capture it. `screenshots/latest.png`
remains black. Status changed back to `open`. See BUG-002 file for fix options.

**BAL-001 CONFIRMED OPEN**: Both fronts at final stage, `active=false`, at T92.
World has been static for ~70 turns. TASK-007 (double countdown values) not yet done.

**BAL-002 CONFIRMED OPEN**: All three factions still at tension=60 at T92.

**UX-001 STATUS**: Market District had 2 NPCs at time of dump (Mira Dent, Finn Crowe).
Movement is naturally distributing NPCs. TASK-004 (dedicated Market NPC) still open
but organic presence observed — severity may be lower than originally assessed.

**Simulation health**: NPC movement active, wealth diverging (Lena Marsh 358, Brega 10),
faction power diverging (Guild 77, Order 27, Shadows 51). Core loop is healthy.

---

## 2026-03-29 — Architect Session 2

**Skill**: architect | **Tasks completed**: TASK-008, TASK-011

**TASK-008 — Front chaining** (`src/components.rs`, `src/data.rs`, `src/systems/fronts.rs`):
- Added `starting_countdown: u32` and `successor_front: Option<String>` to `Front` component.
- Added `successor_front: Option<String>` (`#[serde(default)]`) to `FrontTemplate`.
- Updated `spawn_world` to pass both fields when spawning front entities.
- Updated `advance_fronts`: when a front resolves, collects successor names into a Vec,
  then does a second `iter_mut()` pass to activate them (avoids double-borrow). Activating
  a successor sets `active=true`, resets `stage=0`, restores `countdown=starting_countdown`,
  and logs an event. Both current fronts have `successor_front: None` pending TASK-009
  (worldbuild) wiring the names in.

**TASK-011 — Scout action** (`src/resources.rs`, `src/systems/interaction.rs`, `src/ui.rs`):
- Added `PlayerAction::Scout` variant.
- `build_travel_options` now takes `world: Res<WorldState>`; when player is at Watchtower
  location, injects a "Scout the roads below" option into the travel panel.
- `execute_player_action`: added `&AtLocation` to `npc_query` tuple; implemented Scout
  handler using hardcoded outdoor location list (option b — no LocationTemplate changes).
  Reveals up to 3 NPCs in outdoor locations; falls back to "streets are quiet" if none.
- Updated `handle_action_buttons` in ui.rs to include `&AtLocation` in its npc_query.

**Build**: Clean (`cargo build` passes, no new errors or warnings).

---

## 2026-03-29 — Balance Session 1

**Skill**: balance | **Data source**: T92 state dump (Playtest Session 2) + source code analysis

**Modeled front resolution timelines (current values):**
- Guild's Gambit: resolves T22
- Whispers from the Mine: resolves T16

**TASK-007 correction filed**: Blanket ×2 undershoots for Whispers (→ T29, below 40-turn target). Recommended specific values added as comment on TASK-007:
- Guild's Gambit: starting=8, stages 10/10/8/8/0 → **T41** ✓
- Whispers: starting=12, stages 12/10/0 → **T42** ✓

**TASK-003 tension values filed**: Specific starting tensions with rationale added as comment on TASK-003:
- Shadows: start=55 (above instability threshold immediately)
- Guild: start=30 (destabilizes ~T20)
- Order: start=15 (last to destabilize, ~T35)
- Front escalation targeting: Guild's Gambit → Guild+Shadows +10, Order +5

**New findings opened:**
- BAL-003 (#18): Lena Marsh wealth outpaces Guildmaster by T70 (Greedy+StayPut at Docks = 3 coins/tick guaranteed). TASK-012 (#19) filed: 50% gain probability + Aldric wealth path via Guild Hall.
- BAL-004 (#20): Order has no power growth mechanism; net -0.2/tick decline. Projection: power=0 at T165. Design question (intentional thematic decline?) — no task filed pending designer input.

---

## 2026-03-29 — Balance Session 2

**Skill**: balance | **Tasks completed**: TASK-007, TASK-003

**TASK-007 — Front countdown values** (`src/data.rs`):
- Guild's Gambit: starting=8 (was 4), stages [10/10/10/8/0] → resolves ~T41
- Whispers from the Mine: starting=14 (was 7), stages [12/12/0] → resolves ~T27
- Whispers still short of T40 target due to 3-stage structure (stage 2 countdown=0); T27 is a big improvement from T15. Successor content (TASK-009) covers T27+.

**TASK-003 — Faction tension differentiation** (`src/components.rs`, `src/data.rs`, `src/systems/fronts.rs`):
- Added `starting_tension: i32` to `FactionTemplate`; `spawn_world` uses it instead of hardcoded 0
- Shadows start at 55 (instability active from turn 1), Guild at 30, Order at 15
- Added `tension_targets: Vec<(String, i32)>` to `FrontStageTemplate` and `FrontStage`
- `advance_fronts` now does targeted escalation via `WorldState::faction_entity` lookup instead of blanket +10
- Guild's Gambit: Guild+Shadows each take +10/stage, Order +5; open conflict stage gives Order +10
- Whispers from the Mine: all factions +5/stage (ambient dread); Shadows take +10 on final stage (Vex ambush)

**Build**: Clean (`cargo build` passes, no new errors or warnings).

**Simulation health summary (T92):**
- Faction power: Guild 77 / Shadows 51 / Order 27 — diverging as expected
- Faction tension: all at 60 (uniform, BAL-002 open)
- Lena wealth: 358 (runaway, BAL-003 now open)
- Fronts: both resolved ~T22/T16, static for 70 turns (BAL-001 open, TASK-007 pending)
