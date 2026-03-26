/// UI layout, spawn, and update systems.
use bevy::prelude::*;

use crate::components::*;
use crate::resources::*;
use crate::systems::interaction::execute_player_action;

// ── Colours ───────────────────────────────────────────────────────────────────

const BG: Color = Color::srgb(0.08, 0.08, 0.10);
const PANEL_BG: Color = Color::srgb(0.12, 0.12, 0.15);
const ACCENT: Color = Color::srgb(0.75, 0.60, 0.30);
const TEXT: Color = Color::srgb(0.88, 0.85, 0.78);
const TEXT_DIM: Color = Color::srgb(0.55, 0.52, 0.45);
const NODE_DEFAULT: Color = Color::srgb(0.25, 0.40, 0.55);
const NODE_PLAYER: Color = Color::srgb(0.35, 0.75, 0.45);
const NODE_HOVER: Color = Color::srgb(0.55, 0.70, 0.85);
const BTN_BG: Color = Color::srgb(0.18, 0.20, 0.25);
const BTN_HOVER: Color = Color::srgb(0.28, 0.30, 0.38);

// ── Spawn UI ──────────────────────────────────────────────────────────────────

pub fn spawn_ui(mut commands: Commands) {
    // Root: full screen flex row
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            background_color: BG.into(),
            ..default()
        })
        .with_children(|root| {
            // Left sidebar – player info + interactions
            root.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(260.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(8.0)),
                    row_gap: Val::Px(6.0),
                    ..default()
                },
                background_color: PANEL_BG.into(),
                ..default()
            })
            .with_children(|left| {
                // Player panel
                left.spawn((
                    PlayerPanelUi,
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(6.0)),
                            row_gap: Val::Px(4.0),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        border_color: ACCENT.into(),
                        background_color: BG.into(),
                        ..default()
                    },
                ))
                .with_children(|p| {
                    p.spawn(TextBundle::from_section(
                        "PLAYER",
                        TextStyle { font_size: 13.0, color: ACCENT, ..default() },
                    ));
                    // Placeholder text nodes updated each frame
                    for _ in 0..5 {
                        p.spawn(TextBundle::from_section(
                            "",
                            TextStyle { font_size: 12.0, color: TEXT, ..default() },
                        ));
                    }
                });

                // Current location panel
                left.spawn((
                    LocationPanelUi,
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(6.0)),
                            row_gap: Val::Px(4.0),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        border_color: ACCENT.into(),
                        background_color: BG.into(),
                        ..default()
                    },
                ))
                .with_children(|p| {
                    p.spawn(TextBundle::from_section(
                        "LOCATION",
                        TextStyle { font_size: 13.0, color: ACCENT, ..default() },
                    ));
                    for _ in 0..8 {
                        p.spawn(TextBundle::from_section(
                            "",
                            TextStyle { font_size: 12.0, color: TEXT, ..default() },
                        ));
                    }
                });

                // Time display
                left.spawn(NodeBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(4.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|p| {
                    p.spawn((
                        TimeLabelUi,
                        TextBundle::from_section(
                            "Turn 0",
                            TextStyle { font_size: 12.0, color: TEXT_DIM, ..default() },
                        ),
                    ));
                });

                // [T] Travel / [P] Pause buttons
                left.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(4.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|row| {
                    spawn_button(row, "[T] Travel", UiAction::OpenTravel);
                    spawn_button(row, "[P] Pause", UiAction::TogglePause);
                });
            });

            // Centre – map
            root.spawn(NodeBundle {
                style: Style {
                    flex_grow: 1.0,
                    height: Val::Percent(100.0),
                    position_type: PositionType::Relative,
                    ..default()
                },
                background_color: BG.into(),
                ..default()
            })
            .with_children(|centre| {
                // Location nodes drawn as absolute-positioned boxes
                // Actual nodes are spawned by spawn_map_nodes system
                centre.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        position_type: PositionType::Relative,
                        ..default()
                    },
                    ..default()
                });
            });

            // Right sidebar – interactions + event log
            root.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(300.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(8.0)),
                    row_gap: Val::Px(6.0),
                    ..default()
                },
                background_color: PANEL_BG.into(),
                ..default()
            })
            .with_children(|right| {
                // Interaction panel
                right.spawn((
                    InteractionPanelUi,
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(220.0),
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(6.0)),
                            row_gap: Val::Px(4.0),
                            border: UiRect::all(Val::Px(1.0)),
                            overflow: Overflow::clip(),
                            ..default()
                        },
                        border_color: ACCENT.into(),
                        background_color: BG.into(),
                        ..default()
                    },
                ))
                .with_children(|p| {
                    p.spawn(TextBundle::from_section(
                        "INTERACTION",
                        TextStyle { font_size: 13.0, color: ACCENT, ..default() },
                    ));
                });

                // Event log panel
                right.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        flex_grow: 1.0,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(6.0)),
                        row_gap: Val::Px(3.0),
                        border: UiRect::all(Val::Px(1.0)),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    border_color: ACCENT.into(),
                    background_color: BG.into(),
                    ..default()
                })
                .with_children(|p| {
                    p.spawn(TextBundle::from_section(
                        "EVENT LOG",
                        TextStyle { font_size: 13.0, color: ACCENT, ..default() },
                    ));
                    // Log lines (updated each frame)
                    for _ in 0..20 {
                        p.spawn((
                            EventLogUi,
                            TextBundle::from_section(
                                "",
                                TextStyle { font_size: 11.0, color: TEXT_DIM, ..default() },
                            ),
                        ));
                    }
                });
            });
        });
}

