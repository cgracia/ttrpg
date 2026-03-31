/// UI layout, spawn, and update systems.
use bevy::{ecs::hierarchy::ChildSpawnerCommands, prelude::*};

use crate::components::*;
use crate::resources::*;
use crate::systems::interaction::execute_player_action;

// -- Colours -----------------------------------------------------------------

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

fn ui_text(value: impl Into<String>, font_size: f32, color: Color) -> impl Bundle {
    (
        Text::new(value.into()),
        TextFont {
            font_size,
            ..default()
        },
        TextColor(color),
    )
}

// -- Spawn UI ----------------------------------------------------------------

pub fn spawn_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            BackgroundColor(BG),
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    width: Val::Px(260.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(8.0)),
                    row_gap: Val::Px(6.0),
                    ..default()
                },
                BackgroundColor(PANEL_BG),
            ))
            .with_children(|left| {
                left.spawn((
                    PlayerPanelUi,
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(6.0)),
                        row_gap: Val::Px(4.0),
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    BorderColor::all(ACCENT),
                    BackgroundColor(BG),
                ))
                .with_children(|p| {
                    p.spawn(ui_text("PLAYER", 13.0, ACCENT));
                    for _ in 0..5 {
                        p.spawn(ui_text("", 12.0, TEXT));
                    }
                });

                left.spawn((
                    LocationPanelUi,
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(6.0)),
                        row_gap: Val::Px(4.0),
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    BorderColor::all(ACCENT),
                    BackgroundColor(BG),
                ))
                .with_children(|p| {
                    p.spawn(ui_text("LOCATION", 13.0, ACCENT));
                    for _ in 0..8 {
                        p.spawn(ui_text("", 12.0, TEXT));
                    }
                });

                left.spawn(Node {
                    padding: UiRect::all(Val::Px(4.0)),
                    ..default()
                })
                .with_children(|p| {
                    p.spawn((TimeLabelUi, ui_text("Turn 0", 12.0, TEXT_DIM)));
                });

                left.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(4.0),
                    ..default()
                })
                .with_children(|row| {
                    spawn_button(row, "[T] Travel", UiAction::OpenTravel);
                    spawn_button(row, "[P] Pause", UiAction::TogglePause);
                });
            });

            root.spawn((
                Node {
                    flex_grow: 1.0,
                    height: Val::Percent(100.0),
                    position_type: PositionType::Relative,
                    ..default()
                },
                BackgroundColor(BG),
            ))
            .with_children(|centre| {
                centre.spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Relative,
                    ..default()
                });
            });

            root.spawn((
                Node {
                    width: Val::Px(300.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(8.0)),
                    row_gap: Val::Px(6.0),
                    ..default()
                },
                BackgroundColor(PANEL_BG),
            ))
            .with_children(|right| {
                right.spawn((
                    InteractionPanelUi,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(220.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(6.0)),
                        row_gap: Val::Px(4.0),
                        border: UiRect::all(Val::Px(1.0)),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BorderColor::all(ACCENT),
                    BackgroundColor(BG),
                ))
                .with_children(|p| {
                    p.spawn(ui_text("INTERACTION", 13.0, ACCENT));
                });

                right.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_grow: 1.0,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(6.0)),
                        row_gap: Val::Px(3.0),
                        border: UiRect::all(Val::Px(1.0)),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BorderColor::all(ACCENT),
                    BackgroundColor(BG),
                ))
                .with_children(|p| {
                    p.spawn(ui_text("EVENT LOG", 13.0, ACCENT));
                    for _ in 0..20 {
                        p.spawn((EventLogUi, ui_text("", 11.0, TEXT_DIM)));
                    }
                });
            });
        });
}

// -- Map node spawning -------------------------------------------------------

