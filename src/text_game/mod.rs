pub mod game;
pub mod story;
pub mod terminal;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        game::plugin,
        story::plugin,
        terminal::plugin,
    ));
}