// ── Map node spawning ─────────────────────────────────────────────────────────

pub fn spawn_map_nodes(
    mut commands: Commands,
    locations: Query<(Entity, &ActorName, &MapPos), With<LocationMarker>>,
) {
    let map_center = Vec2::new(0.0, 0.0);
    let scale = 0.85_f32;

    for (loc_entity, name, map_pos) in locations.iter() {
        // Convert world coords to UI coords (centre of map panel ~500×600)
        let ui_x = 500.0 / 2.0 + map_pos.0.x * scale;
        let ui_y = 600.0 / 2.0 - map_pos.0.y * scale; // flip Y for UI
        let _ = map_center;

        commands.spawn((
            LocationNodeUi { location_entity: loc_entity },
            ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(ui_x - 40.0),
                    top: Val::Px(ui_y - 16.0),
                    width: Val::Px(80.0),
                    height: Val::Px(32.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: NODE_DEFAULT.into(),
                ..default()
            },
        ))
        .with_children(|btn| {
            btn.spawn(TextBundle::from_section(
                name.0.clone(),
                TextStyle { font_size: 10.0, color: TEXT, ..default() },
            ));
        });
    }
}

// ── Update systems ────────────────────────────────────────────────────────────

/// Refresh the player panel.
pub fn update_player_panel(
    player: Query<(&AtLocation, &Wealth, &Knowledge, &Stats), With<Player>>,
    loc_names: Query<(&ActorName, &Description), With<LocationMarker>>,
    world_time: Res<WorldTime>,
    mut panel_children: Query<&Children, With<PlayerPanelUi>>,
    mut texts: Query<&mut Text>,
) {
    let Ok(children) = panel_children.get_single_mut() else { return };
    let Ok((at_loc, wealth, knowledge, stats)) = player.get_single() else { return };

    let loc_name = loc_names.get(at_loc.0).map(|(n, _)| n.0.as_str()).unwrap_or("?");

    let lines = [
        format!("Turn {}", world_time.turn),
        format!("Location: {}", loc_name),
        format!("Coin: {}", wealth.0),
        format!("STR:{} CHA:{} CUN:{} RES:{}", stats.strength, stats.charisma, stats.cunning, stats.resolve),
        format!("Knowledge: {} rumors", knowledge.0.len()),
    ];

    for (i, child) in children.iter().enumerate().skip(1) {
        if i - 1 >= lines.len() { break; }
        if let Ok(mut text) = texts.get_mut(*child) {
            text.sections[0].value = lines[i - 1].clone();
        }
    }
}

