use crate::screens::Screen;
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub mod assets;
pub mod cv;

use crate::game::cv::COLOR_BACKGROUND;
pub use assets::Assets;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        assets::plugin,
        // add plugins here
    ));

    app.add_systems(OnEnter(Screen::Gameplay), (spawn_level,));

    app.insert_resource(ClearColor(COLOR_BACKGROUND));

    // configure game physics
    // app.insert_resource(Gravity::ZERO);
    // app.insert_resource(SubstepCount(6));
    // app.insert_resource(DefaultFriction(Friction::new(0.0)));
}

pub fn spawn_level(mut commands: Commands, assets: Res<Assets>) {
    // spawn the player
    commands.spawn((Sprite {
        image: assets.player.clone(),
        anchor: Anchor::Center,
        custom_size: Some(Vec2::splat(64.0)),
        ..default()
    },));
}
