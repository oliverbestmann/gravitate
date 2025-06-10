use bevy::app::App;
use bevy::prelude::States;

mod gameplay;
mod loading;
mod title;

/// The game's main screen states.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum Screen {
    #[default]
    Loading,
    Title,
    Gameplay,
}

pub fn plugin(app: &mut App) {
    app.add_plugins((loading::plugin, gameplay::plugin, loading::plugin));
}
