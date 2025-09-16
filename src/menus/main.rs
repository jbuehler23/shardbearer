//! The main menu (seen on the title screen).

use bevy::prelude::*;

use crate::{asset_tracking::ResourceHandles, menus::Menu, screens::Screen, theme::widget};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), (spawn_main_menu, spawn_title_text));
}

fn spawn_title_text(mut commands: Commands) {
    commands.spawn((
        Name::new("Title Text"),
        Node {
            width: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            top: Val::Px(50.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        GlobalZIndex(3),
        DespawnOnExit(Menu::Main),
    ))
    .with_children(|parent| {
        // Main title
        parent.spawn((
            Text("SHARDBEARER".to_string()),
            TextFont {
                font_size: 72.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.0, 1.0)), // Magenta
        ));

        // Subtitle
        parent.spawn((
            Text("A New Carthage Story".to_string()),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::srgb(0.0, 1.0, 0.8)), // Cyan
            Node {
                margin: UiRect::top(Val::Px(10.0)),
                ..default()
            },
        ));
    });
}

fn spawn_main_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root(""),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Main),
        #[cfg(not(target_family = "wasm"))]
        children![
            widget::button("Play", enter_loading_or_gameplay_screen),
            widget::button("Settings", open_settings_menu),
            widget::button("Credits", open_credits_menu),
            widget::button("Exit", exit_app),
        ],
        #[cfg(target_family = "wasm")]
        children![
            widget::button("Play", enter_loading_or_gameplay_screen),
            widget::button("Settings", open_settings_menu),
            widget::button("Credits", open_credits_menu),
        ],
    ));
}

fn enter_loading_or_gameplay_screen(
    _: On<Pointer<Click>>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if resource_handles.is_all_done() {
        next_screen.set(Screen::Gameplay);
    } else {
        next_screen.set(Screen::Loading);
    }
}

fn open_settings_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn open_credits_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Credits);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: On<Pointer<Click>>, mut app_exit: MessageWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
