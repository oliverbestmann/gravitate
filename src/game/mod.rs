use crate::screens::Screen;
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub mod assets;

pub use assets::Assets;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        assets::plugin,
        // add plugins here
    ));

    app.add_systems(OnEnter(Screen::Gameplay), (spawn_level,));

    // configure game physics
    // app.insert_resource(Gravity::ZERO);
    // app.insert_resource(SubstepCount(6));
    // app.insert_resource(DefaultFriction(Friction::new(0.0)));
}

pub fn spawn_level(mut commands: Commands, assets: Res<Assets>) {
    // spawn the player
    commands.spawn(
        (Sprite {
            image: assets.player.clone(),
            anchor: Anchor::Center,
            custom_size: Some(Vec2::splat(64.0)),
            ..default()
        }),
    );
}

#[cfg(target_arch = "wasm32")]
fn player_name() -> Option<String> {
    let Some(window) = web_sys::window() else {
        return "Unknown".into();
    };

    window
        .get("Player")
        .and_then(|f| f.as_string())
        .filter(|name| name.chars().any(|ch| !ch.is_whitespace()))
}

#[cfg(not(target_arch = "wasm32"))]
fn player_name() -> Option<String> {
    std::env::var("USER")
        .ok()
        .filter(|name| name.chars().any(|ch| !ch.is_whitespace()))
}
