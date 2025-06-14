use crate::AppSystems;
use crate::common::rand::Rand;
use bevy::prelude::*;
use fastnoise_lite::FastNoiseLite;
use rand::Rng;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_wiggle.in_set(AppSystems::Update));

    // initialize seed for new wiggles
    app.add_observer(initialize_wiggle);
}

#[derive(Component)]
#[require(Transform)]
pub struct Wiggle {
    pub seed: i32,
    pub scale_rotation: f32,
    pub scale_transform: f32,
    pub offset: Vec2,
    pub offset_angle: f32,
}

impl Wiggle {
    pub fn with_offset(offset: Vec2) -> Self {
        Self {
            offset,
            ..default()
        }
    }
}

impl Default for Wiggle {
    fn default() -> Self {
        Self {
            seed: 0,
            scale_rotation: 2.0_f32.to_radians(),
            scale_transform: 1.0,
            offset: Vec2::ZERO,
            offset_angle: 0.0,
        }
    }
}

fn initialize_wiggle(
    trigger: Trigger<OnAdd, Wiggle>,
    wiggles: Query<&mut Wiggle>,
    mut rand: ResMut<Rand>,
) -> Result {
    let mut wiggle = wiggles.get_inner(trigger.target())?;
    wiggle.seed = rand.random();

    Ok(())
}

fn update_wiggle(time: Res<Time>, query: Query<(&mut Transform, &Wiggle)>) {
    for (mut transform, wiggle) in query {
        let mut noise = FastNoiseLite::with_seed(wiggle.seed);
        noise.frequency = 3.0;

        // get random rotation and scale it with the wiggle factor
        let amount = noise.get_noise_2d(time.elapsed_secs(), 0.0);
        let rotation = wiggle.offset_angle + amount * wiggle.scale_rotation;

        // offset by +/- one pixel
        let wiggle_x = noise.get_noise_2d(time.elapsed_secs(), 10.0);
        let wiggle_y = noise.get_noise_2d(time.elapsed_secs(), 20.0);

        transform.rotation = Quat::from_rotation_z(rotation);
        transform.translation.x = wiggle.offset.x + wiggle_x * wiggle.scale_transform;
        transform.translation.y = wiggle.offset.y + wiggle_y * wiggle.scale_transform;
    }
}
