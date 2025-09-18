//! The main menu (seen on the title screen).

use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

use crate::{asset_tracking::ResourceHandles, menus::Menu, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
    app.add_systems(
        Update,
        (handle_menu_input, update_menu_selection).run_if(in_state(Menu::Main)),
    );
}

#[derive(Component)]
struct MainMenuContainer;

#[derive(Component)]
struct MenuOption {
    index: usize,
    action: MenuAction,
}

#[derive(Clone, Copy)]
enum MenuAction {
    Play,
    Settings,
    Credits,
    #[cfg(not(target_family = "wasm"))]
    Exit,
}

#[derive(Resource)]
struct MenuSelection {
    current: usize,
    max: usize,
}

const BANNER: &str = r#"
    ███████╗██╗  ██╗ █████╗ ██████╗ ██████╗ ██████╗ ███████╗ █████╗ ██████╗ ███████╗██████╗
    ██╔════╝██║  ██║██╔══██╗██╔══██╗██╔══██╗██╔══██╗██╔════╝██╔══██╗██╔══██╗██╔════╝██╔══██╗
    ███████╗███████║███████║██████╔╝██║  ██║██████╔╝█████╗  ███████║██████╔╝█████╗  ██████╔╝
    ╚════██║██╔══██║██╔══██║██╔══██╗██║  ██║██╔══██╗██╔══╝  ██╔══██║██╔══██╗██╔══╝  ██╔══██╗
    ███████║██║  ██║██║  ██║██║  ██║██████╔╝██████╔╝███████╗██║  ██║██║  ██║███████╗██║  ██║
    ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚═════╝ ╚═════╝ ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝

             ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
             ▓                  A   N E W   C A R T H A G E   S T O R Y              ▓
             ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
"#;

fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Initialize menu selection
    #[cfg(not(target_family = "wasm"))]
    let max_selection = 3;
    #[cfg(target_family = "wasm")]
    let max_selection = 2;

    commands.insert_resource(MenuSelection {
        current: 0,
        max: max_selection,
    });

    // Terminal-style container
    commands
        .spawn((
            Name::new("Main Menu Container"),
            MainMenuContainer,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.01, 0.01, 0.05)), // Match terminal dark blue-black
            GlobalZIndex(2),
            DespawnOnExit(Menu::Main),
        ))
        .with_children(|parent| {
            // Banner section
            parent.spawn((
                Text(BANNER.to_string()),
                TextFont {
                    font: asset_server.load("fonts/FiraCode-Regular.ttf"),
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 1.0, 0.8)), // Cyan to match terminal
                TextLayout::default(),
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
                    // Play option
                    parent.spawn((
                        MenuOption {
                            index: 0,
                            action: MenuAction::Play,
                        },
                        Text(format!("  {}  ", "Play")),
                        TextFont {
                            font: asset_server.load("fonts/FiraCode-Regular.ttf"),
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.0, 1.0, 0.8)), // Cyan - selected by default
                    ));

                    // Settings option
                    parent.spawn((
                        MenuOption {
                            index: 1,
                            action: MenuAction::Settings,
                        },
                        Text(format!("  {}  ", "Settings")),
                        TextFont {
                            font: asset_server.load("fonts/FiraCode-Regular.ttf"),
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.5, 0.5, 0.6)),
                    ));

                    // Credits option
                    parent.spawn((
                        MenuOption {
                            index: 2,
                            action: MenuAction::Credits,
                        },
                        Text(format!("  {}  ", "Credits")),
                        TextFont {
                            font: asset_server.load("fonts/FiraCode-Regular.ttf"),
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.5, 0.5, 0.6)),
                    ));

                    // Exit option (desktop only)
                    #[cfg(not(target_family = "wasm"))]
                    parent.spawn((
                        MenuOption {
                            index: 3,
                            action: MenuAction::Exit,
                        },
                        Text(format!("  {}  ", "Exit")),
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
                Text("↑↓ Navigate  •  Enter Select".to_string()),
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

fn handle_menu_input(
    mut keyboard_events: MessageReader<KeyboardInput>,
    mut menu_selection: ResMut<MenuSelection>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut next_menu: ResMut<NextState<Menu>>,
    #[cfg(not(target_family = "wasm"))] mut app_exit: MessageWriter<AppExit>,
) {
    for event in keyboard_events.read() {
        if event.state != ButtonState::Pressed {
            continue;
        }

        match event.key_code {
            KeyCode::ArrowUp | KeyCode::KeyW => {
                if menu_selection.current > 0 {
                    menu_selection.current -= 1;
                }
            }
            KeyCode::ArrowDown | KeyCode::KeyS => {
                if menu_selection.current < menu_selection.max {
                    menu_selection.current += 1;
                }
            }
            KeyCode::Enter | KeyCode::Space => {
                let action = match menu_selection.current {
                    0 => MenuAction::Play,
                    1 => MenuAction::Settings,
                    2 => MenuAction::Credits,
                    #[cfg(not(target_family = "wasm"))]
                    3 => MenuAction::Exit,
                    _ => return,
                };

                match action {
                    MenuAction::Play => {
                        if resource_handles.is_all_done() {
                            next_screen.set(Screen::Gameplay);
                        } else {
                            next_screen.set(Screen::Loading);
                        }
                    }
                    MenuAction::Settings => {
                        next_menu.set(Menu::Settings);
                    }
                    MenuAction::Credits => {
                        next_menu.set(Menu::Credits);
                    }
                    #[cfg(not(target_family = "wasm"))]
                    MenuAction::Exit => {
                        app_exit.write(AppExit::Success);
                    }
                }
            }
            _ => {}
        }
    }
}

fn update_menu_selection(
    menu_selection: Res<MenuSelection>,
    mut query: Query<(&MenuOption, &mut TextColor)>,
) {
    if !menu_selection.is_changed() {
        return;
    }

    for (option, mut color) in &mut query {
        if option.index == menu_selection.current {
            // Selected - cyan to match terminal
            color.0 = Color::srgb(0.0, 1.0, 0.8);
        } else {
            // Not selected - dim gray
            color.0 = Color::srgb(0.5, 0.5, 0.6);
        }
    }
}
