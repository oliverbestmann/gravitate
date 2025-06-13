use avian2d::prelude::Gravity;
use bevy::prelude::*;

pub mod assets;
pub mod attraction;
pub mod camera;
pub mod cv;
pub mod input;
pub mod input_viz;
pub mod layer;
pub mod level;
pub mod planet;
pub mod player;
pub mod rocket;
pub mod shadow;
pub mod wiggle;

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
        input::plugin,
        input_viz::plugin,
        planet::plugin,
        attraction::plugin,
    ));

    app.insert_resource(ClearColor(cv::COLOR_BACKGROUND));

    // configure game physics
    app.insert_resource(Gravity::ZERO);
    // app.insert_resource(SubstepCount(6));
    // app.insert_resource(DefaultFriction(Friction::new(0.0)));
}
