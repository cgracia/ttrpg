/// Headless simulation binary for AI playtesting.
///
/// Usage:
///   cargo run --bin simulate -- [turns] [dump_interval]
///
/// Defaults: 100 turns, dump every 25 turns.
/// Output: debug/simulate_TNNN.txt at each interval, plus a final dump.
///
/// No window, no GPU, no display required. Runs as fast as the CPU allows.
mod components;
mod data;
mod resources;
mod systems;

use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use std::time::Duration;

use data::build_world_data;
use resources::{EventLog, GameMode, InteractionState, TickEvent, WorldState, WorldTime};
use systems::fronts::{advance_fronts, faction_tension_effects, faction_tension_thresholds};
use systems::npc_ai::{faction_power_tick, npc_movement, npc_wealth_tick, spread_rumors};
use systems::player::player_exposure_tick;
use systems::time::cleanup_tick;

#[derive(Resource)]
struct SimConfig {
    target_turns: u32,
    dump_interval: u32,
}

fn main() {
    let mut args = std::env::args().skip(1);
    let target_turns: u32 = args.next().and_then(|s| s.parse().ok()).unwrap_or(100);
    let dump_interval: u32 = args.next().and_then(|s| s.parse().ok()).unwrap_or(25);

    eprintln!("[simulate] Running {target_turns} turns (dump every {dump_interval})");

    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::ZERO)))
        .insert_resource(WorldTime::new())
        .insert_resource(WorldState::default())
        .insert_resource(EventLog::default())
        .insert_resource(GameMode::default())
        .insert_resource(InteractionState::default())
        .insert_resource(SimConfig { target_turns, dump_interval })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                headless_tick,
                npc_movement,
                npc_wealth_tick,
                faction_power_tick,
                spread_rumors,
                player_exposure_tick,
                advance_fronts,
                faction_tension_thresholds,
                faction_tension_effects,
                cleanup_tick,
                check_limit,
            )
                .chain(),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut world_state: ResMut<WorldState>,
    mut event_log: ResMut<EventLog>,
) {
    let world_data = build_world_data();
    data::spawn_world(&mut commands, &world_data, &mut world_state, &mut event_log);
}

/// One tick per frame — replaces the real-time timer used in the windowed binary.
fn headless_tick(mut world_time: ResMut<WorldTime>, mut commands: Commands) {
    world_time.turn += 1;
    commands.insert_resource(TickEvent);
}

fn check_limit(
    world_time: Res<WorldTime>,
    config: Res<SimConfig>,
    world_state: Res<WorldState>,
    event_log: Res<EventLog>,
    npc_q: Query<
        (
            &components::ActorName,
            &components::AtLocation,
            &components::Wealth,
            &components::Knowledge,
            Option<&components::FactionMember>,
        ),
        With<components::Npc>,
    >,
    faction_q: Query<
        (&components::ActorName, &components::FactionPower, &components::FactionTension),
        With<components::FactionMarker>,
    >,
    front_q: Query<(&components::Front, &components::FrontStages)>,
) {
    let turn = world_time.turn;
    let is_interval = turn % config.dump_interval == 0;
    let is_done = turn >= config.target_turns;

    if !is_interval && !is_done {
        return;
    }

    let path = format!("debug/simulate_T{turn:04}.txt");
    let content = build_dump(turn, &world_state, &event_log, &npc_q, &faction_q, &front_q);
    std::fs::create_dir_all("debug").ok();
    std::fs::write(&path, &content).ok();
    eprintln!("[simulate] T{turn:04}: dump → {path}");

    if is_done {
        eprintln!("[simulate] Done.");
        std::process::exit(0);
    }
}

fn build_dump(
    turn: u32,
    world_state: &WorldState,
    event_log: &EventLog,
    npc_q: &Query<
        (
            &components::ActorName,
            &components::AtLocation,
            &components::Wealth,
            &components::Knowledge,
            Option<&components::FactionMember>,
        ),
        With<components::Npc>,
    >,
    faction_q: &Query<
        (&components::ActorName, &components::FactionPower, &components::FactionTension),
        With<components::FactionMarker>,
    >,
    front_q: &Query<(&components::Front, &components::FrontStages)>,
) -> String {
    let mut out = String::new();
    out.push_str(&format!("=== Ashenveil Simulation — Turn {turn} ===\n\n"));

    // Factions
    out.push_str("── FACTIONS ──\n");
    for (_id, name, entity) in &world_state.factions {
        if let Ok((_, power, tension)) = faction_q.get(*entity) {
            out.push_str(&format!("  {name}: power={}, tension={}\n", power.0, tension.0));
        }
    }
    out.push('\n');

    // NPCs
    out.push_str("── NPCs ──\n");
    for (_id, name, entity) in &world_state.npcs {
        if let Ok((_, at_loc, wealth, knowledge, faction)) = npc_q.get(*entity) {
            let loc = world_state.location_name(at_loc.0).unwrap_or("?");
            let faction_name = faction
                .and_then(|fm| {
                    world_state.factions.iter()
                        .find(|(_, _, e)| *e == fm.0)
                        .map(|(_, n, _)| n.as_str())
                })
                .unwrap_or("none");
            out.push_str(&format!(
                "  {name} @ {loc} [{faction_name}] wealth={} rumors={}\n",
                wealth.0,
                knowledge.0.len()
            ));
        }
    }
    out.push('\n');

    // Fronts
    out.push_str("── FRONTS ──\n");
    for (front, stages) in front_q.iter() {
        out.push_str(&format!(
            "  {} [stage {}/{}] active={} countdown={}\n",
            front.name,
            front.stage,
            stages.0.len().saturating_sub(1),
            front.active,
            front.countdown,
        ));
    }
    out.push('\n');

    // Narrative events — filter out trivial movement/wealth entries
    let is_trivial = |text: &str| {
        text.contains("moves to")
            || text.contains("picks up a rumor")
            || text.contains("profitable deal")
    };
    out.push_str("── NARRATIVE EVENTS (all) ──\n");
    let narrative: Vec<_> = event_log
        .recent(usize::MAX)
        .filter(|e| e.turn > 0 && !is_trivial(&e.text))
        .take(30)
        .collect();
    if narrative.is_empty() {
        out.push_str("  (none)\n");
    } else {
        for entry in narrative {
            out.push_str(&format!("  [T{}] {}\n", entry.turn, entry.text));
        }
    }
    out.push('\n');

    // Recent events — all, last 10
    out.push_str("── RECENT EVENTS (last 10) ──\n");
    for entry in event_log.recent(10) {
        if entry.turn > 0 {
            out.push_str(&format!("  [T{}] {}\n", entry.turn, entry.text));
        }
    }

    out
}