/// Refresh the location panel: show who's here.
pub fn update_location_panel(
    player: Query<&AtLocation, With<Player>>,
    npcs: Query<(&ActorName, &AtLocation, Option<&FactionMember>), With<Npc>>,
    loc_names: Query<(&ActorName, &Description), With<LocationMarker>>,
    faction_names: Query<&ActorName, With<FactionMarker>>,
    panel_children: Query<&Children, With<LocationPanelUi>>,
    mut texts: Query<&mut Text>,
) {
    let Ok(player_at) = player.get_single() else { return };
    let Ok(children) = panel_children.get_single() else { return };

    let (loc_name, loc_desc) = loc_names
        .get(player_at.0)
        .map(|(n, d)| (n.0.clone(), d.0.clone()))
        .unwrap_or_default();

    let npcs_here: Vec<String> = npcs
        .iter()
        .filter(|(_, at, _)| at.0 == player_at.0)
        .map(|(name, _, faction)| {
            let fac = faction
                .and_then(|fm| faction_names.get(fm.0).ok())
                .map(|n| format!(" [{}]", n.0))
                .unwrap_or_default();
            format!("  • {}{}", name.0, fac)
        })
        .collect();

    let mut lines: Vec<String> = vec![loc_name, loc_desc];
    if npcs_here.is_empty() {
        lines.push("  (nobody else here)".into());
    } else {
        lines.push("Present:".into());
        lines.extend(npcs_here);
    }

    for (i, child) in children.iter().enumerate().skip(1) {
        if let Ok(mut text) = texts.get_mut(*child) {
            text.sections[0].value = lines.get(i - 1).cloned().unwrap_or_default();
        }
    }
}

/// Refresh the time label.
pub fn update_time_label(
    world_time: Res<WorldTime>,
    mut labels: Query<&mut Text, With<TimeLabelUi>>,
) {
    for mut text in labels.iter_mut() {
        let paused = if world_time.paused { " [PAUSED]" } else { "" };
        text.sections[0].value = format!("Turn {}{}", world_time.turn, paused);
    }
}

/// Refresh the event log panel.
pub fn update_event_log(
    log: Res<EventLog>,
    mut texts: Query<(&mut Text, &EventLogUi)>,
) {
    let entries: Vec<String> = log
        .recent(20)
        .map(|e| {
            if e.turn > 0 {
                format!("[T{}] {}", e.turn, e.text)
            } else {
                e.text.clone()
            }
        })
        .collect();

    for (i, (mut text, _)) in texts.iter_mut().enumerate() {
        text.sections[0].value = entries.get(i).cloned().unwrap_or_default();
    }
}

