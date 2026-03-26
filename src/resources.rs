use bevy::prelude::*;

// ── World time ────────────────────────────────────────────────────────────────

#[derive(Resource, Default)]
pub struct WorldTime {
    pub turn: u32,
    /// Seconds of real time since last tick (auto-advance)
    pub real_timer: f32,
    /// How many real seconds per simulation turn
    pub seconds_per_turn: f32,
    pub paused: bool,
}

impl WorldTime {
    pub fn new() -> Self {
        WorldTime {
            turn: 0,
            real_timer: 0.0,
            seconds_per_turn: 6.0,
            paused: false,
        }
    }
}

// ── Event log ─────────────────────────────────────────────────────────────────

#[derive(Resource, Default)]
pub struct EventLog {
    pub entries: Vec<LogEntry>,
}

pub struct LogEntry {
    pub turn: u32,
    pub text: String,
}

impl EventLog {
    pub fn push(&mut self, text: String) {
        self.entries.push(LogEntry { turn: 0, text });
    }

    pub fn push_at(&mut self, turn: u32, text: String) {
        self.entries.push(LogEntry { turn, text });
    }

    /// Most recent N entries, newest first
    pub fn recent(&self, n: usize) -> impl Iterator<Item = &LogEntry> {
        self.entries.iter().rev().take(n)
    }
}

// ── World state ───────────────────────────────────────────────────────────────

/// Global lookup tables populated during spawn.
#[derive(Resource, Default)]
pub struct WorldState {
    /// (id, name, entity)
    pub locations: Vec<(String, String, Entity)>,
    pub factions: Vec<(String, String, Entity)>,
    pub npcs: Vec<(String, String, Entity)>,
    pub player_location: Option<Entity>,
}

impl WorldState {
    pub fn location_entity(&self, id: &str) -> Option<Entity> {
        self.locations.iter().find(|(lid, _, _)| lid == id).map(|(_, _, e)| *e)
    }

    pub fn location_name(&self, entity: Entity) -> Option<&str> {
        self.locations.iter().find(|(_, _, e)| *e == entity).map(|(_, n, _)| n.as_str())
    }

    pub fn faction_entity(&self, id: &str) -> Option<Entity> {
        self.factions.iter().find(|(fid, _, _)| fid == id).map(|(_, _, e)| *e)
    }

    pub fn npc_name(&self, entity: Entity) -> Option<&str> {
        self.npcs.iter().find(|(_, _, e)| *e == entity).map(|(_, n, _)| n.as_str())
    }
}

// ── Game mode / UI state ──────────────────────────────────────────────────────

#[derive(Resource, Default, PartialEq, Eq, Clone, Debug)]
pub enum GameMode {
    #[default]
    Exploration,
    Interaction(Entity), // interacting with an NPC
    Travel,              // choosing destination
}

// ── Interaction state ─────────────────────────────────────────────────────────

#[derive(Resource, Default)]
pub struct InteractionState {
    pub selected_npc: Option<Entity>,
    pub dialogue_lines: Vec<String>,
    pub options: Vec<InteractionOption>,
}

#[derive(Clone, Debug)]
pub struct InteractionOption {
    pub label: String,
    pub action: PlayerAction,
}

#[derive(Clone, Debug)]
pub enum PlayerAction {
    AskRumor,
    AskAboutFaction(Entity),
    LeaveConversation,
    TravelTo(Entity),
}

// ── Pending tick signal ───────────────────────────────────────────────────────

/// Inserted as a resource when a simulation tick should fire this frame.
#[derive(Resource, Default)]
pub struct TickEvent;
