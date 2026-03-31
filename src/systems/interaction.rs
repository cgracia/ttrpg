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

            // NPC-specific world-state actions
            let npc_id = world.npc_id_of(npc_entity).unwrap_or("");

            if npc_id == "lena" && !interaction.warned_lena {
                interaction.options.push(InteractionOption {
                    label: "Warn her about the Guild's plans".into(),
                    action: PlayerAction::WarnLena,
                });
            }

            if npc_id == "finn" {
                // PayForInfo: wealth guard checked in execute_player_action
                interaction.options.push(InteractionOption {
                    label: "Pay for information (10 coin)".into(),
                    action: PlayerAction::PayForInfo,
                });
            }

            // ShareWithThess: 2-rumor gate checked in execute_player_action
            if npc_id == "canon_thess" && !interaction.shared_with_thess {
                interaction.options.push(InteractionOption {
                    label: "Tell her what you know".into(),
                    action: PlayerAction::ShareWithThess,
                });
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
    player_query: &mut Query<(&mut AtLocation, &mut Knowledge, &mut Wealth), (With<Player>, Without<Npc>)>,
    npc_query: &Query<(&ActorName, &Knowledge, &Goals, &Traits, &Wealth, &AtLocation), (With<Npc>, Without<Player>)>,
    faction_query: &mut Query<(&ActorName, &mut FactionPower, &mut FactionTension, &Description), With<FactionMarker>>,
    front_query: &mut Query<&mut Front>,
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
                    if let Ok((_, mut player_know, _)) = player_query.single_mut() {
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
            if let Ok((mut at_loc, _, _)) = player_query.single_mut() {
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

        PlayerAction::WarnLena => {
            // Add +4 countdown to "The Guild's Gambit" or "The Iron Ledger" (whichever is active)
            let target_names = ["The Guild's Gambit", "The Iron Ledger"];
            for mut front in front_query.iter_mut() {
                if front.active && target_names.contains(&front.name.as_str()) {
                    front.countdown = front.countdown.saturating_add(4);
                    break;
                }
            }
            let msg = "You warn Lena about what you've heard. She listens carefully, then nods once. \
                       \"I'll be careful.\" The Guild's timeline slips — for now.";
            log.push_at(time.turn, msg.to_string());
            interaction.dialogue_lines.push(msg.to_string());
            interaction.warned_lena = true;
            // Remove the WarnLena option from the current options list
            interaction.options.retain(|o| !matches!(o.action, PlayerAction::WarnLena));
        }

        PlayerAction::PayForInfo => {
            // Guard: require player wealth >= 10
            let can_afford = player_query
                .single()
                .map(|(_, _, w)| w.0 >= 10)
                .unwrap_or(false);

            if !can_afford {
                interaction.dialogue_lines.push(
                    "You don't have enough coin.".to_string(),
                );
                return;
            }

            // Deduct wealth
            if let Ok((_, mut player_know, mut wealth)) = player_query.single_mut() {
                wealth.0 -= 10;

                // Pick a rumor Finn knows; prefer one the player doesn't have yet
                let rumor = if let Some(npc_entity) = interaction.selected_npc {
                    npc_query.get(npc_entity).ok().and_then(|(_, knowledge, _, _, _, _)| {
                        let unknown: Vec<&crate::components::Rumor> = knowledge.0.iter()
                            .filter(|r| !player_know.0.iter().any(|pr| pr.text == r.text))
                            .collect();
                        if !unknown.is_empty() {
                            let mut rng = rand::thread_rng();
                            Some(unknown[rng.gen_range(0..unknown.len())].clone())
                        } else if !knowledge.0.is_empty() {
                            let mut rng = rand::thread_rng();
                            Some(knowledge.0[rng.gen_range(0..knowledge.0.len())].clone())
                        } else {
                            None
                        }
                    })
                } else {
                    None
                };

                if let Some(r) = rumor {
                    let msg = format!("Finn pockets the coin and leans in close. \"{}\"", r.text);
                    log.push_at(time.turn, msg.clone());
                    interaction.dialogue_lines.push(msg);
                    player_know.0.push(r);
                } else {
                    let msg = "Finn pockets the coin and shrugs. \"Nothing new to tell, friend.\"";
                    log.push_at(time.turn, msg.to_string());
                    interaction.dialogue_lines.push(msg.to_string());
                }
            }

            // 33% chance Shadows tension +8
            let mut rng = rand::thread_rng();
            if rng.gen_bool(0.33) {
                if let Some(shadows_entity) = world.faction_entity("shadows") {
                    if let Ok((_, _, mut tension, _)) = faction_query.get_mut(shadows_entity) {
                        tension.0 = (tension.0 + 8).min(100);
                    }
                }
                let notice_msg = "Finn's eyes drift toward the door as you leave. Someone noticed you were asking.";
                log.push_at(time.turn, notice_msg.to_string());
                interaction.dialogue_lines.push(notice_msg.to_string());
            }
        }

        PlayerAction::ShareWithThess => {
            // Guard: player must have at least 2 rumors
            let rumor_count = player_query.single().map(|(_, k, _)| k.0.len()).unwrap_or(0);
            if rumor_count < 2 {
                interaction.dialogue_lines.push(
                    "Canon Thess listens patiently. \"Come back when you have more to tell me.\"".to_string(),
                );
                return;
            }

            // Order +8 power
            if let Some(order_entity) = world.faction_entity("order") {
                if let Ok((_, mut power, _, _)) = faction_query.get_mut(order_entity) {
                    power.0 = (power.0 + 8).min(100);
                }
            }

            // Guild +5 tension
            if let Some(guild_entity) = world.faction_entity("guild") {
                if let Ok((_, _, mut tension, _)) = faction_query.get_mut(guild_entity) {
                    tension.0 = (tension.0 + 5).min(100);
                }
            }

            // "Whispers from the Mine" front countdown -4
            for mut front in front_query.iter_mut() {
                if front.active && front.name == "Whispers from the Mine" {
                    front.countdown = front.countdown.saturating_sub(4);
                    break;
                }
            }

            let msg = "You share what you've learned with Canon Thess. She listens without speaking, \
                       then: \"Then the Accord must speak.\" The Order stirs. \
                       The mine's secret draws closer to daylight.";
            log.push_at(time.turn, msg.to_string());
            interaction.dialogue_lines.push(msg.to_string());
            interaction.shared_with_thess = true;
            interaction.options.retain(|o| !matches!(o.action, PlayerAction::ShareWithThess));
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
