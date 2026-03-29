/// NPC decision-making and movement each simulation tick.
use bevy::prelude::*;
use rand::Rng;
use std::collections::HashMap;

use crate::components::*;
use crate::resources::{EventLog, TickEvent, WorldState, WorldTime};

/// Move NPCs according to their routine.
pub fn npc_movement(
    tick: Option<Res<TickEvent>>,
    mut npcs: Query<
        (&ActorName, &mut AtLocation, &mut NpcBehavior),
        With<Npc>,
    >,
    connections: Query<&Connections>,
    time: Res<WorldTime>,
    world: Res<WorldState>,
    mut log: ResMut<EventLog>,
) {
    if tick.is_none() {
        return;
    }

    let mut rng = rand::thread_rng();

    for (name, mut at_loc, mut behavior) in npcs.iter_mut() {
        if behavior.move_cooldown > 0 {
            behavior.move_cooldown -= 1;
            continue;
        }

        match behavior.routine.clone() {
            NpcRoutine::StayPut => {}

            NpcRoutine::Patrol(waypoints) => {
                if waypoints.is_empty() {
                    continue;
                }
                // Find next waypoint after current location
                let current = at_loc.0;
                let next = next_patrol_stop(&waypoints, current);
                if next != current {
                    let loc_name = world.location_name(next).unwrap_or("somewhere");
                    log.push_at(
                        time.turn,
                        format!("{} moves to {}.", name.0, loc_name),
                    );
                    at_loc.0 = next;
                    behavior.move_cooldown = rng.gen_range(1..=2);
                }
            }

            NpcRoutine::SeekGoal => {
                // Wander to a random connected location
                if let Ok(conns) = connections.get(at_loc.0) {
                    if !conns.0.is_empty() {
                        let dest = conns.0[rng.gen_range(0..conns.0.len())];
                        let loc_name = world.location_name(dest).unwrap_or("somewhere");
                        log.push_at(
                            time.turn,
                            format!("{} slips away to {}.", name.0, loc_name),
                        );
                        at_loc.0 = dest;
                        behavior.move_cooldown = rng.gen_range(2..=4);
                    }
                }
            }
        }
    }
}

fn next_patrol_stop(waypoints: &[Entity], current: Entity) -> Entity {
    let pos = waypoints.iter().position(|&e| e == current);
    match pos {
        Some(i) => waypoints[(i + 1) % waypoints.len()],
        None => waypoints[0],
    }
}

/// NPCs with Greedy trait accumulate a little wealth each tick when at the market or docks.
pub fn npc_wealth_tick(
    tick: Option<Res<TickEvent>>,
    mut npcs: Query<(&Traits, &AtLocation, &mut Wealth, &ActorName), With<Npc>>,
    location_names: Query<&ActorName, (With<LocationMarker>, Without<Npc>)>,
    mut log: ResMut<EventLog>,
    time: Res<WorldTime>,
) {
    if tick.is_none() {
        return;
    }

    let mut rng = rand::thread_rng();

    for (traits, at_loc, mut wealth, name) in npcs.iter_mut() {
        let loc_name = location_names
            .get(at_loc.0)
            .map(|n| n.0.as_str())
            .unwrap_or("");

        if traits.0.contains(&TraitKind::Greedy)
            && (loc_name.contains("Market") || loc_name.contains("Docks"))
        {
            let gain = rng.gen_range(1..=5);
            wealth.0 += gain;
            if rng.gen_bool(0.2) {
                log.push_at(
                    time.turn,
                    format!("{} closes a profitable deal (+{} coin).", name.0, gain),
                );
            }
        }
    }
}

