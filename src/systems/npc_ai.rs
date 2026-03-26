/// NPC decision-making and movement each simulation tick.
use bevy::prelude::*;
use rand::Rng;

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

/// Spread rumors: when an NPC with Knowledge is at the same location as another,
/// they might share a rumor (only interesting ones).
pub fn spread_rumors(
    tick: Option<Res<TickEvent>>,
    mut npcs: Query<(&ActorName, &AtLocation, &mut Knowledge), With<Npc>>,
    time: Res<WorldTime>,
    mut log: ResMut<EventLog>,
) {
    if tick.is_none() {
        return;
    }

    let mut rng = rand::thread_rng();

    // Collect all (entity, location, rumor snapshot)
    let snapshot: Vec<(Entity, Entity, Vec<String>)> = npcs
        .iter()
        .map(|(_, at, know)| {
            // We can't use entity() in iter() directly — use a workaround
            (Entity::PLACEHOLDER, at.0, know.0.iter().map(|r| r.text.clone()).collect())
        })
        .collect();

    // For each NPC pair at the same location, maybe share a rumor
    let entities: Vec<Entity> = npcs.iter().map(|(_, at, _)| at.0).collect();
    let _ = (snapshot, entities);

    // Simplified: 15% chance each tick that some NPC "hears something" and we log it
    if rng.gen_bool(0.15) {
        let rumors = [
            "Someone overheard Aldric Voss talking about 'the next phase.'",
            "A cloaked figure was seen entering the back alleys after midnight.",
            "Tomas Reed looks like he hasn't slept in days.",
            "The dockmaster's ledger has been tampered with.",
            "Brega Halm was seen arguing with a Guild enforcer.",
            "Finn Crowe is offering information — for a price.",
            "Canon Thess held a secret meeting at the temple.",
        ];
        let rumor = rumors[rng.gen_range(0..rumors.len())];
        log.push_at(time.turn, format!("(Rumor) {}", rumor));
    }
}
