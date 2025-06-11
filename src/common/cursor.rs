use crate::MainCamera;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn plugin(app: &mut App) {
    app.init_resource::<WorldCursor>();
    app.add_systems(PreUpdate, update_world_cursor);
}

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
pub struct WorldCursor(pub Vec2);

fn update_world_cursor(
    mut coords: ResMut<WorldCursor>,
    // query to get the window (so we can read the current cursor position)
    window: Single<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    let (camera, camera_transform) = &*camera;

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    {
        coords.0 = world_position;
    }
}
