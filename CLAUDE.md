# Ashenveil — Project Bible

## Overview

Ashenveil is a **simulation-driven RPG** built in **Bevy 0.18 + Rust**. The player inhabits a living world of autonomous NPCs, evolving factions, and hidden information. Core fantasy: *you are one person in a world that moves without you — but where your choices meaningfully shape outcomes.*

## Design Pillars

1. **Living World** — NPCs have goals, relationships, routines. Factions pursue interests. Fronts (evolving situations) progress over time. The player is not the center.
2. **Partial Information** — Knowledge is incomplete, biased, contextual. No omniscient UI. Learn through dialogue, observation, rumor, consequence.
3. **Meaningful Choices** — Decisions are bets made on incomplete knowledge. They carry risk and commitment.
4. **Simple Systems, Deep Interactions** — Each system is intentionally simple. Depth emerges from interaction, not complexity.
5. **Modular World Design** — Small reusable modules (fronts, location sets, NPC groups) enable expansion and experimentation.

## Architecture

### ECS Layout

**Components** (`src/components.rs`):
- Markers: `Player`, `Npc`, `LocationMarker`, `FactionMarker`
- Identity: `ActorName(String)`, `Description(String)`
- Stats: `Stats { strength, charisma, cunning, resolve }`
- Character: `Traits(Vec<TraitKind>)`, `Goals(Vec<Goal>)`, `Relationships`, `Knowledge(Vec<Rumor>)`
- Spatial: `AtLocation(Entity)`, `MapPos(Vec2)`, `Connections(Vec<Entity>)`
- Faction: `FactionMember(Entity)`, `FactionPower(i32)`, `FactionTension(i32)`
- Economy: `Wealth(i32)`
- Narrative: `Front { name, description, stage, countdown, active }`, `FrontStages(Vec<FrontStage>)`
- Behavior: `NpcBehavior { routine: NpcRoutine, move_cooldown }`
- UI: `EventLogUi`, `PlayerPanelUi`, `LocationPanelUi`, `InteractionPanelUi`, `LocationNodeUi`, etc.

**Resources** (`src/resources.rs`):
- `WorldTime` — turn counter, real-time timer, pause state
- `EventLog` — timestamped event entries
- `WorldState` — lookup tables for locations/factions/NPCs by id/name/entity
- `GameMode` — Exploration | Interaction(Entity) | Travel
- `InteractionState` — selected NPC, dialogue lines, action options
- `TickEvent` — inserted when a simulation tick should fire

**Systems** (`src/systems/`):
- `time.rs` — `advance_time` (timer → TickEvent), `cleanup_tick` (remove TickEvent)
- `npc_ai.rs` — `npc_movement`, `npc_wealth_tick`, `faction_power_tick`, `spread_rumors`
- `fronts.rs` — `advance_fronts`, `faction_tension_effects`
- `interaction.rs` — `build_interaction`, `build_travel_options`, `execute_player_action`
- `debug.rs` — `screenshot_on_f12`, `state_dump_on_f11` (debug/dev tools)

**UI** (`src/ui.rs`):
- `spawn_ui` / `spawn_map_nodes` — layout and map node rendering
- `update_*` systems — refresh panels each frame
- `handle_*` systems — button clicks, keyboard shortcuts

**Data** (`src/data.rs`):
- RON-serializable templates (`LocationTemplate`, `NpcTemplate`, etc.)
- `build_world_data()` — currently hardcoded inline, will transition to RON files
- `spawn_world()` — instantiates entities from templates

### System Ordering

Simulation systems are chained in `main.rs`:
```
advance_time → npc_movement → npc_wealth_tick → faction_power_tick → spread_rumors → advance_fronts → faction_tension_effects → cleanup_tick
```

UI systems run in parallel (no chain needed).

### Key Patterns

- **Without<T> filters**: Required to avoid query conflicts (e.g., Player vs Npc queries both accessing AtLocation)
- **TickEvent resource**: Simulation systems guard on `Res<TickEvent>` existence — only run logic when a tick fires
- **Chained systems**: The update chain preserves simulation ordering and Bevy inserts deferred application points automatically where needed
- **Entity references**: Components like `AtLocation(Entity)`, `FactionMember(Entity)`, `Connections(Vec<Entity>)` store entity handles directly

### Known Pitfalls

- Bevy 0.18 uses the component-style UI and camera API (`Camera2d`, `Node`, `Button`, `Text`) instead of the older bundle wrappers
- Query conflicts: any two queries accessing the same component mutably (or one mutable + one immutable) on overlapping archetypes must use `Without<T>` filters
- `Query::single()` and `single_mut()` return `Result`; handle missing or duplicate matches explicitly

## Current State

**Working**: Vertical slice with 8 locations, 3 factions, 10 NPCs, 2 fronts, map UI, interaction system, time progression, NPC movement, rumor spreading, front advancement.

**Missing**: RON data loading (data is hardcoded), save/load, combat, equipment system, deeper dialogue, procedural content, art assets.

## Visual Feedback (Debug Tools)

- **F12** — Screenshot → saves to `screenshots/latest.png`
- **F11** — State dump → saves world state to `debug/world_state.txt`

These let AI skills "see" the game and analyze simulation state.

## File Structure

```
src/
├── main.rs          — App setup, system registration, startup
├── components.rs    — All ECS components
├── resources.rs     — All ECS resources
├── data.rs          — World data templates and spawning
├── ui.rs            — UI layout and update systems
└── systems/
    ├── mod.rs       — Module declarations
    ├── time.rs      — Time progression
    ├── npc_ai.rs    — NPC behavior systems
    ├── fronts.rs    — Front progression
    ├── interaction.rs — Player interaction logic
    └── debug.rs     — Screenshot and state dump systems
```

## Project Tracking

- `WORKFLOW.md` — how skills collaborate, artifact conventions, review process, `gh` CLI commands
- GitHub Issues — tasks and findings tracked at https://github.com/cgracia/ttrpg/issues
  - Open tasks: `gh issue list --label task`
  - Open findings: `gh issue list --label finding`
  - Current sprint: `gh issue list --label sprint:current`
- `findings/SESSIONS.md` — append-only session history log (local)

Read WORKFLOW.md and check GitHub Issues when starting a session to understand current project state.

## Conventions

- Keep systems small and focused — one responsibility per system function
- Prefer `Query` filters over runtime checks
- Use `EventLog` for player-visible events, `info!()` for dev logging
- Content generation should match existing template patterns in `data.rs`
- Trait/goal enums are the canonical vocabulary — extend them when adding content
