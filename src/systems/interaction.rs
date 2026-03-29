/// Player interaction with NPCs and the travel system.
use bevy::prelude::*;
use rand::Rng;

use crate::components::*;
use crate::resources::*;

/// Build interaction options for the currently selected NPC.
pub fn build_interaction(
    mut interaction: ResMut<InteractionState>,
    mode: Res<GameMode>,
    npcs: Query<(&ActorName, &Description, &Goals, &Traits, &AtLocation, Option<&FactionMember>), With<Npc>>,
    factions: Query<(&ActorName, &FactionPower, &FactionTension), With<FactionMarker>>,
    world: Res<WorldState>,
) {
    if let GameMode::Interaction(npc_entity) = *mode {
        if let Ok((name, desc, goals, traits, _at, faction_opt)) = npcs.get(npc_entity) {
            // Only rebuild if NPC changed
            if interaction.selected_npc == Some(npc_entity) {
                return;
            }

            interaction.selected_npc = Some(npc_entity);
            interaction.dialogue_lines.clear();
            interaction.options.clear();

            // Opening line based on traits
            let greeting = if traits.0.contains(&TraitKind::Greedy) {
                format!("{} eyes you appraisingly. \"What can I do for you — profitably?\"", name.0)
            } else if traits.0.contains(&TraitKind::Ruthless) {
                format!("{} stares coldly. \"Make it quick.\"", name.0)
            } else if traits.0.contains(&TraitKind::Idealistic) {
                format!("{} smiles warmly. \"Ah, a new face. Ashenveil needs more honest folk.\"", name.0)
            } else if traits.0.contains(&TraitKind::Cowardly) {
                format!("{} glances around nervously. \"I... don't really want trouble.\"", name.0)
            } else {
                format!("{} nods. \"{}\"", name.0, desc.0)
            };

            interaction.dialogue_lines.push(greeting);

            // Show top goal as a hint
            if let Some(goal) = goals.0.first() {
                interaction.dialogue_lines.push(format!(
                    "  [Seems focused on: {}]",
                    goal.kind.label()
                ));
            }

            // Options
            interaction.options.push(InteractionOption {
                label: "Ask about local rumors".into(),
                action: PlayerAction::AskRumor,
            });

            if let Some(faction_member) = faction_opt {
                if let Ok((fname, power, tension)) = factions.get(faction_member.0) {
                    interaction.options.push(InteractionOption {
                        label: format!("Ask about the {} (Power: {}, Tension: {})", fname.0, power.0, tension.0),
                        action: PlayerAction::AskAboutFaction(faction_member.0),
                    });
                }
            }

            interaction.options.push(InteractionOption {
                label: "Leave".into(),
                action: PlayerAction::LeaveConversation,
            });
        }
    }
}

/// Build travel options for the player's current location.
pub fn build_travel_options(
    mut interaction: ResMut<InteractionState>,
    mode: Res<GameMode>,
    player: Query<&AtLocation, With<Player>>,
    connections: Query<&Connections>,
    loc_names: Query<&ActorName, With<LocationMarker>>,
    world: Res<WorldState>,
) {
    if *mode != GameMode::Travel {
        return;
    }

    if let Ok(at) = player.single() {
        interaction.dialogue_lines.clear();
        interaction.options.clear();
        interaction.selected_npc = None;

        interaction.dialogue_lines.push("Where do you want to go?".into());

        if let Ok(conns) = connections.get(at.0) {
            for &dest in &conns.0 {
                if let Ok(name) = loc_names.get(dest) {
                    interaction.options.push(InteractionOption {
                        label: format!("→ {}", name.0),
                        action: PlayerAction::TravelTo(dest),
                    });
                }
            }
        }

        // Location-specific actions
        if world.location_entity("watchtower") == Some(at.0) {
            interaction.options.push(InteractionOption {
                label: "Survey the roads below".into(),
                action: PlayerAction::Scout,
            });
        }

        interaction.options.push(InteractionOption {
            label: "Stay here".into(),
            action: PlayerAction::LeaveConversation,
        });
    }
}

