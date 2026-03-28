/// Player interaction with NPCs and the travel system.
use bevy::prelude::*;

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
    npc_query: &Query<(&ActorName, &Knowledge, &Goals, &Traits, &Wealth), (With<Npc>, Without<Player>)>,
    faction_query: &Query<(&ActorName, &FactionPower, &FactionTension, &Description), With<FactionMarker>>,
    world: &WorldState,
    log: &mut EventLog,
    time: &WorldTime,
) {
    match action {
        PlayerAction::AskRumor => {
            if let Some(npc_entity) = interaction.selected_npc {
                if let Ok((npc_name, knowledge, goals, traits, wealth)) = npc_query.get(npc_entity) {
                    let rumor = generate_rumor(npc_name, goals, traits, wealth);
                    log.push_at(time.turn, format!("{} says: \"{}\"", npc_name.0, rumor));
                    interaction.dialogue_lines.push(format!("\"{}\"", rumor));

                    // Player learns the rumor
                    if let Ok((_, mut player_know)) = player_query.single_mut() {
                        player_know.0.push(crate::components::Rumor {
                            text: rumor,
                            credibility: 60,
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