/// Refresh the interaction panel.
pub fn update_interaction_panel(
    interaction: Res<InteractionState>,
    mode: Res<GameMode>,
    panel: Query<(Entity, &Children), With<InteractionPanelUi>>,
    mut texts: Query<&mut Text>,
    mut commands: Commands,
    btn_query: Query<Entity, (With<Button>, With<UiActionTag>)>,
) {
    if !interaction.is_changed() && !mode.is_changed() {
        return;
    }

    let Ok((panel_entity, children)) = panel.get_single() else { return };

    // Update dialogue lines (children skip(1) = skip header label)
    for (i, child) in children.iter().enumerate().skip(1) {
        if let Ok(mut text) = texts.get_mut(*child) {
            let line = interaction.dialogue_lines.get(i - 1).cloned().unwrap_or_default();
            text.sections[0].value = line;
        }
    }

    // Remove old action buttons
    for btn_entity in btn_query.iter() {
        commands.entity(btn_entity).despawn_recursive();
    }

    // Spawn new action buttons as children of the panel
    for (idx, opt) in interaction.options.iter().enumerate() {
        let label = opt.label.clone();
        let action = opt.action.clone();
        commands.entity(panel_entity).with_children(|p| {
            p.spawn((
                UiActionTag { action, idx },
                ButtonBundle {
                    style: Style {
                        padding: UiRect::new(
                            Val::Px(6.0), Val::Px(6.0), Val::Px(3.0), Val::Px(3.0),
                        ),
                        margin: UiRect::top(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: BTN_BG.into(),
                    ..default()
                },
            ))
            .with_children(|btn| {
                btn.spawn(TextBundle::from_section(
                    format!("[{}] {}", idx + 1, label),
                    TextStyle { font_size: 11.0, color: TEXT, ..default() },
                ));
            });
        });
    }
}

// ── Button interaction ────────────────────────────────────────────────────────

#[derive(Component, Clone)]
pub struct TimeLabelUi;

#[derive(Component, Clone)]
pub struct UiActionTag {
    pub action: PlayerAction,
    pub idx: usize,
}

#[derive(Component, Clone)]
pub enum UiAction {
    OpenTravel,
    TogglePause,
}

fn spawn_button(parent: &mut ChildBuilder, label: &str, action: UiAction) {
    parent
        .spawn((
            action,
            ButtonBundle {
                style: Style {
                    padding: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(4.0), Val::Px(4.0)),
                    ..default()
                },
                background_color: BTN_BG.into(),
                ..default()
            },
        ))
        .with_children(|btn| {
            btn.spawn(TextBundle::from_section(
                label,
                TextStyle { font_size: 12.0, color: TEXT, ..default() },
            ));
        });
}

/// Handle top-bar button clicks.
pub fn handle_ui_buttons(
    mut interaction_query: Query<
        (&Interaction, &UiAction, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut mode: ResMut<GameMode>,
    mut world_time: ResMut<WorldTime>,
) {
    for (interact, action, mut bg) in interaction_query.iter_mut() {
        match interact {
            Interaction::Pressed => {
                match action {
                    UiAction::OpenTravel => {
                        if *mode == GameMode::Exploration {
                            *mode = GameMode::Travel;
                        }
                    }
                    UiAction::TogglePause => {
                        world_time.paused = !world_time.paused;
                    }
                }
            }
            Interaction::Hovered => { *bg = BTN_HOVER.into(); }
            Interaction::None => { *bg = BTN_BG.into(); }
        }
    }
}

/// Handle action button clicks in the interaction panel.
pub fn handle_action_buttons(
    mut interaction_query: Query<
        (&Interaction, &UiActionTag, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut commands: Commands,
    mut mode: ResMut<GameMode>,
    mut interaction_state: ResMut<InteractionState>,
    mut player_query: Query<(&mut AtLocation, &mut Knowledge), With<Player>>,
    npc_query: Query<(&ActorName, &Knowledge, &Goals, &Traits, &Wealth), With<Npc>>,
    faction_query: Query<(&ActorName, &FactionPower, &FactionTension, &Description), With<FactionMarker>>,
    world: Res<WorldState>,
    mut log: ResMut<EventLog>,
    time: Res<WorldTime>,
) {
    for (interact, tag, mut bg) in interaction_query.iter_mut() {
        match interact {
            Interaction::Pressed => {
                execute_player_action(
                    tag.action.clone(),
                    &mut commands,
                    &mut mode,
                    &mut interaction_state,
                    &mut player_query,
                    &npc_query,
                    &faction_query,
                    &world,
                    &mut log,
                    &time,
                );
            }
            Interaction::Hovered => { *bg = BTN_HOVER.into(); }
            Interaction::None => { *bg = BTN_BG.into(); }
        }
    }
}

/// Handle clicking on a location node (travel or inspect).
pub fn handle_location_clicks(
    mut interaction_query: Query<
        (&Interaction, &LocationNodeUi, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut player: Query<(&mut AtLocation, &Knowledge), With<Player>>,
    connections: Query<&Connections>,
    loc_names: Query<&ActorName, With<LocationMarker>>,
    npcs: Query<(&ActorName, &AtLocation, Option<&FactionMember>), With<Npc>>,
    faction_names: Query<&ActorName, (With<FactionMarker>, Without<Npc>, Without<LocationMarker>)>,
    mut mode: ResMut<GameMode>,
    mut interaction_state: ResMut<InteractionState>,
    mut log: ResMut<EventLog>,
    time: Res<WorldTime>,
    world: Res<WorldState>,
) {
    let Ok((ref player_at, _)) = player.get_single() else { return };
    let player_loc = player_at.0;

    for (interact, loc_node, mut bg) in interaction_query.iter_mut() {
        let loc_entity = loc_node.location_entity;

        // Highlight player location
        let is_player_here = player_loc == loc_entity;
        if *interact == Interaction::None {
            *bg = if is_player_here { NODE_PLAYER.into() } else { NODE_DEFAULT.into() };
        }

        match interact {
            Interaction::Hovered => { *bg = NODE_HOVER.into(); }
            Interaction::Pressed => {
                if is_player_here {
                    // Show NPCs at this location in interaction panel
                    let npcs_here: Vec<String> = npcs
                        .iter()
                        .filter(|(_, at, _)| at.0 == loc_entity)
                        .map(|(name, _, fm)| {
                            let fac = fm
                                .and_then(|f| faction_names.get(f.0).ok())
                                .map(|n| format!(" [{}]", n.0))
                                .unwrap_or_default();
                            format!("{}{}", name.0, fac)
                        })
                        .collect();

                    interaction_state.selected_npc = None;
                    interaction_state.dialogue_lines.clear();
                    interaction_state.options.clear();

                    let loc_name = loc_names.get(loc_entity).map(|n| n.0.as_str()).unwrap_or("?");
                    interaction_state.dialogue_lines.push(format!("At: {}", loc_name));
                    if npcs_here.is_empty() {
                        interaction_state.dialogue_lines.push("Nobody notable here.".into());
                    } else {
                        for npc_name in &npcs_here {
                            interaction_state.dialogue_lines.push(format!("  • {}", npc_name));
                        }
                    }
                } else {
                    // Check if location is adjacent — if so, travel there
                    let connected = connections
                        .get(player_loc)
                        .map(|c| c.0.contains(&loc_entity))
                        .unwrap_or(false);

                    if connected {
                        let dest_name = world.location_name(loc_entity).unwrap_or("?");
                        log.push_at(time.turn, format!("You travel to {}.", dest_name));

                        if let Ok((mut at_loc, _)) = player.get_single_mut() {
                            at_loc.0 = loc_entity;
                        }
                        *mode = GameMode::Exploration;
                    } else {
                        let loc_name = loc_names.get(loc_entity).map(|n| n.0.as_str()).unwrap_or("?");
                        interaction_state.dialogue_lines.clear();
                        interaction_state.dialogue_lines.push(format!("{} — not directly reachable from here.", loc_name));
                    }
                }
            }
            Interaction::None => {}
        }
    }
}

/// Handle clicking on an NPC in the location panel — open interaction.
pub fn handle_npc_interaction_from_location(
    npcs_here: Query<(Entity, &ActorName, &AtLocation), With<Npc>>,
    player: Query<&AtLocation, With<Player>>,
    mut mode: ResMut<GameMode>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // Keyboard shortcut: press I to interact with first NPC at location
    if keyboard.just_pressed(KeyCode::KeyI) {
        if let Ok(player_at) = player.get_single() {
            let first_npc = npcs_here
                .iter()
                .find(|(_, _, at)| at.0 == player_at.0)
                .map(|(e, _, _)| e);

            if let Some(npc_e) = first_npc {
                *mode = GameMode::Interaction(npc_e);
            }
        }
    }
}

/// Keyboard shortcuts: [T] travel, [P] pause.
pub fn keyboard_shortcuts(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut mode: ResMut<GameMode>,
    mut world_time: ResMut<WorldTime>,
) {
    if keyboard.just_pressed(KeyCode::KeyT) {
        if *mode == GameMode::Exploration {
            *mode = GameMode::Travel;
        } else if *mode == GameMode::Travel {
            *mode = GameMode::Exploration;
        }
    }

    if keyboard.just_pressed(KeyCode::KeyP) {
        world_time.paused = !world_time.paused;
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        *mode = GameMode::Exploration;
    }
}
