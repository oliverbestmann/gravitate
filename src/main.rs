// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use avian2d::prelude::PhysicsSet;
use bevy::render::camera;
use bevy::window::WindowResolution;
use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod asset_tracking;
pub mod common;
pub mod game;
pub mod menus;
pub mod player_name;
pub mod screens;
pub mod ui;

fn main() -> AppExit {
    App::new().add_plugins(app_plugin).run()
}

const WORLD_SCALE: f32 = 2.0;

fn app_plugin(app: &mut App) {
    let window_scale_factor = 2.0;

    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                // Wasm builds will check for meta files (that don't exist) if this isn't set.
                // This causes errors and even panics on web build on itch.
                // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Window {
                    title: "Gravitate".to_string(),
                    fit_canvas_to_parent: true,
                    resolution: WindowResolution::new(
                        window_scale_factor * 512.0,
                        window_scale_factor * 768.0,
                    )
                    .with_scale_factor_override(window_scale_factor),
                    ..default()
                }
                .into(),
                ..default()
            }),
    );

    app.add_plugins((
        avian2d::PhysicsPlugins::default(),
        bevy_vector_shapes::Shape2dPlugin::default(),
    ));

    #[cfg(debug_assertions)]
    app.add_plugins((
        avian2d::debug_render::PhysicsDebugPlugin::default(),
        EguiPlugin {
            enable_multipass_for_primary_context: true,
        },
        WorldInspectorPlugin::new(),
    ));

    // Add other plugins.
    app.add_plugins((
        asset_tracking::plugin,
        common::plugin,
        screens::plugin,
        menus::plugin,
        ui::plugin,
        game::plugin,
    ));

    // Order new `AppSystems` variants by adding them here:
    app.configure_sets(
        Update,
        (
            AppSystems::TickTimers,
            AppSystems::RecordInput,
            PhysicsSet::StepSimulation,
            PhysicsSet::Sync,
            AppSystems::Update,
            AppSystems::UpdateCamera,
        )
            .chain(),
    );

    // Set up the `Pause` state.
    app.init_state::<Pause>();
    app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));

    // Spawn the main camera.
    app.add_systems(Startup, spawn_camera);
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
    /// Update the camera
    UpdateCamera,
}

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
struct Pause(pub bool);

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct PausableSystems;

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    let mut projection = OrthographicProjection::default_2d();

    projection.scaling_mode = camera::ScalingMode::FixedHorizontal {
        viewport_width: WORLD_SCALE * 512.0,
    };

    commands.spawn((
        Name::new("Camera"),
        Camera2d,
        MainCamera,
        Projection::Orthographic(projection),
    ));
}
