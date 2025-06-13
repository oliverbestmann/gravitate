use crate::common::rand::{Generate, Rand};
use crate::game;
use crate::game::cv::{LAYER_ROCKET, LAYER_STARS};
use crate::game::rocket;
use crate::game::shadow::Shadow;
use crate::game::wiggle::Wiggle;
use crate::screens::Screen;
use avian2d::prelude::{LinearVelocity, Sensor};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::prelude::IndexedRandom;
use rand::Rng;
use std::f32::consts::PI;
use crate::game::input::Input;
use crate::game::player::Player;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), (spawn_level, spawn_stars));
}

pub fn spawn_level(mut commands: Commands, mut rand: ResMut<Rand>, assets: Res<game::Assets>) {
    commands.spawn((
        Name::new("Player"),
        StateScoped(Screen::Gameplay),
        LinearVelocity(vec2(0., 100.)),
        Sensor,
        Player,
        Input,
        LAYER_ROCKET,
        rocket::bundle(&assets, &mut rand),
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
                ..Wiggle::with_seed(rand.random())
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
