/// Debug tools: screenshot capture and world state dump.
use bevy::{
    prelude::*,
    render::view::screenshot::{save_to_disk, Screenshot},
};

use crate::components::*;
use crate::resources::*;

/// Press F12 to save a screenshot to `screenshots/latest.png`.
/// Tries external capture tools first (more reliable in X11/nix-shell environments),
/// then falls back to Bevy's built-in screenshot API.
pub fn screenshot_on_f12(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut event_log: ResMut<EventLog>,
    time: Res<WorldTime>,
) {
    if !keyboard.just_pressed(KeyCode::F12) {
        return;
    }

    let dir = "screenshots";
    std::fs::create_dir_all(dir).ok();
    let path = format!("{dir}/latest.png");

    // Try grim (Wayland-native) → scrot (X11 fallback) → Bevy API (last resort)
    let tool = if std::process::Command::new("grim")
        .args([&path])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        Some("grim")
    } else if std::process::Command::new("scrot")
        .args(["-d", "0", "--overwrite", &path])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        Some("scrot")
    } else {
        None
    };

    if let Some(tool_name) = tool {
        event_log.push_at(
            time.turn,
            format!("Debug: screenshot saved ({tool_name}) -> screenshots/latest.png"),
        );
    } else {
        // Fall back to Bevy's screenshot API (may produce black image in some environments)
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
        event_log.push_at(
            time.turn,
            "Debug: screenshot requested (Bevy API) -> screenshots/latest.png".into(),
        );
    }
}

