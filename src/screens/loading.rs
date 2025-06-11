//! A loading screen during which game assets are loaded if necessary.
//! This reduces stuttering, especially for audio on Wasm.

use bevy::asset::embedded_asset;
use bevy::image::{ImageLoaderSettings, ImageSampler};
use bevy::prelude::*;

use crate::ui::widget;
use crate::{asset_tracking::ResourceHandles, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    embedded_asset!(app, "loading.jpg");

    app.add_systems(OnEnter(Screen::Loading), spawn_loading_screen);

    app.add_systems(
        Update,
        enter_gameplay_screen.run_if(
            in_state(Screen::Loading)
                .and(all_assets_loaded)
                .and(is_loading_time_reached),
        ),
    );
}

fn spawn_loading_screen(mut commands: Commands, assets: ResMut<AssetServer>) {
    commands.spawn((
        widget::ui_root("Loading Screen"),
        StateScoped(Screen::Loading),
        children![(
            Name::new("Splash image"),
            Node {
                margin: UiRect::all(Val::Auto),
                width: Val::Percent(100.0),
                ..default()
            },
            ImageNode::new(assets.load_with_settings(
                "embedded://gravitate/screens/loading.jpg",
                |settings: &mut ImageLoaderSettings| {
                    // Make an exception for the splash image in case
                    // `ImagePlugin::default_nearest()` is used for pixel art.
                    settings.sampler = ImageSampler::linear();
                },
            )),
        )],
    ));
}

fn is_loading_time_reached(time: Res<Time>) -> bool {
    let delay = if (cfg!(debug_assertions)) { 0.1 } else { 0.5 };
    time.elapsed_secs() > delay
}

fn enter_gameplay_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

fn all_assets_loaded(resource_handles: Res<ResourceHandles>) -> bool {
    resource_handles.is_all_done()
}
