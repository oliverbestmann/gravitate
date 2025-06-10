use bevy::app::App;

pub mod cursor;
pub mod markers;
pub mod squishy;
pub mod rand;

pub fn plugin(app: &mut App) {
    app.add_plugins((cursor::plugin, markers::plugin, squishy::plugin, rand::plugin));
}
