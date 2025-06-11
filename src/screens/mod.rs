use bevy::app::App;
use bevy::prelude::{AppExtStates, States};

mod gameplay;
mod loading;
mod title;

pub fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.add_plugins((loading::plugin, gameplay::plugin, title::plugin));
}

/// The game's main screen states.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum Screen {
    #[default]
    Loading,
    Title,
    Gameplay,
}
