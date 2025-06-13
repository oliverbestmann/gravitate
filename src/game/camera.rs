use crate::game::player::Player;
use crate::{AppSystems, MainCamera};
use avian2d::prelude::LinearVelocity;
use bevy::math::ops::ln;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, follow_player.in_set(AppSystems::UpdateCamera));
}

fn follow_player(
    mut camera: Single<&mut Transform, With<MainCamera>>,
    time: Res<Time>,
    query_player: Query<(&Transform, &LinearVelocity), (With<Player>, Without<MainCamera>)>,
) {
    let Ok((player_transform, player_velocity)) = query_player.single() else {
        return;
    };

    let mut current = camera.translation.xy();

    // target the position the player might be soon
    let offset = (player_velocity.0 * 4.0).clamp_length_max(512.0);
    let target = player_transform.translation.xy() + offset;

    // nudge the position a little
    current.smooth_nudge(&target, ln(5.0), time.delta_secs());

    camera.translation.x = current.x;
    camera.translation.y = current.y;
}
