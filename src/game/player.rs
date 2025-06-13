use crate::common::rand::Rand;
use crate::game;
use crate::game::attraction::Attractable;
use crate::game::cv::LAYER_ROCKET;
use crate::game::input::{Input, InputActive, OnThurst};
use crate::game::rocket;
use avian2d::prelude::{ExternalForce, LinearVelocity};
use bevy::prelude::*;
use std::time::Duration;

pub(super) fn plugin(_app: &mut App) {
    // TODO
}

#[derive(Component, Reflect)]
pub struct Player;

#[derive(Component, Reflect)]
pub struct Thrust {
    pub remaining: Duration,
    pub force: Vec2,
}

pub fn bundle(assets: &game::Assets, rand: &mut Rand) -> impl Bundle {
    (
        rocket::bundle(assets, rand),
        LAYER_ROCKET,
        LinearVelocity::ZERO,
        ExternalForce::ZERO.with_persistence(false),
        Attractable,
        Player,
        Input,
    )
}

pub fn slow_time_on_input(
    _: Trigger<OnAdd, InputActive>,
    mut time: ResMut<Time<Virtual>>,
    mut fixed: ResMut<Time<Fixed>>,
) {
    time.set_relative_speed(0.05);
    fixed.set_timestep_hz(64.0 / 0.05);
}

pub fn reset_time_after_input(
    _: Trigger<OnRemove, (InputActive, Player)>,
    mut time: ResMut<Time<Virtual>>,
    mut fixed: ResMut<Time<Fixed>>,
) {
    time.set_relative_speed(1.0);
    fixed.set_timestep_hz(64.0);
}

pub fn handle_on_thrust(
    trigger: Trigger<OnThurst>,
    mut commands: Commands,
    player: Single<Entity, With<Player>>,
) {
    commands.entity(*player).insert(Thrust {
        force: trigger.direction * 100_000.0,
        remaining: trigger.duration,
    });
}
