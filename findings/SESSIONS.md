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

---

## 2026-03-29 — Playtest Session 3

**Skill**: playtest | **Mode**: Headless simulation | **Turns**: 0–100 (dumps at 25, 50, 75, 100)

**Build**: Clean — same pre-existing warnings, no new errors.

**BAL-002 VERIFIED FIXED**: Tensions at T25 are Guild=55, Order=30, Shadows=80. Diverge correctly from the start. Issue #4 closed.

**BAL-001 SIGNIFICANTLY IMPROVED** (TASK-007 + TASK-009 in effect):
- Whispers from the Mine resolves ~T28 (was T16)
- The Guild's Gambit resolves ~T45 (was T22)
- Successor fronts activate at those points; both resolve ~T55–T65
- Dead zone now T65–T100 (~35 turns). Was dead at T22. Major progress, still open.

**NEW — BAL-005 opened** (#21): Faction tension caps at 100 with no release mechanism.
- Shadows hits 100 at T50, Guild hits 100 at T75. Both stay pegged for 25–50 turns.
- No crisis fires, no decay, no consequence. Tension loses signal value.

**BAL-003 ONGOING**: Lena Marsh 349 at T100, Aldric frozen at 200. TASK-012 still open.

**BAL-004 ONGOING**: Order power 60→27 over 100 turns, trajectory confirmed. No task yet.

**Simulation health**: NPC movement healthy. Rumor spread working (BUG-001 still fixed: Finn Crowe 7 rumors, Lena 7 rumors). Faction power diverging (Guild 82, Shadows 63, Order 27).

---

## 2026-03-29 — Designer Session 1

**Skill**: designer | **Data source**: T50 + T100 simulation dumps, Playtest Session 3

### Core diagnosis

The simulation is generating *data* but not *stories*. Two layers exist:
1. **Fronts** — authored narrative with player hooks. Live, legible.
2. **Background simulation** — movement, wealth, tension, power. Always running, never speaking.

When fronts go silent (~T65), the world is technically alive but experientially inert. Adding more fronts is a short-term patch. The long-term answer is giving the background simulation a *voice*.

### Decisions made

- **Target session length**: open question — logged as design consideration in DESIGN-002. Affects BAL-001 strategy.
- **Order of Accord**: intentional decline vs. mechanical gap — flagged as decision point (TASK-013).

### Findings opened

- **DESIGN-002** (#22): Simulation needs a heartbeat — emergent events from stat thresholds. Major.

### Tasks opened

- **TASK-013** (#23): Designer decision — Order of Accord decline (intentional arc or gap?)
- **TASK-014** (#24): Implement tension threshold events (architect + worldbuild, high priority)

### Recommendation priority

1. TASK-014 — tension threshold events (high impact, low-cost, unlocks world liveness post-fronts)
2. TASK-013 — Order design decision (medium, answers BAL-004 direction)
3. DESIGN-002 emergent events system — medium-term architecture work

---

## 2026-03-30 — Designer Session 2

**Skill**: designer | **Tasks completed**: TASK-015

### Core question answered

The main gameplay loop: **undercover Crown investigator, investigating a hidden conspiracy, in a town where every front is a ticking clock against them.**

Prototype target: ~T65. The scenario ends when the fronts resolve. The player's choices determine the shape of that ending.

**Order of Accord decline**: intentional for the prototype. A waning institution as backdrop, not a mechanical problem to fix.

### Scenario frame (DESIGN-003, findings/DESIGN-003-scenario-frame.md)

- **Player**: The Envoy — arrives under cover as a traveling scribe. One contact (Lena Marsh). Knows nothing else.
- **The conspiracy**: Aldric Voss (Guild) secretly funds Sable (Shadows) for enforcement. Mine contains evidence: the Crown's missing assessor, miners' remains, Vex's documents.
- **Two new mechanics needed**: Evidence (rumor/testimony/document tiers) and Exposure (cover integrity with threshold events).
- **Five ending states**: Crown Justice / Partial Case / Empty Hands / Burned / Untimely End.
- **The clock**: front inflection points at T28, T45, T50, T65.

### What this unblocks

- TASK-016 (player action design) — now has full context to spec actions
- Worldbuild — can tune starter rumors to plant the right breadcrumbs
- Architect — evidence + exposure are the two new player stats needed

---

## 2026-03-31 — Playtest Session 4

**Skill**: playtest | **Mode**: Headless simulation | **Turns**: 0–100 (dumps at 25, 50, 75, 100) + per-turn 0–38

**Build**: Clean — 8 pre-existing dead-code warnings, no new errors. Both `ttrpg` and `simulate` binaries compile.

### Verifying TASK-014 (tension threshold events)

**BUG-003 OPENED** (#28, major): `faction_tension_thresholds` is not registered in `src/simulate.rs`. TASK-014 was wired into `src/main.rs` only. Zero tension threshold events appear in any dump across 100 turns. Fix is a one-line import + chain entry in simulate.rs.

Tension timeline confirmed (per-turn sim):
- Shadows: hits 75 at T17, hits 100 at T31 — no threshold events
- Merchant Guild: hits 75 around T36–T40 (in longer runs) — no threshold events

### Front pacing (BAL-001 re-check)

Current state:
- T25: Guild's Gambit stage 2/4 (countdown=5), Whispers stage 1/2 (countdown=2) — both active ✓
- Whispers resolves ~T29. Guild's Gambit resolves ~T40. Successors activate at those points.
- All 4 fronts complete by T67–T75. Dead zone: ~T67–T100.

**BAL-001 status**: Major improvement from original (T22 dead), now ~T67 dead. Still open. Tension threshold events (once fixed) may fill some of this gap in windowed mode.

### Ongoing confirmed findings

- **BAL-003**: Lena Marsh reaches 382 wealth at T100, Aldric frozen at 200. Cause: Aldric has no Greedy trait and stays in guild_hall. TASK-012 still open.
- **BAL-004**: Order of Accord power declines from 40 (T0) to 31 (T100). Confirmed intentional per designer. No task.
- **BAL-005**: Merchant Guild and Shadows both hit tension=100 by T50 and stay pegged. No release mechanism. Still open.

### TASK-017 (world-state player actions)

Three NPC-specific actions (WarnLena, PayForInfo, ShareWithThess) added via commit 8aeb434. Not testable in headless mode — requires windowed playtest with UI interaction.

### Simulation health

NPC movement healthy. Rumor spread working (BUG-001 still fixed: most NPCs 5–7 rumors at T100, Sable isolated at 2 due to Back Alleys). Faction power diverging: Guild 82 / Shadows 53 / Order 31 at T100.

---

## 2026-04-01 — Architect Session 2

**Skill**: architect | **Tasks completed**: BUG-003 fix

### BUG-003 fixed

Added `faction_tension_thresholds` to `src/simulate.rs` import and system chain (between `advance_fronts` and `faction_tension_effects`). Fix is one import line + one chain entry. Verified: threshold events fire at T20, T28, T31, T40 in a 40-turn test run. Issue #28 closed.

### Dump improvement: narrative events section

`build_dump` in `simulate.rs` now includes a `NARRATIVE EVENTS (all)` section that filters out movement/wealth noise and shows all non-trivial events in reverse chronological order (up to 30). This was needed because the `RECENT EVENTS (last 10)` window was being completely dominated by NPC movement events, hiding front progression and tension events. Previous playtest sessions were only seeing partial event timelines as a result.

### BAL-001 revised picture

With the dump fixed and BUG-003 resolved, the actual narrative event timeline for a 100-turn run:
- T9, T15, T20×2, T28, T29, T31×2, T40×3, T41, T51×2, T52×3, T61, T62
- 21 narrative events, excellent density through T62
- Dead zone: T63–T100

Designer session 2 set prototype target at T65 (game ends when fronts resolve), so the dead zone is outside the intended play window. BAL-001 remains formally open but is not blocking prototype. The long-term fix (DESIGN-002: emergent events from stat thresholds) is the correct architectural direction — filed as design issue, no task yet.