/// Ambitious NPCs slowly grow their faction's power when at their faction's stronghold.
pub fn faction_power_tick(
    tick: Option<Res<TickEvent>>,
    npcs: Query<(&Traits, &AtLocation, &FactionMember), With<Npc>>,
    mut factions: Query<(&mut FactionPower, &ActorName), With<FactionMarker>>,
    location_names: Query<&ActorName, (With<LocationMarker>, Without<Npc>, Without<FactionMarker>)>,
) {
    if tick.is_none() {
        return;
    }

    let mut rng = rand::thread_rng();

    for (traits, at_loc, faction_member) in npcs.iter() {
        if !traits.0.contains(&TraitKind::Ambitious) {
            continue;
        }

        if let Ok((mut power, _fname)) = factions.get_mut(faction_member.0) {
            let loc_name = location_names
                .get(at_loc.0)
                .map(|n| n.0.as_str())
                .unwrap_or("");

            // Ambitious NPCs at their faction's "home" location grow power
            if loc_name.contains("Guild") || loc_name.contains("Temple") || loc_name.contains("Alley") {
                if rng.gen_bool(0.3) {
                    power.0 = (power.0 + 1).min(100);
                }
            }
        }
    }
}

/// Spread rumors: when an NPC with Knowledge is co-located with another NPC,
/// there's a chance each tick that one shares a rumor with the other.
/// Credibility degrades slightly on each hop (partial information).
pub fn spread_rumors(
    tick: Option<Res<TickEvent>>,
    mut npcs: Query<(Entity, &ActorName, &AtLocation, &mut Knowledge), With<Npc>>,
    time: Res<WorldTime>,
    mut log: ResMut<EventLog>,
) {
    if tick.is_none() {
        return;
    }

    let mut rng = rand::thread_rng();

    // Phase 1: snapshot location → [(npc_entity, rumors)] (immutable pass)
    let mut by_location: HashMap<Entity, Vec<(Entity, Vec<Rumor>)>> = HashMap::new();
    for (entity, _name, at_loc, knowledge) in npcs.iter() {
        by_location
            .entry(at_loc.0)
            .or_default()
            .push((entity, knowledge.0.clone()));
    }

    // Phase 2: for each location with 2+ NPCs, attempt a rumor transfer (mutable pass)
    for (_loc, present) in &by_location {
        if present.len() < 2 {
            continue;
        }
        // 30% chance of any exchange at this location this tick
        if !rng.gen_bool(0.30) {
            continue;
        }

        // Pick a donor: must have at least one rumor
        let donor_indices: Vec<usize> = present
            .iter()
            .enumerate()
            .filter(|(_, (_, rumors))| !rumors.is_empty())
            .map(|(i, _)| i)
            .collect();
        if donor_indices.is_empty() {
            continue;
        }
        let di = donor_indices[rng.gen_range(0..donor_indices.len())];
        let (donor_entity, donor_rumors) = &present[di];

        // Pick a random different NPC as recipient
        let other: Vec<usize> = (0..present.len()).filter(|&i| i != di).collect();
        let ri = other[rng.gen_range(0..other.len())];
        let (recipient_entity, recipient_rumors) = &present[ri];

        // Find a rumor the recipient doesn't already have
        let novel: Vec<&Rumor> = donor_rumors
            .iter()
            .filter(|dr| !recipient_rumors.iter().any(|rr| rr.text == dr.text))
            .collect();
        if novel.is_empty() {
            continue;
        }

        let picked = novel[rng.gen_range(0..novel.len())];
        let spread = Rumor {
            text: picked.text.clone(),
            // Credibility degrades 15% on each hop — rumors get less reliable as they travel
            credibility: ((picked.credibility as u32 * 85) / 100).max(10) as u8,
            turn_learned: time.turn,
        };

        if let Ok((_, name, _, mut knowledge)) = npcs.get_mut(*recipient_entity) {
            knowledge.0.push(spread);
            // Log sparsely so it doesn't flood — player won't always know who heard what
            if rng.gen_bool(0.25) {
                log.push_at(time.turn, format!("{} picks up a rumor.", name.0));
            }
        }
        let _ = donor_entity; // used in snapshot, not needed here
    }
}
