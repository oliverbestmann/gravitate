use bevy::app::App;

pub mod cursor;
pub mod markers;
pub mod pause;
pub mod rand;
pub mod squishy;
pub mod stopwatch;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        cursor::plugin,
        markers::plugin,
        squishy::plugin,
        rand::plugin,
        pause::plugin,
    ));
}
