use bevy::app::App;

pub mod interaction;
pub mod widget;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((interaction::plugin,));
}
