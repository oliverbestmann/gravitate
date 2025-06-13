use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    // Set up the `Pause` state.
    app.init_state::<Pause>();
    app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));

    app.add_systems(OnEnter(Pause(true)), pause_stop_time);
    app.add_systems(OnEnter(Pause(false)), pause_resume_time);
}

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub struct Pause(pub bool);

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PausableSystems;

fn pause_stop_time(mut time: ResMut<Time<Virtual>>) {
    time.pause();
}

fn pause_resume_time(mut time: ResMut<Time<Virtual>>) {
    time.unpause();
}