/// Execute an interaction option the player chose (called from UI).
pub fn execute_player_action(
    action: PlayerAction,
    commands: &mut Commands,
    mode: &mut GameMode,
    interaction: &mut InteractionState,
    player_query: &mut Query<(&mut AtLocation, &mut Knowledge), (With<Player>, Without<Npc>)>,
    npc_query: &Query<(&ActorName, &Knowledge, &Goals, &Traits, &Wealth, &AtLocation), (With<Npc>, Without<Player>)>,
    faction_query: &Query<(&ActorName, &FactionPower, &FactionTension, &Description), With<FactionMarker>>,
    world: &WorldState,
    log: &mut EventLog,
    time: &WorldTime,
) {
    match action {
        PlayerAction::AskRumor => {
            if let Some(npc_entity) = interaction.selected_npc {
                if let Ok((npc_name, knowledge, goals, traits, wealth, _at)) = npc_query.get(npc_entity) {
                    // Prefer a rumor the NPC actually knows; fall back to trait-based generation
                    let (rumor_text, credibility) = if !knowledge.0.is_empty() {
                        let mut rng = rand::thread_rng();
                        let r = &knowledge.0[rng.gen_range(0..knowledge.0.len())];
                        (r.text.clone(), r.credibility)
                    } else {
                        (generate_rumor(npc_name, goals, traits, wealth), 60u8)
                    };

                    log.push_at(time.turn, format!("{} says: \"{}\"", npc_name.0, rumor_text));
                    interaction.dialogue_lines.push(format!("\"{}\"", rumor_text));

                    // Player learns the rumor
                    if let Ok((_, mut player_know)) = player_query.single_mut() {
                        player_know.0.push(crate::components::Rumor {
                            text: rumor_text,
                            credibility,
                            turn_learned: time.turn,
                        });
                    }
                }
            }
        }

        PlayerAction::AskAboutFaction(faction_entity) => {
            if let Ok((fname, power, tension, desc)) = faction_query.get(faction_entity) {
                let status = if tension.0 > 70 {
                    "on a war footing"
                } else if tension.0 > 40 {
                    "restless"
                } else {
                    "stable"
                };
                let line = format!(
                    "The {}? {} They seem {}. Power: {}/100.",
                    fname.0, desc.0, status, power.0
                );
                interaction.dialogue_lines.push(line.clone());
                log.push_at(time.turn, line);
            }
        }

        PlayerAction::LeaveConversation => {
            *mode = GameMode::Exploration;
            interaction.dialogue_lines.clear();
            interaction.options.clear();
            interaction.selected_npc = None;
        }

        PlayerAction::TravelTo(dest) => {
            if let Ok((mut at_loc, _)) = player_query.single_mut() {
                let dest_name = world.location_name(dest).unwrap_or("unknown");
                log.push_at(time.turn, format!("You travel to {}.", dest_name));
                at_loc.0 = dest;
            }
            *mode = GameMode::Exploration;
            interaction.dialogue_lines.clear();
            interaction.options.clear();
        }

        PlayerAction::Scout => {
            // Outdoor locations visible from the Watchtower
            let outdoor_ids = ["town_square", "market", "docks", "back_alley", "watchtower"];
            let outdoor_entities: Vec<Entity> = outdoor_ids
                .iter()
                .filter_map(|id| world.location_entity(id))
                .collect();

            let mut sightings: Vec<String> = npc_query
                .iter()
                .filter(|(_, _, _, _, _, at)| outdoor_entities.contains(&at.0))
                .take(3)
                .map(|(name, _, _, _, _, at)| {
                    let loc = world.location_name(at.0).unwrap_or("somewhere");
                    format!("{} near the {}", name.0, loc)
                })
                .collect();

            let msg = if sightings.is_empty() {
                "The roads below are empty. Whatever's happening in this town, it's happening indoors.".to_string()
            } else {
                format!(
                    "From this height you can see who moves through the open streets. {}.",
                    sightings.join(". ")
                )
            };

            log.push_at(time.turn, msg.clone());
            interaction.dialogue_lines.push(msg);
        }
    }

    let _ = commands;
}

fn generate_rumor(
    name: &ActorName,
    goals: &Goals,
    traits: &Traits,
    wealth: &Wealth,
) -> String {
    if traits.0.contains(&TraitKind::Greedy) && wealth.0 > 50 {
        return "Money moves through this town in ways that don't show up in any ledger. \
                Follow the coin if you want the truth."
            .into();
    }
    if traits.0.contains(&TraitKind::Ruthless) {
        return "Someone is cleaning up loose ends. I'd be careful about what you look into."
            .into();
    }
    if traits.0.contains(&TraitKind::Cowardly) {
        return "I don't know anything. Really. Please don't ask me again."
            .into();
    }
    if let Some(goal) = goals.0.first() {
        match goal.kind {
            GoalKind::GainInfluence => {
                "The balance of power here is shifting. Whoever controls the docks \
                 controls Ashenveil."
                    .into()
            }
            GoalKind::MaintainOrder => {
                "The Order is struggling. Without support they won't be able to keep \
                 things from boiling over."
                    .into()
            }
            GoalKind::DestroyRival => {
                format!(
                    "{} lowers their voice. \"There are people in this town who will \
                     not stop until their enemies are gone. All of them.\"",
                    name.0
                )
            }
            GoalKind::SeekKnowledge => {
                "Something happened at the old mine. The Guild is paying good money \
                 to keep it quiet."
                    .into()
            }
            _ => "Keep your head down and your purse close. That's my advice.".into(),
        }
    } else {
        "Nothing special to report. Quiet town, mostly.".into()
    }
}
