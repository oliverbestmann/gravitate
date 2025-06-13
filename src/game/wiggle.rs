use crate::AppSystems;
use bevy::prelude::*;
use fastnoise_lite::FastNoiseLite;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_wiggle.in_set(AppSystems::Update));
}

#[derive(Component)]
#[require(Transform)]
pub struct Wiggle {
    pub noise: FastNoiseLite,
    pub scale_rotation: f32,
    pub scale_transform: f32,
    pub offset: Vec2,
    pub offset_angle: f32,
}

impl Wiggle {
    pub fn with_seed(seed: i32) -> Self {
        let mut noise = FastNoiseLite::with_seed(seed);
        noise.frequency = 3.0;

        Self {
            noise,
            scale_rotation: 2.0_f32.to_radians(),
            scale_transform: 1.0,
            offset: Vec2::ZERO,
            offset_angle: 0.0,
        }
    }
}

fn update_wiggle(time: Res<Time>, query: Query<(&mut Transform, &Wiggle)>) {
    for (mut transform, wiggle) in query {
        // get random rotation and scale it with the wiggle factor
        let amount = wiggle.noise.get_noise_2d(time.elapsed_secs(), 0.0);
        let rotation = wiggle.offset_angle + amount * wiggle.scale_rotation;

        // offset by +/- one pixel
        let wiggle_x = wiggle.noise.get_noise_2d(time.elapsed_secs(), 10.0);
        let wiggle_y = wiggle.noise.get_noise_2d(time.elapsed_secs(), 20.0);

        transform.rotation = Quat::from_rotation_z(rotation);
        transform.translation.x = wiggle.offset.x + wiggle_x * wiggle.scale_transform;
        transform.translation.y = wiggle.offset.y + wiggle_y * wiggle.scale_transform;
    }
}
