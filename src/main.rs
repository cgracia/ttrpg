mod components;
mod data;
mod resources;
mod systems;
mod ui;

use bevy::prelude::*;

use data::build_world_data;
use resources::*;
use systems::debug::{screenshot_on_f12, state_dump_on_f11};
use systems::fronts::{advance_fronts, faction_tension_effects, faction_tension_thresholds};
use systems::interaction::{build_interaction, build_travel_options};
use systems::npc_ai::{faction_power_tick, npc_movement, npc_wealth_tick, spread_rumors};
use systems::player::player_exposure_tick;
use systems::time::{advance_time, cleanup_tick};
use components::*;
use ui::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Ashenveil — Intrigue Prototype".into(),
                resolution: (1200, 700).into(),
                ..default()
            }),
            ..default()
        }))
        // Resources
        .insert_resource(WorldTime::new())
        .insert_resource(WorldState::default())
        .insert_resource(EventLog::default())
        .insert_resource(GameMode::default())
        .insert_resource(InteractionState::default())
        // Startup
        .add_systems(Startup, (setup_world, spawn_ui, spawn_map_nodes).chain())
        // Simulation (runs every frame, guards internally on TickEvent)
        .add_systems(
            Update,
            (
                advance_time,
                npc_movement,
                npc_wealth_tick,
                faction_power_tick,
                spread_rumors,
                player_exposure_tick,
                advance_fronts,
                faction_tension_thresholds,
                faction_tension_effects,
                cleanup_tick,
            )
                .chain(),
        )
        // UI update systems
        .add_systems(
            Update,
            (
                // build_interaction must run before handle_action_buttons — both access
                // FactionMarker components (one reads, one mutates after TASK-017).
                build_interaction.before(handle_action_buttons),
                build_travel_options,
                update_player_panel,
                update_location_panel,
                update_time_label,
                update_event_log,
                update_interaction_panel,
                handle_ui_buttons,
                handle_action_buttons,
                handle_location_clicks,
                handle_npc_interaction_from_location,
                keyboard_shortcuts,
                highlight_player_location,
                screenshot_on_f12,
                state_dump_on_f11,
            ),
        )
        .run();
}

fn setup_world(
    mut commands: Commands,
    mut world_state: ResMut<WorldState>,
    mut event_log: ResMut<EventLog>,
) {
    // 2D camera
    commands.spawn(Camera2d);

    let world_data = build_world_data();
    data::spawn_world(&mut commands, &world_data, &mut world_state, &mut event_log);
}

/// Keep location node colours in sync with player position.
fn highlight_player_location(
    player: Query<&components::AtLocation, With<components::Player>>,
    mut nodes: Query<(&LocationNodeUi, &mut BackgroundColor)>,
) {
    let Ok(player_at) = player.single() else { return };
    for (node, mut bg) in nodes.iter_mut() {
        let is_here = node.location_entity == player_at.0;
        // Only set default/player colour; hover colour is set in handle_location_clicks
        *bg = if is_here {
            Color::srgb(0.35, 0.75, 0.45).into()
        } else {
            Color::srgb(0.25, 0.40, 0.55).into()
        };
    }
}
