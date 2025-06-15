use crate::common::pause::PausableSystems;
use crate::game::cv::{
    LAYER_OFFSET_ROCKET_FIN_BG, LAYER_OFFSET_ROCKET_FIN_FG, LAYER_OFFSET_ROCKET_PLUME,
};
use crate::game::player::Thrust;
use crate::game::shadow::Shadow;
use crate::game::wiggle::Wiggle;
use crate::screens::Screen;
use crate::{AppSystems, game};
use avian2d::prelude::{Collider, ExternalForce, RigidBody};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use std::f32::consts::PI;
use std::time::Duration;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            apply_thrust,
            apply_plume_visibility,
            rotate_direction_of_thrust,
        )
            .run_if(in_state(Screen::Gameplay))
            .in_set(AppSystems::PrePhysics)
            .in_set(PausableSystems),
    );
}

#[derive(Component)]
pub struct Rocket;

#[derive(Component)]
pub struct Plume;

#[derive(Component, Reflect)]
pub struct FuelTank {
    pub capacity: Fuel,
    pub remaining: Fuel,
}

impl FuelTank {
    pub fn full(capacity: Fuel) -> Self {
        FuelTank {
            capacity,
            remaining: capacity,
        }
    }

    pub fn burn(&mut self, amount: Duration) {
        self.remaining.burn(amount);
    }
}

#[derive(Copy, Clone, Debug, Reflect)]
pub struct Fuel(Duration);

impl Fuel {
    pub fn new(amount: Duration) -> Self {
        Self(amount)
    }

    pub fn from_secs(seconds: u64) -> Self {
        Self::new(Duration::from_secs(seconds))
    }

    pub fn as_secs(&self) -> f32 {
        self.0.as_secs_f32()
    }

    pub fn burn(&mut self, amount: Duration) {
        self.0 = self.0.saturating_sub(amount);
    }
}

pub fn bundle(assets: &game::Assets) -> impl Bundle {
    let fin_offset = vec2(0., -40.);

    // spawn the player
    (
        RigidBody::Dynamic,
        Collider::capsule(32., 48.),
        Rocket,
        Visibility::Inherited,
        children![
            (
                Name::new("PlumeGroup"),
                Plume,
                Transform::from_xyz(0., -56.0, 0.),
                Visibility::Hidden,
                children![
                    plume_bundle(assets, 0),
                    plume_bundle(assets, 1),
                    plume_bundle(assets, 2),
                ],
            ),
            (
                Name::new("Body"),
                Shadow::default(),
                Wiggle::default(),
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
                    ..default()
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
                    ..default()
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

fn plume_bundle(assets: &game::Assets, idx: i32) -> impl Bundle {
    (
        Name::new("Plume"),
        Shadow::default(),
        Wiggle {
            scale_rotation: 10f32.to_radians(),
            ..default()
        },
        LAYER_OFFSET_ROCKET_PLUME.offset_by(idx),
        Sprite {
            image: assets.plume[idx as usize].clone(),
            anchor: Anchor::Center,
            ..default()
        },
    )
}

fn apply_thrust(
    mut commands: Commands,
    rocket: Query<
        (
            Entity,
            &mut Thrust,
            &mut ExternalForce,
            Option<&mut FuelTank>,
        ),
        With<Rocket>,
    >,
    time: Res<Time>,
) {
    for (entity, mut thrust, mut external_force, fuel_tank) in rocket {
        // reduce remaining thrust
        thrust.remaining = thrust.remaining.saturating_sub(time.delta());

        // reduce fuel in tank
        if let Some(mut tank) = fuel_tank {
            tank.burn(time.delta());
        }

        if thrust.remaining == Duration::ZERO {
            info!("Thrust has finished");
            // remove thrust component once thrust stopped
            commands.entity(entity).remove::<Thrust>();
        }

        // apply last bit of thrust
        external_force.apply_force(thrust.force);
    }
}

fn rotate_direction_of_thrust(
    rocket: Query<(&mut Transform, &Thrust), With<Rocket>>,
    time: Res<Time>,
) {
    for (mut transform, thrust) in rocket {
        // the sprites are not oriented correctly, for the rocket, forwards is up.
        // we need to fix this by applying an offset to the intended rotation
        let offset = -PI / 2.0;

        // target a rotation into the direction of the force to be applied
        let target = Quat::from_rotation_z(thrust.force.to_angle() + offset);

        transform
            .rotation
            .smooth_nudge(&target, 5.0, time.delta_secs());
    }
}

fn apply_plume_visibility(
    rockets: Query<Has<Thrust>, With<Rocket>>,
    plumes: Query<(&ChildOf, &mut Visibility), With<Plume>>,
) {
    for (child_of, mut plume_visibility) in plumes {
        let has_thrust = rockets.get(child_of.parent()).unwrap_or_default();

        let visibility = if has_thrust {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };

        plume_visibility.set_if_neq(visibility);
    }
}
