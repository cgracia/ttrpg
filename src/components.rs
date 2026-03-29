use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// ── Marker components ────────────────────────────────────────────────────────

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Npc;

#[derive(Component)]
pub struct LocationMarker;

#[derive(Component)]
pub struct FactionMarker;

// ── Identity ─────────────────────────────────────────────────────────────────

#[derive(Component, Clone, Debug)]
pub struct ActorName(pub String);

#[derive(Component, Clone, Debug)]
pub struct Description(pub String);

// ── Stats ────────────────────────────────────────────────────────────────────

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Stats {
    pub strength: i32,
    pub charisma: i32,
    pub cunning: i32,
    pub resolve: i32,
}

impl Default for Stats {
    fn default() -> Self {
        Stats { strength: 5, charisma: 5, cunning: 5, resolve: 5 }
    }
}

// ── Traits ───────────────────────────────────────────────────────────────────

#[derive(Component, Clone, Debug, Default)]
pub struct Traits(pub Vec<TraitKind>);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TraitKind {
    Greedy,
    Loyal,
    Ambitious,
    Cowardly,
    Ruthless,
    Cautious,
    Idealistic,
}

impl TraitKind {
    pub fn label(&self) -> &'static str {
        match self {
            TraitKind::Greedy => "Greedy",
            TraitKind::Loyal => "Loyal",
            TraitKind::Ambitious => "Ambitious",
            TraitKind::Cowardly => "Cowardly",
            TraitKind::Ruthless => "Ruthless",
            TraitKind::Cautious => "Cautious",
            TraitKind::Idealistic => "Idealistic",
        }
    }
}

// ── Goals ────────────────────────────────────────────────────────────────────

#[derive(Component, Clone, Debug, Default)]
pub struct Goals(pub Vec<Goal>);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Goal {
    pub kind: GoalKind,
    pub priority: u8,
    pub progress: u8, // 0–100
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum GoalKind {
    AccumulateWealth,
    GainInfluence,
    ProtectFaction,
    DestroyRival,
    SurviveConflict,
    SeekKnowledge,
    MaintainOrder,
}

impl GoalKind {
    pub fn label(&self) -> &'static str {
        match self {
            GoalKind::AccumulateWealth => "Accumulate Wealth",
            GoalKind::GainInfluence => "Gain Influence",
            GoalKind::ProtectFaction => "Protect Faction",
            GoalKind::DestroyRival => "Destroy Rival",
            GoalKind::SurviveConflict => "Survive Conflict",
            GoalKind::SeekKnowledge => "Seek Knowledge",
            GoalKind::MaintainOrder => "Maintain Order",
        }
    }
}

// ── Relationships ─────────────────────────────────────────────────────────────

#[derive(Component, Clone, Debug, Default)]
pub struct Relationships(pub Vec<Relationship>);

#[derive(Clone, Debug)]
pub struct Relationship {
    pub target: Entity,
    pub target_name: String,
    pub disposition: i32, // -100 hostile .. +100 devoted
    pub rel_type: RelType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RelType {
    Ally,
    Rival,
    Neutral,
    Employer,
    Contact,
}

// ── Knowledge ────────────────────────────────────────────────────────────────

#[derive(Component, Clone, Debug, Default)]
pub struct Knowledge(pub Vec<Rumor>);

#[derive(Clone, Debug)]
pub struct Rumor {
    pub text: String,
    pub credibility: u8, // 0–100
    pub turn_learned: u32,
}

// ── Location ─────────────────────────────────────────────────────────────────

/// Which location entity an actor is currently at
#[derive(Component, Clone, Debug)]
pub struct AtLocation(pub Entity);

/// Map position for rendering location nodes
#[derive(Component, Clone, Copy, Debug)]
pub struct MapPos(pub Vec2);

/// Connections to other locations (by entity)
#[derive(Component, Clone, Debug, Default)]
pub struct Connections(pub Vec<Entity>);

// ── Faction membership ───────────────────────────────────────────────────────

#[derive(Component, Clone, Debug)]
pub struct FactionMember(pub Entity);

/// Stored on faction entities
#[derive(Component, Clone, Debug, Default)]
pub struct FactionPower(pub i32); // 0–100

#[derive(Component, Clone, Debug, Default)]
pub struct FactionTension(pub i32); // 0–100, tension with rivals

// ── Inventory / Resources ────────────────────────────────────────────────────

#[derive(Component, Clone, Debug, Default)]
pub struct Wealth(pub i32);

// ── Front (evolving situation) ────────────────────────────────────────────────

#[derive(Component, Clone, Debug)]
pub struct Front {
    pub name: String,
    pub description: String,
    pub stage: u8,        // 0–4, escalating stages
    pub countdown: u32,   // turns until next stage
    pub active: bool,
    pub starting_countdown: u32,
    pub successor_front: Option<String>,
}

#[derive(Component, Clone, Debug, Default)]
pub struct FrontStages(pub Vec<FrontStage>);

#[derive(Clone, Debug)]
pub struct FrontStage {
    pub description: String,
    pub event_log_entry: String,
    pub countdown_turns: u32,
}

// ── UI state components ───────────────────────────────────────────────────────

#[derive(Component)]
pub struct EventLogUi;

#[derive(Component)]
pub struct PlayerPanelUi;

#[derive(Component)]
pub struct LocationPanelUi;

#[derive(Component)]
pub struct InteractionPanelUi;

#[derive(Component)]
pub struct LocationNodeUi {
    pub location_entity: Entity,
}

#[derive(Component)]
pub struct LocationLabel;

#[derive(Component)]
pub struct NpcListUi;

// ── NPC schedule / behavior ───────────────────────────────────────────────────

#[derive(Component, Clone, Debug)]
pub struct NpcBehavior {
    pub routine: NpcRoutine,
    pub move_cooldown: u32, // turns before NPC moves again
}

#[derive(Clone, Debug, PartialEq)]
pub enum NpcRoutine {
    StayPut,
    Patrol(Vec<Entity>),  // cycles through locations
    SeekGoal,
}
