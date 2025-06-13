use crate::common::rand::Rand;
use crate::game::assets::PlanetAssets;
use crate::game::attraction::Attractor;
use crate::game::cv::LAYER_PLANETS;
use crate::game::wiggle::Wiggle;
use avian2d::prelude::{Collider, ColliderDensity, RigidBody};
use bevy::ecs::spawn::SpawnIter;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::Rng;

pub(super) fn plugin(_app: &mut App) {
    // app.add_systems(Update, foobar);
}

pub struct Planet;

pub fn bundle(assets: &PlanetAssets, rand: &mut Rand, radius: f32) -> impl Bundle {
    let mut layer_bundle = |idx: usize, image: &Handle<Image>| {
        (
            LAYER_PLANETS.offset_by(idx as i32),
            Wiggle {
                scale_rotation: 0.5_f32.to_radians(),
                ..Wiggle::with_seed(rand.random())
            },
            Sprite {
                image: image.clone(),
                custom_size: Some(Vec2::splat(2.0 * radius)),
                anchor: Anchor::Center,
                ..default()
            },
        )
    };

    let children: Vec<_> = assets
        .images
        .iter()
        .enumerate()
        .map(|(idx, image)| layer_bundle(idx, image))
        .collect();

    (
        RigidBody::Static,
        Collider::circle(radius),
        ColliderDensity(100000.0),
        Attractor,
        Children::spawn(SpawnIter(children.into_iter())),
    )
}
