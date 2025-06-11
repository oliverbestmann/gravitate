use crate::game;
use crate::game::shadow::Shadow;
use crate::screens::Screen;
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_level);
}

pub fn spawn_level(mut commands: Commands, assets: Res<game::Assets>) {
    // spawn the player
    commands.spawn((
        Sprite {
            image: assets.player.clone(),
            anchor: Anchor::Center,
            custom_size: Some(Vec2::splat(64.0)),
            ..default()
        },
        Shadow::default(),
    ));
}
