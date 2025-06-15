use crate::game::cv;
use crate::game::input::{InputActive, InputTransformContext};
use crate::game::player::Player;
use crate::game::rocket::FuelTank;
use crate::screens::Screen;
use crate::{AppSystems, MainCamera, game};
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, visualize_thrust_input.in_set(AppSystems::Update));
    app.add_systems(
        OnEnter(Screen::Gameplay),
        (spawn_burn_label, spawn_indicator_line),
    );
}

#[derive(Component, Reflect)]
struct BurnTimeText;

#[derive(Component, Reflect)]
struct ThrustVecLine;

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

fn spawn_indicator_line(mut commands: Commands, assets: Res<game::Assets>) {
    commands.spawn((
        StateScoped(Screen::Gameplay),
        ThrustVecLine,
        Visibility::Hidden,
        Sprite {
            image: assets.line.clone(),
            anchor: Anchor::CenterLeft,
            color: cv::COLOR_THRUST_INPUT_LINE,
            custom_size: Some(vec2(128.0, 8.0)),
            image_mode: SpriteImageMode::Sliced(TextureSlicer {
                border: BorderRect::axes(32.0, 0.0),
                ..default()
            }),
            ..default()
        },
    ));
}

fn visualize_thrust_input(
    player: Query<(&Transform, &InputActive, &FuelTank), With<Player>>,
    input_transform: Single<InputTransformContext, With<MainCamera>>,

    burn_time_label: Single<
        (&mut Text2d, &mut TextColor, &mut Visibility, &mut Transform),
        (With<BurnTimeText>, Without<ThrustVecLine>, Without<Player>),
    >,

    burn_line: Single<
        (&mut Sprite, &mut Transform, &mut Visibility),
        (With<ThrustVecLine>, Without<BurnTimeText>, Without<Player>),
    >,
) {
    let (
        mut burn_time_text,
        mut burn_time_color,
        mut burn_time_visibility,
        mut burn_time_transform,
    ) = burn_time_label.into_inner();

    let (mut line_sprite, mut line_transform, mut line_visibility) = burn_line.into_inner();

    // the input state from the player
    let Ok((player, input, fuel)) = player.single() else {
        line_visibility.set_if_neq(Visibility::Hidden);
        burn_time_visibility.set_if_neq(Visibility::Hidden);
        return;
    };

    let Some(state) = input.state(&input_transform) else {
        line_visibility.set_if_neq(Visibility::Hidden);
        burn_time_visibility.set_if_neq(Visibility::Hidden);
        return;
    };

    let line_shadow_extents = vec2(16.0, 8.0);

    // use the players position into the origin
    let origin = player.translation.xy().extend(cv::LAYER_PLAYER_INPUT.0);

    // the thrust vector
    let thrust_vec = state.end - state.start;

    // thickness of the line
    let thickness = 8.0;

    // // simulate a shadow by drawing multiple transparent lines
    // // of increasing width
    // const STEPS: usize = 20;
    // for idx in 1..=STEPS {
    //     let f = idx as f32 / STEPS as f32;
    //
    //     painter.thickness = thickness + thickness * f.squared();
    //     painter.roundness = thickness + thickness * f.squared();
    //     painter.color = Color::BLACK.with_alpha(1.0 / STEPS as f32);
    //     painter.line(Vec3::ZERO, thrust_vec.extend(0.0));
    // }

    line_transform.translation = origin;
    line_transform.rotation = Quat::from_rotation_z(thrust_vec.to_angle());
    line_sprite.custom_size = Some(vec2(thrust_vec.length(), thickness) + line_shadow_extents);
    line_visibility.set_if_neq(Visibility::Visible);

    let spacing = vec2(
        thrust_vec.normalize().x * 8.0 * thickness,
        thrust_vec.normalize().y * 4.0 * thickness,
    );

    let burn_time = state.duration.as_secs_f32().min(fuel.remaining.as_secs());
    let limited = state.duration.as_secs_f32() >= fuel.remaining.as_secs();

    let color = if limited {
        Color::srgba(1.0, 0.5, 0.5, 1.)
    } else {
        Color::WHITE
    };

    burn_time_text.0 = format!("{:1.2}s", burn_time);
    burn_time_color.set_if_neq(TextColor(color));
    burn_time_visibility.set_if_neq(Visibility::Visible);
    burn_time_transform.translation = origin + (thrust_vec + spacing).extend(0.0);
}
