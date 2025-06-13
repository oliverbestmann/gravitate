use crate::game::cv;
use crate::game::cv::LAYER_PLAYER_INPUT;
use crate::game::input::{InputActive, InputTransformContext};
use crate::game::player::Player;
use crate::screens::Screen;
use crate::{AppSystems, MainCamera};
use bevy::math::FloatPow;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_vector_shapes::painter::ShapePainter;
use bevy_vector_shapes::shapes::LinePainter;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, visualize_thrust_input.in_set(AppSystems::Update));
    app.add_systems(OnEnter(Screen::Gameplay), spawn_burn_label);
}

#[derive(Component, Reflect)]
struct BurnTimeText;

fn spawn_burn_label(mut commands: Commands) {
    commands.spawn((
        StateScoped(Screen::Gameplay),
        BurnTimeText,
        Text2d::new("0123456789.0s"),
        TextFont::from_font_size(24.0),
        Anchor::Center,
        Visibility::Hidden,
    ));
}

fn visualize_thrust_input(
    player: Query<(&Transform, &InputActive), With<Player>>,
    input_transform: Single<InputTransformContext, With<MainCamera>>,

    mut burn_time_label: Single<
        (&mut Text2d, &mut Visibility, &mut Transform),
        (With<BurnTimeText>, Without<Player>),
    >,

    mut painter: ShapePainter,
) {
    let (burn_time_text, burn_time_visibility, burn_time_transform) = &mut *burn_time_label;

    // the input state from the player
    let Ok((player, input)) = player.single() else {
        burn_time_visibility.set_if_neq(Visibility::Hidden);
        return;
    };

    let Some(state) = input.state(&*input_transform) else {
        burn_time_visibility.set_if_neq(Visibility::Hidden);
        return;
    };

    // use the players position into the origin
    let origin = player.translation.xy().extend(LAYER_PLAYER_INPUT.0);

    painter.set_translation(origin);

    // the thrust vector
    let thrust_vec = state.end - state.start;

    // thickness of the line
    let thickness = 8.0;

    // simulate a shadow by drawing multiple transparent lines
    // of increasing width
    const STEPS: usize = 20;
    for idx in 1..=STEPS {
        let f = idx as f32 / STEPS as f32;

        painter.thickness = thickness + thickness * f.squared();
        painter.roundness = thickness + thickness * f.squared();
        painter.color = Color::BLACK.with_alpha(1.0 / STEPS as f32);
        painter.line(Vec3::ZERO, thrust_vec.extend(0.0));
    }

    painter.thickness = thickness;
    painter.roundness = thickness;
    painter.color = cv::COLOR_THRUST_INPUT_LINE;
    painter.line(Vec3::ZERO, thrust_vec.extend(0.0));

    let spacing = vec2(
        thrust_vec.normalize().x * 8.0 * thickness,
        thrust_vec.normalize().y * 4.0 * thickness,
    );

    burn_time_text.0 = format!("{:1.2}s", state.duration.as_secs_f32());
    burn_time_visibility.set_if_neq(Visibility::Visible);
    burn_time_transform.translation = origin + (thrust_vec + spacing).extend(0.0);
}
