/// Advance evolving fronts (situations) each tick.
use bevy::prelude::*;

use crate::components::{
    Front, FrontStages, FactionMarker, FactionPower, FactionTension,
    TensionEvents, FiredThresholds,
};
use crate::resources::{EventLog, TickEvent, WorldState, WorldTime};

pub fn advance_fronts(
    tick: Option<Res<TickEvent>>,
    mut fronts: Query<(&mut Front, &FrontStages)>,
    mut factions: Query<(&mut FactionPower, &mut FactionTension), With<FactionMarker>>,
    mut log: ResMut<EventLog>,
    time: Res<WorldTime>,
    world: Res<WorldState>,
) {
    if tick.is_none() {
        return;
    }

    let mut to_activate: Vec<String> = Vec::new();

    for (mut front, stages) in fronts.iter_mut() {
        if !front.active {
            continue;
        }

        if front.countdown == 0 {
            // Advance to next stage
            let next_stage = front.stage as usize + 1;
            if next_stage >= stages.0.len() {
                front.active = false;
                log.push_at(
                    time.turn,
                    format!(
                        "[{}] The situation has resolved — for now.",
                        front.name
                    ),
                );
                if let Some(succ) = &front.successor_front {
                    to_activate.push(succ.clone());
                }
                continue;
            }

            front.stage = next_stage as u8;
            let stage = &stages.0[next_stage];
            front.countdown = stage.countdown_turns;

            log.push_at(time.turn, stage.event_log_entry.clone());

            // Front escalation raises tension for targeted factions
            let targets = stage.tension_targets.clone();
            for (faction_id, delta) in &targets {
                if let Some(faction_entity) = world.faction_entity(faction_id) {
                    if let Ok((_power, mut tension)) = factions.get_mut(faction_entity) {
                        tension.0 = (tension.0 + delta).clamp(0, 100);
                    }
                }
            }
        } else {
            front.countdown -= 1;
        }
    }

    // Activate any successor fronts
    for succ_name in to_activate {
        for (mut front, _stages) in fronts.iter_mut() {
            if front.name == succ_name && !front.active {
                front.active = true;
                front.stage = 0;
                front.countdown = front.starting_countdown;
                log.push_at(
                    time.turn,
                    format!("[{}] A new situation begins to unfold.", front.name),
                );
                break;
            }
        }
    }
}

/// Fire a narrative event the first time faction tension crosses a threshold.
/// One event per faction per threshold — never repeats.
pub fn faction_tension_thresholds(
    tick: Option<Res<TickEvent>>,
    mut factions: Query<
        (&FactionTension, &TensionEvents, &mut FiredThresholds),
        With<FactionMarker>,
    >,
    mut log: ResMut<EventLog>,
    time: Res<WorldTime>,
) {
    if tick.is_none() {
        return;
    }

    for (tension, events, mut fired) in factions.iter_mut() {
        for (threshold, text) in &events.0 {
            if tension.0 >= *threshold && !fired.0.contains(threshold) {
                fired.0.push(*threshold);
                log.push_at(time.turn, text.clone());
            }
        }
    }
}

/// When tension is high, faction power fluctuates.
pub fn faction_tension_effects(
    tick: Option<Res<TickEvent>>,
    mut factions: Query<(&mut FactionPower, &FactionTension), With<FactionMarker>>,
) {
    if tick.is_none() {
        return;
    }

    let mut rng = rand::thread_rng();
    use rand::Rng;

    for (mut power, tension) in factions.iter_mut() {
        if tension.0 >= 50 && rng.gen_bool(0.2) {
            // High tension causes instability
            let delta: i32 = rng.gen_range(-3..=1);
            power.0 = (power.0 + delta).clamp(0, 100);
        }
    }
}
