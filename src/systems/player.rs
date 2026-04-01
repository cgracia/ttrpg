/// Per-tick player stat updates: exposure decay, proximity checks, threshold events.
use bevy::prelude::*;

use crate::components::{
    AtLocation, Evidence, Exposure, ExposureEvents, FiredExposureThresholds, Npc, Player,
};
use crate::resources::{EventLog, TickEvent, WorldState, WorldTime};

/// Each tick:
/// 1. Raise exposure for risky proximity (Back Alleys + Sable, Vex co-location).
/// 2. Apply passive -1 decay.
/// 3. Fire one-shot threshold events when value crosses 33 / 66 / 100.
pub fn player_exposure_tick(
    tick: Option<Res<TickEvent>>,
    mut player: Query<
        (&AtLocation, &mut Exposure, &ExposureEvents, &mut FiredExposureThresholds),
        With<Player>,
    >,
    npc_locs: Query<&AtLocation, With<Npc>>,
    world: Res<WorldState>,
    mut log: ResMut<EventLog>,
    time: Res<WorldTime>,
) {
    if tick.is_none() {
        return;
    }

    let Ok((player_at, mut exposure, events, mut fired)) = player.single_mut() else {
        return;
    };
    let player_loc = player_at.0;

    // Raise: Back Alleys + Sable present
    let back_alley = world.location_entity("back_alley");
    let sable_loc = world
        .npc_entity("sable")
        .and_then(|e| npc_locs.get(e).ok())
        .map(|at| at.0);

    if Some(player_loc) == back_alley && Some(player_loc) == sable_loc {
        exposure.value = (exposure.value + 8).min(100);
    }

    // Raise: consecutive turns co-located with Vex
    let vex_loc = world
        .npc_entity("vex")
        .and_then(|e| npc_locs.get(e).ok())
        .map(|at| at.0);

    if Some(player_loc) == vex_loc {
        exposure.vex_proximity_turns += 1;
        if exposure.vex_proximity_turns == 2 {
            exposure.value = (exposure.value + 5).min(100);
            exposure.vex_proximity_turns = 0; // reset so it triggers again after another 2-turn streak
        }
    } else {
        exposure.vex_proximity_turns = 0;
    }

    // Passive decay
    exposure.value = exposure.value.saturating_sub(1);

    // Fire threshold events (one-shot, same pattern as faction tension thresholds)
    let current = exposure.value;
    for (threshold, text) in &events.0 {
        if current >= *threshold && !fired.0.contains(threshold) {
            fired.0.push(*threshold);
            log.push_at(time.turn, text.clone());
        }
    }
}

/// Compute the player's weighted evidence score.
/// Call this wherever a numeric total is needed (ending state, UI display).
pub fn evidence_score(evidence: &Evidence, rumor_count: usize) -> u32 {
    rumor_count as u32 + evidence.testimony * 3 + evidence.documents * 5
}