/// Press F11 to dump world state to `debug/world_state.txt`.
pub fn state_dump_on_f11(
    keyboard: Res<ButtonInput<KeyCode>>,
    world_time: Res<WorldTime>,
    world_state: Res<WorldState>,
    event_log: Res<EventLog>,
    player_q: Query<(&AtLocation, &Wealth, &Knowledge, &Stats), With<Player>>,
    npc_q: Query<
        (
            &ActorName,
            &AtLocation,
            &Stats,
            &Traits,
            &Goals,
            &Wealth,
            &Knowledge,
            Option<&FactionMember>,
        ),
        With<Npc>,
    >,
    loc_q: Query<(&ActorName, &Description, &Connections), With<LocationMarker>>,
    faction_q: Query<(&ActorName, &FactionPower, &FactionTension), With<FactionMarker>>,
    front_q: Query<(&Front, &FrontStages)>,
    mode: Res<GameMode>,
) {
    // Also trigger from file: playtest skill writes `debug/trigger` to request a dump.
    let trigger_path = "debug/trigger";
    let file_triggered = std::path::Path::new(trigger_path).exists();
    if file_triggered {
        let _ = std::fs::remove_file(trigger_path);
    }

    if !keyboard.just_pressed(KeyCode::F11) && !file_triggered {
        return;
    }

    let dir = "debug";
    std::fs::create_dir_all(dir).ok();

    let mut out = String::new();

    // Header
    out.push_str(&format!(
        "=== Ashenveil World State — Turn {} ===\n\n",
        world_time.turn
    ));
    out.push_str(&format!(
        "Paused: {} | Mode: {:?}\n\n",
        world_time.paused, *mode
    ));

    // Player
    if let Ok((at_loc, wealth, knowledge, stats)) = player_q.single() {
        let loc_name = world_state.location_name(at_loc.0).unwrap_or("?");
        out.push_str("── PLAYER ──\n");
        out.push_str(&format!("  Location: {}\n", loc_name));
        out.push_str(&format!("  Wealth: {}\n", wealth.0));
        out.push_str(&format!(
            "  Stats: STR:{} CHA:{} CUN:{} RES:{}\n",
            stats.strength, stats.charisma, stats.cunning, stats.resolve
        ));
        out.push_str(&format!("  Rumors known: {}\n", knowledge.0.len()));
        for rumor in &knowledge.0 {
            out.push_str(&format!(
                "    - [T{}, cred:{}] {}\n",
                rumor.turn_learned, rumor.credibility, rumor.text
            ));
        }
        out.push('\n');
    }

    // Factions
    out.push_str("── FACTIONS ──\n");
    for (id, name, entity) in &world_state.factions {
        if let Ok((_, power, tension)) = faction_q.get(*entity) {
            out.push_str(&format!(
                "  {} ({}): power={}, tension={}\n",
                name, id, power.0, tension.0
            ));
        }
    }
    out.push('\n');

    // Locations
    out.push_str("── LOCATIONS ──\n");
    for (id, name, entity) in &world_state.locations {
        if let Ok((_, desc, conns)) = loc_q.get(*entity) {
            let conn_names: Vec<&str> = conns
                .0
                .iter()
                .filter_map(|e| world_state.location_name(*e))
                .collect();
            out.push_str(&format!("  {} ({}):\n", name, id));
            out.push_str(&format!("    {}\n", desc.0));
            out.push_str(&format!("    Connects to: {}\n", conn_names.join(", ")));

            // NPCs here
            let npcs_here: Vec<String> = npc_q
                .iter()
                .filter(|(_, at, _, _, _, _, _, _)| at.0 == *entity)
                .map(|(n, _, _, _, _, _, _, _)| n.0.clone())
                .collect();
            if !npcs_here.is_empty() {
                out.push_str(&format!("    NPCs present: {}\n", npcs_here.join(", ")));
            }
        }
    }
    out.push('\n');

    // NPCs
    out.push_str("── NPCs ──\n");
    for (id, name, entity) in &world_state.npcs {
        if let Ok((_, at_loc, stats, traits, goals, wealth, knowledge, faction)) =
            npc_q.get(*entity)
        {
            let loc_name = world_state.location_name(at_loc.0).unwrap_or("?");
            let faction_name = faction
                .and_then(|fm| {
                    world_state
                        .factions
                        .iter()
                        .find(|(_, _, e)| *e == fm.0)
                        .map(|(_, n, _)| n.as_str())
                })
                .unwrap_or("none");
            let trait_labels: Vec<&str> = traits.0.iter().map(|t| t.label()).collect();
            let goal_labels: Vec<&str> = goals.0.iter().map(|g| g.kind.label()).collect();

            out.push_str(&format!("  {} ({}):\n", name, id));
            out.push_str(&format!("    Location: {} | Faction: {}\n", loc_name, faction_name));
            out.push_str(&format!(
                "    Stats: STR:{} CHA:{} CUN:{} RES:{}\n",
                stats.strength, stats.charisma, stats.cunning, stats.resolve
            ));
            out.push_str(&format!("    Traits: {}\n", trait_labels.join(", ")));
            out.push_str(&format!("    Goals: {}\n", goal_labels.join(", ")));
            out.push_str(&format!("    Wealth: {}\n", wealth.0));
            out.push_str(&format!("    Rumors: {}\n", knowledge.0.len()));
        }
    }
    out.push('\n');

    // Fronts
    out.push_str("── FRONTS ──\n");
    for (front, stages) in front_q.iter() {
        out.push_str(&format!(
            "  {} [stage {}/{}] active={} countdown={}\n",
            front.name,
            front.stage,
            stages.0.len().saturating_sub(1),
            front.active,
            front.countdown
        ));
        out.push_str(&format!("    {}\n", front.description));
        if let Some(stage) = stages.0.get(front.stage as usize) {
            out.push_str(&format!("    Current stage: {}\n", stage.description));
        }
    }
    out.push('\n');

    // Recent events
    out.push_str("── RECENT EVENTS (last 15) ──\n");
    for entry in event_log.recent(15) {
        if entry.turn > 0 {
            out.push_str(&format!("  [T{}] {}\n", entry.turn, entry.text));
        } else {
            out.push_str(&format!("  {}\n", entry.text));
        }
    }

    // Write file
    let path = format!("{}/world_state.txt", dir);
    match std::fs::write(&path, &out) {
        Ok(_) => {}
        Err(e) => eprintln!("Failed to dump world state: {}", e),
    }
}