pub fn spawn_map_nodes(
    mut commands: Commands,
    locations: Query<(Entity, &ActorName, &MapPos), With<LocationMarker>>,
) {
    let scale = 0.85_f32;

    for (loc_entity, name, map_pos) in &locations {
        let ui_x = 500.0 / 2.0 + map_pos.0.x * scale;
        let ui_y = 600.0 / 2.0 - map_pos.0.y * scale;

        commands
            .spawn((
                LocationNodeUi {
                    location_entity: loc_entity,
                },
                Button,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(ui_x - 40.0),
                    top: Val::Px(ui_y - 16.0),
                    width: Val::Px(80.0),
                    height: Val::Px(32.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(NODE_DEFAULT),
            ))
            .with_children(|btn| {
                btn.spawn(ui_text(name.0.clone(), 10.0, TEXT));
            });
    }
}

// -- Update systems ----------------------------------------------------------

pub fn update_player_panel(
    player: Query<(&AtLocation, &Wealth, &Knowledge, &Stats), With<Player>>,
    loc_names: Query<(&ActorName, &Description), With<LocationMarker>>,
    world_time: Res<WorldTime>,
    panel_children: Query<&Children, With<PlayerPanelUi>>,
    mut texts: Query<&mut Text>,
) {
    let Ok(children) = panel_children.single() else {
        return;
    };
    let Ok((at_loc, wealth, knowledge, stats)) = player.single() else {
        return;
    };

    let loc_name = loc_names
        .get(at_loc.0)
        .map(|(n, _)| n.0.as_str())
        .unwrap_or("?");

    let lines = [
        format!("Turn {}", world_time.turn),
        format!("Location: {}", loc_name),
        format!("Coin: {}", wealth.0),
        format!(
            "STR:{} CHA:{} CUN:{} RES:{}",
            stats.strength, stats.charisma, stats.cunning, stats.resolve
        ),
        format!("Knowledge: {} rumors", knowledge.0.len()),
    ];

    for (i, child) in children.iter().enumerate().skip(1) {
        if i - 1 >= lines.len() {
            break;
        }
        if let Ok(mut text) = texts.get_mut(child) {
            text.0 = lines[i - 1].clone();
        }
    }
}

pub fn update_location_panel(
    player: Query<&AtLocation, With<Player>>,
    npcs: Query<(&ActorName, &AtLocation, Option<&FactionMember>), With<Npc>>,
    loc_names: Query<(&ActorName, &Description), With<LocationMarker>>,
    faction_names: Query<&ActorName, With<FactionMarker>>,
    panel_children: Query<&Children, With<LocationPanelUi>>,
    mut texts: Query<&mut Text>,
) {
    let Ok(player_at) = player.single() else {
        return;
    };
    let Ok(children) = panel_children.single() else {
        return;
    };

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
            format!("  - {}{}", name.0, fac)
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
        if let Ok(mut text) = texts.get_mut(child) {
            text.0 = lines.get(i - 1).cloned().unwrap_or_default();
        }
    }
}

pub fn update_time_label(
    world_time: Res<WorldTime>,
    mut labels: Query<&mut Text, With<TimeLabelUi>>,
) {
    for mut text in &mut labels {
        let paused = if world_time.paused { " [PAUSED]" } else { "" };
        text.0 = format!("Turn {}{}", world_time.turn, paused);
    }
}

