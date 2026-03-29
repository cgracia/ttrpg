/// Data loading from RON files and world spawning.
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::*;
use crate::resources::*;

// ── RON-serializable templates ────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LocationTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub x: f32,
    pub y: f32,
    pub connections: Vec<String>, // ids
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FactionTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub starting_power: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NpcTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub faction: String,
    pub location: String,
    pub stats: StatsTemplate,
    pub traits: Vec<String>,
    pub goals: Vec<String>,
    pub wealth: i32,
    pub routine: String,
    pub patrol: Vec<String>,
    /// Rumors this NPC knows at world start. Empty = no starting knowledge.
    #[serde(default)]
    pub starter_rumors: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StatsTemplate {
    pub strength: i32,
    pub charisma: i32,
    pub cunning: i32,
    pub resolve: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FrontTemplate {
    pub name: String,
    pub description: String,
    pub starting_countdown: u32,
    pub stages: Vec<FrontStageTemplate>,
    /// Optional name of a front to activate when this one resolves.
    #[serde(default)]
    pub successor_front: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FrontStageTemplate {
    pub description: String,
    pub event_log_entry: String,
    pub countdown_turns: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WorldData {
    pub locations: Vec<LocationTemplate>,
    pub factions: Vec<FactionTemplate>,
    pub npcs: Vec<NpcTemplate>,
    pub fronts: Vec<FrontTemplate>,
}

// ── Spawn world from data ─────────────────────────────────────────────────────

pub fn spawn_world(
    commands: &mut Commands,
    world_data: &WorldData,
    world_state: &mut WorldState,
    event_log: &mut EventLog,
) {
    // Spawn factions
    let mut faction_map: std::collections::HashMap<String, Entity> =
        std::collections::HashMap::new();
    for ft in &world_data.factions {
        let e = commands
            .spawn((
                FactionMarker,
                ActorName(ft.name.clone()),
                Description(ft.description.clone()),
                FactionPower(ft.starting_power),
                FactionTension(0),
            ))
            .id();
        faction_map.insert(ft.id.clone(), e);
        world_state.factions.push((ft.id.clone(), ft.name.clone(), e));
    }

    // Spawn locations (first pass — no connections yet)
    let mut location_map: std::collections::HashMap<String, Entity> =
        std::collections::HashMap::new();
    for lt in &world_data.locations {
        let e = commands
            .spawn((
                LocationMarker,
                ActorName(lt.name.clone()),
                Description(lt.description.clone()),
                MapPos(Vec2::new(lt.x, lt.y)),
                Connections::default(),
            ))
            .id();
        location_map.insert(lt.id.clone(), e);
        world_state.locations.push((lt.id.clone(), lt.name.clone(), e));
    }

    // Second pass — wire connections
    for lt in &world_data.locations {
        if let Some(&loc_entity) = location_map.get(&lt.id) {
            let conn_entities: Vec<Entity> = lt
                .connections
                .iter()
                .filter_map(|cid| location_map.get(cid).copied())
                .collect();
            commands.entity(loc_entity).insert(Connections(conn_entities));
        }
    }

    // Spawn NPCs
    for nt in &world_data.npcs {
        let faction_entity = nt
            .faction
            .as_str()
            .ne("")
            .then(|| faction_map.get(&nt.faction).copied())
            .flatten();

        let location_entity = location_map
            .get(&nt.location)
            .copied()
            .unwrap_or_else(|| {
                world_state.locations.first().map(|l| l.2).unwrap()
            });

        let traits = parse_traits(&nt.traits);
        let goals = parse_goals(&nt.goals);

        let routine = match nt.routine.as_str() {
            "stay" => NpcRoutine::StayPut,
            "patrol" => {
                let patrol_entities: Vec<Entity> = nt
                    .patrol
                    .iter()
                    .filter_map(|pid| location_map.get(pid).copied())
                    .collect();
                NpcRoutine::Patrol(patrol_entities)
            }
            _ => NpcRoutine::SeekGoal,
        };

        let starting_knowledge = Knowledge(
            nt.starter_rumors
                .iter()
                .map(|text| Rumor {
                    text: text.clone(),
                    credibility: 80,
                    turn_learned: 0,
                })
                .collect(),
        );

        let mut entity_cmd = commands.spawn((
            Npc,
            ActorName(nt.name.clone()),
            Description(nt.description.clone()),
            Stats {
                strength: nt.stats.strength,
                charisma: nt.stats.charisma,
                cunning: nt.stats.cunning,
                resolve: nt.stats.resolve,
            },
            Traits(traits),
            Goals(goals),
            Relationships::default(),
            starting_knowledge,
            AtLocation(location_entity),
            Wealth(nt.wealth),
            NpcBehavior { routine, move_cooldown: 0 },
        ));

        if let Some(fe) = faction_entity {
            entity_cmd.insert(FactionMember(fe));
        }

        let npc_entity = entity_cmd.id();
        world_state
            .npcs
            .push((nt.id.clone(), nt.name.clone(), npc_entity));
    }

    // Spawn fronts
    for ft in &world_data.fronts {
        let stages: Vec<FrontStage> = ft
            .stages
            .iter()
            .map(|s| FrontStage {
                description: s.description.clone(),
                event_log_entry: s.event_log_entry.clone(),
                countdown_turns: s.countdown_turns,
            })
            .collect();

        commands.spawn((
            Front {
                name: ft.name.clone(),
                description: ft.description.clone(),
                stage: 0,
                countdown: ft.starting_countdown,
                active: true,
                starting_countdown: ft.starting_countdown,
                successor_front: ft.successor_front.clone(),
            },
            FrontStages(stages),
        ));
    }

    // Spawn player at first location
    if let Some(&start_loc) = location_map.get("town_square") {
        let player_start = start_loc;
        commands.spawn((
            Player,
            ActorName("You".to_string()),
            Stats::default(),
            Knowledge::default(),
            Wealth(20),
            AtLocation(player_start),
        ));
        world_state.player_location = Some(player_start);
        event_log.push("You arrive in Ashenveil. The town is tense.".to_string());
    }
}

// ── Trait / Goal parsers ──────────────────────────────────────────────────────

fn parse_traits(raw: &[String]) -> Vec<TraitKind> {
    raw.iter()
        .filter_map(|s| match s.as_str() {
            "greedy" => Some(TraitKind::Greedy),
            "loyal" => Some(TraitKind::Loyal),
            "ambitious" => Some(TraitKind::Ambitious),
            "cowardly" => Some(TraitKind::Cowardly),
            "ruthless" => Some(TraitKind::Ruthless),
            "cautious" => Some(TraitKind::Cautious),
            "idealistic" => Some(TraitKind::Idealistic),
            _ => None,
        })
        .collect()
}

fn parse_goals(raw: &[String]) -> Vec<Goal> {
    raw.iter()
        .enumerate()
        .filter_map(|(i, s)| {
            let kind = match s.as_str() {
                "wealth" => Some(GoalKind::AccumulateWealth),
                "influence" => Some(GoalKind::GainInfluence),
                "protect_faction" => Some(GoalKind::ProtectFaction),
                "destroy_rival" => Some(GoalKind::DestroyRival),
                "survive" => Some(GoalKind::SurviveConflict),
                "knowledge" => Some(GoalKind::SeekKnowledge),
                "order" => Some(GoalKind::MaintainOrder),
                _ => None,
            };
            kind.map(|k| Goal {
                kind: k,
                priority: (10u8.saturating_sub(i as u8)).max(1),
                progress: 0,
            })
        })
        .collect()
}

// ── Hardcoded world data ──────────────────────────────────────────────────────
// Loaded inline instead of from files to keep the prototype self-contained.

pub fn build_world_data() -> WorldData {
    WorldData {
        locations: vec![
            LocationTemplate {
                id: "town_square".into(),
                name: "Town Square".into(),
                description: "The central plaza of Ashenveil. A well stands at its heart, surrounded by notice boards covered in proclamations.".into(),
                x: 0.0, y: 0.0,
                connections: vec!["tavern".into(), "guild_hall".into(), "market".into(), "temple".into()],
            },
            LocationTemplate {
                id: "tavern".into(),
                name: "The Salted Crow".into(),
                description: "A smoky tavern frequented by miners and merchants. Gossip flows freely here.".into(),
                x: -200.0, y: 80.0,
                connections: vec!["town_square".into(), "back_alley".into()],
            },
            LocationTemplate {
                id: "guild_hall".into(),
                name: "Merchant Guild Hall".into(),
                description: "The power center of the Merchant Guild. Ledgers and contracts fill every shelf.".into(),
                x: 180.0, y: 100.0,
                connections: vec!["town_square".into(), "market".into()],
            },
            LocationTemplate {
                id: "market".into(),
                name: "Market District".into(),
                description: "Stalls selling everything from grain to contraband. The Guild's informants watch the crowds.".into(),
                x: 100.0, y: -120.0,
                connections: vec!["town_square".into(), "guild_hall".into(), "docks".into()],
            },
            LocationTemplate {
                id: "temple".into(),
                name: "Temple of Accord".into(),
                description: "A modest temple serving as neutral ground. The Order of Accord holds mediation here.".into(),
                x: -150.0, y: -100.0,
                connections: vec!["town_square".into(), "back_alley".into()],
            },
            LocationTemplate {
                id: "back_alley".into(),
                name: "Back Alleys".into(),
                description: "Narrow streets where debts are collected and secrets are sold. The Shadows operate here.".into(),
                x: -220.0, y: -60.0,
                connections: vec!["tavern".into(), "temple".into(), "docks".into()],
            },
            LocationTemplate {
                id: "docks".into(),
                name: "River Docks".into(),
                description: "Cargo barges and fishing boats. Smuggling is an open secret.".into(),
                x: 50.0, y: -250.0,
                connections: vec!["market".into(), "back_alley".into()],
            },
            LocationTemplate {
                id: "watchtower".into(),
                name: "North Watchtower".into(),
                description: "A crumbling tower at the town's edge. Whoever holds it has a view of all roads in and out.".into(),
                x: 30.0, y: 200.0,
                connections: vec!["town_square".into()],
            },
        ],
        factions: vec![
            FactionTemplate {
                id: "guild".into(),
                name: "Merchant Guild".into(),
                description: "Controls trade and credit in Ashenveil. Ruthlessly pragmatic.".into(),
                starting_power: 65,
            },
            FactionTemplate {
                id: "order".into(),
                name: "Order of Accord".into(),
                description: "Peacekeepers and mediators. Losing influence as tensions rise.".into(),
                starting_power: 40,
            },
            FactionTemplate {
                id: "shadows".into(),
                name: "The Shadows".into(),
                description: "A criminal network operating through intimidation and blackmail.".into(),
                starting_power: 45,
            },
        ],
        npcs: vec![
            NpcTemplate {
                id: "aldric".into(),
                name: "Aldric Voss".into(),
                description: "Guildmaster of the Merchant Guild. Powerful and calculating.".into(),
                faction: "guild".into(),
                location: "guild_hall".into(),
                stats: StatsTemplate { strength: 3, charisma: 8, cunning: 9, resolve: 7 },
                traits: vec!["ambitious".into(), "ruthless".into()],
                goals: vec!["influence".into(), "wealth".into()],
                wealth: 200,
                routine: "stay".into(),
                patrol: vec![],
                starter_rumors: vec![],
            },
            NpcTemplate {
                id: "mira".into(),
                name: "Mira Dent".into(),
                description: "Guild broker and Aldric's right hand. Handles the dirty details.".into(),
                faction: "guild".into(),
                location: "market".into(),
                stats: StatsTemplate { strength: 4, charisma: 7, cunning: 8, resolve: 6 },
                traits: vec!["greedy".into(), "cautious".into()],
                goals: vec!["wealth".into(), "influence".into()],
                wealth: 80,
                routine: "patrol".into(),
                patrol: vec!["market".into(), "guild_hall".into(), "docks".into()],
                starter_rumors: vec![],
            },
            NpcTemplate {
                id: "canon_thess".into(),
                name: "Canon Thess".into(),
                description: "Head of the Order of Accord. Idealistic but increasingly desperate.".into(),
                faction: "order".into(),
                location: "temple".into(),
                stats: StatsTemplate { strength: 3, charisma: 9, cunning: 5, resolve: 8 },
                traits: vec!["idealistic".into(), "loyal".into()],
                goals: vec!["order".into(), "protect_faction".into()],
                wealth: 15,
                routine: "patrol".into(),
                patrol: vec!["temple".into(), "town_square".into()],
                starter_rumors: vec![],
            },
            NpcTemplate {
                id: "brega".into(),
                name: "Brega Halm".into(),
                description: "Order enforcer. Loyal but conflicted about the Order's weakening position.".into(),
                faction: "order".into(),
                location: "town_square".into(),
                stats: StatsTemplate { strength: 8, charisma: 5, cunning: 4, resolve: 7 },
                traits: vec!["loyal".into(), "cautious".into()],
                goals: vec!["protect_faction".into(), "survive".into()],
                wealth: 10,
                routine: "patrol".into(),
                patrol: vec!["town_square".into(), "temple".into(), "watchtower".into()],
                starter_rumors: vec![],
            },
            NpcTemplate {
                id: "sable".into(),
                name: "Sable".into(),
                description: "Leader of the Shadows. Never seen in daylight.".into(),
                faction: "shadows".into(),
                location: "back_alley".into(),
                stats: StatsTemplate { strength: 6, charisma: 7, cunning: 10, resolve: 8 },
                traits: vec!["ruthless".into(), "ambitious".into()],
                goals: vec!["influence".into(), "destroy_rival".into()],
                wealth: 120,
                routine: "stay".into(),
                patrol: vec![],
                starter_rumors: vec![],
            },
            NpcTemplate {
                id: "finn".into(),
                name: "Finn Crowe".into(),
                description: "Shadows runner and informant. Sells information to anyone who can pay.".into(),
                faction: "shadows".into(),
                location: "tavern".into(),
                stats: StatsTemplate { strength: 5, charisma: 6, cunning: 7, resolve: 4 },
                traits: vec!["greedy".into(), "cowardly".into()],
                goals: vec!["wealth".into(), "survive".into()],
                wealth: 30,
                routine: "patrol".into(),
                patrol: vec!["tavern".into(), "back_alley".into(), "docks".into(), "market".into()],
                starter_rumors: vec![
                    "The Guild is buying up dock leases. Lena's been paid to look the other way.".into(),
                    "Sable has eyes on someone new in town. Asking questions gets people hurt.".into(),
                ],
            },
            NpcTemplate {
                id: "lena".into(),
                name: "Lena Marsh".into(),
                description: "Dockmaster. Plays all sides to keep her operation running.".into(),
                faction: "".into(),
                location: "docks".into(),
                stats: StatsTemplate { strength: 6, charisma: 7, cunning: 7, resolve: 6 },
                traits: vec!["cautious".into(), "greedy".into()],
                goals: vec!["wealth".into(), "survive".into()],
                wealth: 60,
                routine: "stay".into(),
                patrol: vec![],
                starter_rumors: vec![
                    "Three barges arrived last week with manifests that don't match their cargo. Guild business.".into(),
                ],
            },
            NpcTemplate {
                id: "otto".into(),
                name: "Otto Brix".into(),
                description: "Veteran tavern keeper. Remembers when things were simpler.".into(),
                faction: "".into(),
                location: "tavern".into(),
                stats: StatsTemplate { strength: 7, charisma: 6, cunning: 5, resolve: 8 },
                traits: vec!["loyal".into(), "cautious".into()],
                goals: vec!["order".into(), "survive".into()],
                wealth: 40,
                routine: "stay".into(),
                patrol: vec![],
                starter_rumors: vec![
                    "Miners who went to look at the old shaft never came back. Four of them. Nobody talks about it.".into(),
                    "The Order's been losing tithing money. Canon Thess covers for it but the books don't lie.".into(),
                ],
            },
            NpcTemplate {
                id: "vex".into(),
                name: "Vex".into(),
                description: "Mysterious stranger. Has been asking too many questions about the old mine.".into(),
                faction: "".into(),
                location: "tavern".into(),
                stats: StatsTemplate { strength: 7, charisma: 5, cunning: 8, resolve: 7 },
                traits: vec!["ambitious".into(), "cautious".into()],
                goals: vec!["knowledge".into(), "wealth".into()],
                wealth: 25,
                routine: "patrol".into(),
                patrol: vec!["tavern".into(), "town_square".into(), "docks".into()],
                starter_rumors: vec![
                    "Something was found in the mine collapse. Not ore. The Guild sealed it off immediately.".into(),
                ],
            },
            NpcTemplate {
                id: "tomas".into(),
                name: "Tomas Reed".into(),
                description: "Young guild clerk. Discovers things he shouldn't. Growing fearful.".into(),
                faction: "guild".into(),
                location: "guild_hall".into(),
                stats: StatsTemplate { strength: 3, charisma: 5, cunning: 6, resolve: 3 },
                traits: vec!["cowardly".into(), "idealistic".into()],
                goals: vec!["survive".into(), "knowledge".into()],
                wealth: 12,
                routine: "patrol".into(),
                patrol: vec!["guild_hall".into(), "town_square".into(), "tavern".into()],
                starter_rumors: vec![
                    "Aldric authorized a payment to someone outside the Guild. The ledger entry was altered the next day.".into(),
                ],
            },
        ],
        fronts: vec![
            FrontTemplate {
                name: "The Guild's Gambit".into(),
                description: "The Merchant Guild is executing a slow takeover of Ashenveil's docks and supply chains, squeezing out the Order and the Shadows.".into(),
                starting_countdown: 4,
                successor_front: None, // TODO TASK-009: worldbuild to wire successor
                stages: vec![
                    FrontStageTemplate {
                        description: "The Guild begins buying up dock leases.".into(),
                        event_log_entry: "Dock workers report that the Guild has purchased two more berth leases. Lena Marsh looks nervous.".into(),
                        countdown_turns: 5,
                    },
                    FrontStageTemplate {
                        description: "Guild enforcers appear in the market. Prices spike.".into(),
                        event_log_entry: "Armed Guild agents are now stationed in the market. Merchants who refuse protection are 'having accidents.'".into(),
                        countdown_turns: 5,
                    },
                    FrontStageTemplate {
                        description: "The Shadows retaliate. A Guild warehouse burns.".into(),
                        event_log_entry: "A fire broke out at the Guild's riverside warehouse last night. The Shadows are suspected. Aldric Voss is furious.".into(),
                        countdown_turns: 4,
                    },
                    FrontStageTemplate {
                        description: "Open conflict erupts. The Order tries to intervene.".into(),
                        event_log_entry: "Street fighting between Guild agents and Shadow operatives. Canon Thess attempts mediation and is ignored by both sides.".into(),
                        countdown_turns: 4,
                    },
                    FrontStageTemplate {
                        description: "One side collapses or a fragile truce is struck.".into(),
                        event_log_entry: "The conflict reaches a breaking point. Ashenveil will never be the same.".into(),
                        countdown_turns: 0,
                    },
                ],
            },
            FrontTemplate {
                name: "Whispers from the Mine".into(),
                description: "Something was discovered in the old collapsed mine east of town. People are starting to disappear.".into(),
                starting_countdown: 7,
                successor_front: None, // TODO TASK-009: worldbuild to wire successor
                stages: vec![
                    FrontStageTemplate {
                        description: "Rumors of strange lights near the mine.".into(),
                        event_log_entry: "Miners at the tavern whisper about strange lights seen at the old collapsed mine. Three men who went to investigate haven't returned.".into(),
                        countdown_turns: 6,
                    },
                    FrontStageTemplate {
                        description: "A body is found with no explanation.".into(),
                        event_log_entry: "A body was recovered from the river — a miner who'd been asking about the mine. No apparent cause of death.".into(),
                        countdown_turns: 5,
                    },
                    FrontStageTemplate {
                        description: "Vex is connected to the mine. Someone wants them silenced.".into(),
                        event_log_entry: "Vex narrowly escapes an ambush in the back alleys. They carry documents no one should have.".into(),
                        countdown_turns: 0,
                    },
                ],
            },
        ],
    }
}
