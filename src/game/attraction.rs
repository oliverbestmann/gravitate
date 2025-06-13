use crate::AppSystems;
use crate::common::pause::PausableSystems;
use crate::screens::Screen;
use avian2d::prelude::{ComputedMass, ExternalForce};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        apply_attraction_forces
            .run_if(in_state(Screen::Gameplay))
            .in_set(PausableSystems)
            .in_set(AppSystems::PrePhysics),
    );
}

#[derive(Component, Reflect)]
pub struct Attractor;

#[derive(Component, Reflect)]
pub struct Attractable;

fn apply_attraction_forces(
    attractors: Query<(&Transform, &ComputedMass), With<Attractor>>,
    attractable: Query<(&Transform, &mut ExternalForce), With<Attractable>>,
) {
    let attractors: Vec<_> = attractors.into_iter().collect();

    for (transform, mut force) in attractable {
        for (attr_transform, attr_mass) in &attractors {
            let direction = attr_transform.translation.xy() - transform.translation.xy();
            let amount = attr_mass.value() / direction.length_squared();
            force.apply_force(direction.normalize() * amount);
        }
    }
}