pub fn update_event_log(log: Res<EventLog>, mut texts: Query<(&mut Text, &EventLogUi)>) {
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
        text.0 = entries.get(i).cloned().unwrap_or_default();
    }
}

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

    let Ok((panel_entity, children)) = panel.single() else {
        return;
    };

    for (i, child) in children.iter().enumerate().skip(1) {
        if let Ok(mut text) = texts.get_mut(child) {
            let line = interaction
                .dialogue_lines
                .get(i - 1)
                .cloned()
                .unwrap_or_default();
            text.0 = line;
        }
    }

    for btn_entity in &btn_query {
        commands
            .entity(btn_entity)
            .despawn_related::<Children>()
            .despawn();
    }

    for (idx, opt) in interaction.options.iter().enumerate() {
        let label = opt.label.clone();
        let action = opt.action.clone();
        commands.entity(panel_entity).with_children(|p| {
            p.spawn((
                UiActionTag { action, idx },
                Button,
                Node {
                    padding: UiRect::new(
                        Val::Px(6.0),
                        Val::Px(6.0),
                        Val::Px(3.0),
                        Val::Px(3.0),
                    ),
                    margin: UiRect::top(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(BTN_BG),
            ))
            .with_children(|btn| {
                btn.spawn(ui_text(format!("[{}] {}", idx + 1, label), 11.0, TEXT));
            });
        });
    }
}

// -- Button interaction ------------------------------------------------------

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

fn spawn_button(parent: &mut ChildSpawnerCommands, label: &str, action: UiAction) {
    parent
        .spawn((
            action,
            Button,
            Node {
                padding: UiRect::new(
                    Val::Px(8.0),
                    Val::Px(8.0),
                    Val::Px(4.0),
                    Val::Px(4.0),
                ),
                ..default()
            },
            BackgroundColor(BTN_BG),
        ))
        .with_children(|btn| {
            btn.spawn(ui_text(label, 12.0, TEXT));
        });
}

pub fn handle_ui_buttons(
    mut interaction_query: Query<
        (&Interaction, &UiAction, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut mode: ResMut<GameMode>,
    mut world_time: ResMut<WorldTime>,
) {
    for (interact, action, mut bg) in &mut interaction_query {
        match interact {
            Interaction::Pressed => match action {
                UiAction::OpenTravel => {
                    if *mode == GameMode::Exploration {
                        *mode = GameMode::Travel;
                    }
                }
                UiAction::TogglePause => {
                    world_time.paused = !world_time.paused;
                }
            },
            Interaction::Hovered => *bg = BTN_HOVER.into(),
            Interaction::None => *bg = BTN_BG.into(),
        }
    }
}

pub fn handle_action_buttons(
    mut interaction_query: Query<
        (&Interaction, &UiActionTag, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut commands: Commands,
    mut mode: ResMut<GameMode>,
    mut interaction_state: ResMut<InteractionState>,
    mut player_query: Query<(&mut AtLocation, &mut Knowledge, &mut Wealth), (With<Player>, Without<Npc>)>,
    npc_query:
        Query<(&ActorName, &Knowledge, &Goals, &Traits, &Wealth, &AtLocation), (With<Npc>, Without<Player>)>,
    mut faction_query: Query<
        (&ActorName, &mut FactionPower, &mut FactionTension, &Description),
        With<FactionMarker>,
    >,
    mut front_query: Query<&mut Front>,
    world: Res<WorldState>,
    mut log: ResMut<EventLog>,
    time: Res<WorldTime>,
) {
    for (interact, tag, mut bg) in &mut interaction_query {
        match interact {
            Interaction::Pressed => {
                execute_player_action(
                    tag.action.clone(),
                    &mut commands,
                    &mut mode,
                    &mut interaction_state,
                    &mut player_query,
                    &npc_query,
                    &mut faction_query,
                    &mut front_query,
                    &world,
                    &mut log,
                    &time,
                );
            }
            Interaction::Hovered => *bg = BTN_HOVER.into(),
            Interaction::None => *bg = BTN_BG.into(),
        }
    }
}

pub fn handle_location_clicks(
    mut interaction_query: Query<
        (&Interaction, &LocationNodeUi, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut player: Query<(&mut AtLocation, &Knowledge), (With<Player>, Without<Npc>)>,
    connections: Query<&Connections>,
    loc_names: Query<&ActorName, With<LocationMarker>>,
    npcs: Query<(&ActorName, &AtLocation, Option<&FactionMember>), (With<Npc>, Without<Player>)>,
    faction_names:
        Query<&ActorName, (With<FactionMarker>, Without<Npc>, Without<LocationMarker>)>,
    mut mode: ResMut<GameMode>,
    mut interaction_state: ResMut<InteractionState>,
    mut log: ResMut<EventLog>,
    time: Res<WorldTime>,
    world: Res<WorldState>,
) {
    let Ok((player_at, _)) = player.single() else {
        return;
    };
    let player_loc = player_at.0;

    for (interact, loc_node, mut bg) in &mut interaction_query {
        let loc_entity = loc_node.location_entity;
        let is_player_here = player_loc == loc_entity;

        if *interact == Interaction::None {
            *bg = if is_player_here {
                NODE_PLAYER.into()
            } else {
                NODE_DEFAULT.into()
            };
        }

        match interact {
            Interaction::Hovered => *bg = NODE_HOVER.into(),
            Interaction::Pressed => {
                if is_player_here {
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
                    interaction_state
                        .dialogue_lines
                        .push(format!("At: {}", loc_name));
                    if npcs_here.is_empty() {
                        interaction_state
                            .dialogue_lines
                            .push("Nobody notable here.".into());
                    } else {
                        for npc_name in &npcs_here {
                            interaction_state
                                .dialogue_lines
                                .push(format!("  - {}", npc_name));
                        }
                    }
                } else {
                    let connected = connections
                        .get(player_loc)
                        .map(|c| c.0.contains(&loc_entity))
                        .unwrap_or(false);

                    if connected {
                        let dest_name = world.location_name(loc_entity).unwrap_or("?");
                        log.push_at(time.turn, format!("You travel to {}.", dest_name));

                        if let Ok((mut at_loc, _)) = player.single_mut() {
                            at_loc.0 = loc_entity;
                        }
                        *mode = GameMode::Exploration;
                    } else {
                        let loc_name = loc_names.get(loc_entity).map(|n| n.0.as_str()).unwrap_or("?");
                        interaction_state.dialogue_lines.clear();
                        interaction_state.dialogue_lines.push(format!(
                            "{} - not directly reachable from here.",
                            loc_name
                        ));
                    }
                }
            }
            Interaction::None => {}
        }
    }
}

pub fn handle_npc_interaction_from_location(
    npcs_here: Query<(Entity, &ActorName, &AtLocation), With<Npc>>,
    player: Query<&AtLocation, With<Player>>,
    mut mode: ResMut<GameMode>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyI) {
        if let Ok(player_at) = player.single() {
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
