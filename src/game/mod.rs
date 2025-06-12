use avian2d::prelude::Gravity;
use bevy::prelude::*;

pub mod assets;
pub mod camera;
pub mod cv;
pub mod level;
pub mod player;
pub mod shadow;
pub mod wiggle;
pub mod rocket;
pub mod layer;

pub use assets::Assets;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        assets::plugin,
        shadow::plugin,
        level::plugin,
        wiggle::plugin,
        camera::plugin,
        player::plugin,
        rocket::plugin,
        layer::plugin,
    ));

    app.insert_resource(ClearColor(cv::COLOR_BACKGROUND));

    // configure game physics
    app.insert_resource(Gravity::ZERO);
    // app.insert_resource(SubstepCount(6));
    // app.insert_resource(DefaultFriction(Friction::new(0.0)));
}
