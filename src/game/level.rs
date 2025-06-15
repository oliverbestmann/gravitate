use crate::common::rand::{Generate, Rand};
use crate::game;
use crate::game::cv::LAYER_STARS;
use crate::game::planet::CropCache;
use crate::game::shadow::Shadow;
use crate::game::wiggle::Wiggle;
use crate::game::{planet, player};
use crate::screens::Screen;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::Rng;
use rand::prelude::IndexedRandom;
use std::f32::consts::PI;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), (spawn_level, spawn_stars));
}

pub fn spawn_level(
    mut commands: Commands,
    assets: Res<game::Assets>,
    mut crop_cache: ResMut<CropCache>,
    mut images: ResMut<Assets<Image>>,
) {
    commands
        .spawn((
            Name::new("Player"),
            StateScoped(Screen::Gameplay),
            player::bundle(&assets),
        ))
        .observe(player::slow_time_on_input)
        .observe(player::reset_time_after_input)
        .observe(player::handle_on_thrust);

    // spawn a planet
    commands.spawn((
        Transform::from_xyz(-50., 350., 0.),
        planet::bundle(&assets.planets[0], &mut crop_cache, &mut images, 128.0),
    ));

    // spawn a planet
    commands.spawn((
        Transform::from_xyz(100., 650., 0.),
        planet::bundle(&assets.planets[1], &mut crop_cache, &mut images, 128.0),
    ));
    // spawn a planet
    commands.spawn((
        Transform::from_xyz(60., -250., 0.),
        planet::bundle(&assets.planets[2], &mut crop_cache, &mut images, 96.0),
    ));
}

fn spawn_stars(mut commands: Commands, mut rand: ResMut<Rand>, assets: Res<game::Assets>) {
    let mut g = Generate::new(4096.0, 0.0, vec2(0.0, 1024.0));
    let points = g.generate(|radius| rand.vec2() * radius, 500, 256.0);

    for point in points {
        let rotation = rand.random_range(0. ..2.0 * PI);

        commands.spawn((
            Name::new("Star"),
            StateScoped(Screen::Gameplay),
            LAYER_STARS,
            Shadow::default(),
            Wiggle {
                offset: point.floor(),
                offset_angle: rotation,
                ..default()
            },
            Sprite {
                image: Handle::clone(
                    [&assets.star_small, &assets.star_large]
                        .choose(&mut rand)
                        .unwrap(),
                ),
                anchor: Anchor::Center,
                ..default()
            },
        ));
    }
}
