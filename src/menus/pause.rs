//! The pause menu.

use bevy::{input::ButtonState, input::keyboard::KeyboardInput, prelude::*};

use crate::{menus::Menu, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Pause), spawn_pause_menu);
    app.add_systems(
        Update,
        (handle_pause_input, update_pause_selection).run_if(in_state(Menu::Pause)),
    );
}

#[derive(Component)]
struct PauseMenuContainer;

#[derive(Component)]
struct PauseOption {
    index: usize,
    action: PauseAction,
}

#[derive(Clone, Copy)]
enum PauseAction {
    Continue,
    Settings,
    QuitToTitle,
}

#[derive(Resource)]
struct PauseSelection {
    current: usize,
    max: usize,
}

fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Initialize pause selection
    commands.insert_resource(PauseSelection {
        current: 0,
        max: 2,
    });

    // Terminal-style pause menu container
    commands
        .spawn((
            Name::new("Pause Menu Container"),
            PauseMenuContainer,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.01, 0.01, 0.05, 0.95)), // Semi-transparent dark blue-black
            GlobalZIndex(2),
            DespawnOnExit(Menu::Pause),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text("GAME PAUSED".to_string()),
                TextFont {
                    font: asset_server.load("fonts/FiraCode-Regular.ttf"),
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 1.0, 0.8)), // Cyan
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            ));

            // Menu options container
            parent
                .spawn((Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(15.0),
                    ..default()
                },))
                .with_children(|parent| {
                    // Continue option
                    parent.spawn((
                        PauseOption {
                            index: 0,
                            action: PauseAction::Continue,
                        },
                        Text(format!("  {}  ", "Continue")),
                        TextFont {
                            font: asset_server.load("fonts/FiraCode-Regular.ttf"),
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.0, 1.0, 0.8)), // Cyan - selected by default
                    ));

                    // Settings option
                    parent.spawn((
                        PauseOption {
                            index: 1,
                            action: PauseAction::Settings,
                        },
                        Text(format!("  {}  ", "Settings")),
                        TextFont {
                            font: asset_server.load("fonts/FiraCode-Regular.ttf"),
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.5, 0.5, 0.6)),
                    ));

                    // Quit to title option
                    parent.spawn((
                        PauseOption {
                            index: 2,
                            action: PauseAction::QuitToTitle,
                        },
                        Text(format!("  {}  ", "Quit to Title")),
                        TextFont {
                            font: asset_server.load("fonts/FiraCode-Regular.ttf"),
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.5, 0.5, 0.6)),
                    ));
                });

            // Instructions
            parent.spawn((
                Text("↑↓ Navigate  •  Enter Select  •  Esc Back".to_string()),
                TextFont {
                    font: asset_server.load("fonts/FiraCode-Regular.ttf"),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.4, 0.4, 0.5)),
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(30.0),
                    ..default()
                },
            ));
        });
}

fn handle_pause_input(
    mut keyboard_events: MessageReader<KeyboardInput>,
    mut pause_selection: ResMut<PauseSelection>,
    mut next_menu: ResMut<NextState<Menu>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    for event in keyboard_events.read() {
        if event.state != ButtonState::Pressed {
            continue;
        }

        match event.key_code {
            KeyCode::ArrowUp | KeyCode::KeyW => {
                if pause_selection.current > 0 {
                    pause_selection.current -= 1;
                }
            }
            KeyCode::ArrowDown | KeyCode::KeyS => {
                if pause_selection.current < pause_selection.max {
                    pause_selection.current += 1;
                }
            }
            KeyCode::Enter | KeyCode::Space => {
                let action = match pause_selection.current {
                    0 => PauseAction::Continue,
                    1 => PauseAction::Settings,
                    2 => PauseAction::QuitToTitle,
                    _ => return,
                };

                match action {
                    PauseAction::Continue => {
                        next_menu.set(Menu::None);
                    }
                    PauseAction::Settings => {
                        next_menu.set(Menu::Settings);
                    }
                    PauseAction::QuitToTitle => {
                        next_screen.set(Screen::Title);
                    }
                }
            }
            KeyCode::Escape => {
                next_menu.set(Menu::None);
            }
            _ => {}
        }
    }
}

fn update_pause_selection(
    pause_selection: Res<PauseSelection>,
    mut query: Query<(&PauseOption, &mut TextColor)>,
) {
    if !pause_selection.is_changed() {
        return;
    }

    for (option, mut color) in &mut query {
        if option.index == pause_selection.current {
            // Selected - cyan to match terminal
            color.0 = Color::srgb(0.0, 1.0, 0.8);
        } else {
            // Not selected - dim gray
            color.0 = Color::srgb(0.5, 0.5, 0.6);
        }
    }
}
