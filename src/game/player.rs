use crate::game::input::InputTransformContext;
use crate::MainCamera;
use bevy::prelude::*;
use bevy_vector_shapes::painter::ShapePainter;

pub(super) fn plugin(app: &mut App) {
    // app.add_systems(Update, foobar);
}

#[derive(Component, Reflect)]
pub struct Player;
