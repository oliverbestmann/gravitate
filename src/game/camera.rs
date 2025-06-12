use crate::{AppSystems, MainCamera};
use bevy::prelude::*;
use crate::game::player::Player;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, follow_player.in_set(AppSystems::UpdateCamera));
}

fn follow_player(
    mut camera: Single<&mut Transform, With<MainCamera>>,
    query_player: Query<&Transform, (With<Player>, Without<MainCamera>)>,
) {
    let Ok(player) = query_player.single() else {
        return;
    };

    camera.translation.x = player.translation.x;
    camera.translation.y = player.translation.y;
}
