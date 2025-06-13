use crate::{AppSystems, MainCamera, PausableSystems};
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::time::Duration;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<OnThurst>();

    app.add_systems(
        Update,
        (update_input_state, mouse_input_start)
            .chain()
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );

    app.add_observer(input_deactivated);
    app.add_observer(slow_time_on_input);
    app.add_observer(reset_time_after_input);
}

#[derive(Component)]
pub struct Input;

#[derive(Component)]
pub struct InputActive {
    start: Vec2,
    end: Vec2,
    touch_id: Option<u64>,
}

impl InputActive {
    pub fn state(&self, context: &InputTransformContext) -> Option<InputState> {
        let (camera, camera_transform) = context;

        let start = camera
            .viewport_to_world_2d(camera_transform, self.start)
            .unwrap();

        let end = camera
            .viewport_to_world_2d(camera_transform, self.end)
            .unwrap();

        let length = start.distance(end);
        if length < 16.0 {
            // we discard this as a failure / no actual input
            return None;
        }

        let duration = Duration::from_secs_f32(length / 100.0);
        Some(InputState {
            duration,
            start,
            end,
        })
    }
}

pub struct InputState {
    pub duration: Duration,
    pub start: Vec2,
    pub end: Vec2,
}

pub type InputTransformContext<'w> = (&'w Camera, &'w GlobalTransform);

#[derive(Event, Debug)]
pub struct OnThurst {
    pub direction: Vec2,
    pub duration: Duration,
}

fn mouse_input_start(
    mut commands: Commands,
    mut events: EventReader<MouseButtonInput>,
    target_query: Query<(Entity, Has<InputActive>), With<Input>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    for event in events.read() {
        let Some(cursor) = windows
            .get(event.window)
            .ok()
            .and_then(Window::cursor_position)
        else {
            warn!("No window or cursor for event");
            return;
        };

        match event.state {
            ButtonState::Pressed => {
                for (entity, active) in target_query {
                    if active {
                        // already active on this component
                        continue;
                    }

                    commands.entity(entity).insert(InputActive {
                        start: cursor,
                        end: cursor,
                        touch_id: None,
                    });
                }
            }

            ButtonState::Released => {
                // remove the input component from the target if it exists
                for (entity, active) in target_query {
                    if active {
                        commands.entity(entity).remove::<InputActive>();
                    }
                }
            }
        }
    }
}

fn update_input_state(
    inputs: Query<&mut InputActive>,
    window: Single<&Window, With<PrimaryWindow>>,
    touches: Res<Touches>,
) {
    for mut input in inputs {
        match input.touch_id {
            None => {
                let Some(cursor) = window.cursor_position() else {
                    warn!("No cursor position in window");
                    continue;
                };

                input.end = cursor;
            }

            Some(touch_id) => {
                let Some(touch) = touches.get_pressed(touch_id) else {
                    warn!("Touch does not exist");
                    continue;
                };

                input.end = touch.position();
            }
        }
    }
}

fn slow_time_on_input(_: Trigger<OnAdd, InputActive>, mut time: ResMut<Time<Virtual>>) {
    time.set_relative_speed(0.05);
}

fn input_deactivated(
    trigger: Trigger<OnRemove, InputActive>,
    mut commands: Commands,
    inputs: Query<&InputActive>,
    camera: Single<InputTransformContext, With<MainCamera>>,
) {
    let Ok(input) = inputs.get(trigger.target()) else {
        return;
    };

    let Some(state) = input.state(&*camera) else {
        return;
    };

    let direction = (state.end - state.start).normalize();

    let thrust = OnThurst {
        duration: state.duration,
        direction,
    };

    info!("Trigger thrust event: {:?}", thrust);
    commands.entity(trigger.target()).trigger(thrust);
}

fn reset_time_after_input(_: Trigger<OnRemove, InputActive>, mut time: ResMut<Time<Virtual>>) {
    time.set_relative_speed(1.0);
}
