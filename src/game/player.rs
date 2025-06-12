use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    // app.add_systems(Update, foobar);
}

#[derive(Component, Reflect)]
pub struct Player;
