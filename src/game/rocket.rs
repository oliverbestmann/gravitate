use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::Rng;
use crate::common::rand::Rand;
use crate::game;
use crate::game::cv::{LAYER_ROCKET, LAYER_OFFSET_ROCKET_FIN_BG, LAYER_OFFSET_ROCKET_FIN_FG, LAYER_OFFSET_ROCKET_PLUME};
use crate::game::shadow::Shadow;
use crate::game::wiggle::Wiggle;

pub(super) fn plugin(app: &mut App) {
    // app.add_systems(Update, foobar);
}

#[derive(Component)]
pub struct Rocket;

#[derive(Component)]
pub struct Plume;



pub fn bundle(
    assets: &game::Assets,
    rand: &mut Rand,
) -> impl Bundle {
    let fin_offset = vec2(0., -40.);

    let mut plume = |idx: i32| {
        (
            Name::new("Plume"),
            Shadow::default(),
            Wiggle::with_seed(rand.random()),
            LAYER_OFFSET_ROCKET_PLUME.offset_by(idx),
            Sprite {
                image: assets.plume[idx as usize].clone(),
                anchor: Anchor::Center,
                ..default()
            },
        )
    };

    // spawn the player
    (
        RigidBody::Dynamic,
        Collider::capsule(32., 48.),
        Rocket,
        children![
            (
                Name::new("PlumeGroup"),
                Plume,
                Transform::from_xyz(0., -56.0, 0.),
                children![plume(0), plume(1), plume(2),],
            ),
            (
                Name::new("Body"),
                Shadow::default(),
                Wiggle::with_seed(rand.random()),
                Sprite {
                    image: assets.rocket_base.clone(),
                    anchor: Anchor::Center,
                    ..default()
                },
            ),
            (
                Name::new("FinBG"),
                Shadow::default(),
                Wiggle {
                    offset: fin_offset,
                    scale_rotation: 0.5_f32.to_radians(),
                    scale_transform: 0.5,
                    ..Wiggle::with_seed(rand.random())
                },
                LAYER_OFFSET_ROCKET_FIN_BG,
                Sprite {
                    image: assets.rocket_fin_bg.clone(),
                    anchor: Anchor::Center,
                    ..default()
                },
            ),
            (
                Name::new("FinFG"),
                Shadow::default(),
                Wiggle {
                    offset: fin_offset,
                    scale_rotation: 0.5_f32.to_radians(),
                    scale_transform: 0.5,
                    ..Wiggle::with_seed(rand.random())
                },
                LAYER_OFFSET_ROCKET_FIN_FG,
                Sprite {
                    image: assets.rocket_fin_fg.clone(),
                    anchor: Anchor::Center,
                    ..default()
                },
            ),
        ],
    )
}
