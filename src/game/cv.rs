use crate::game::layer::Layer;
use bevy::prelude::Color;

pub const COLOR_BACKGROUND: Color = srgb_from_u32(0x553683ff);
pub const COLOR_THRUST_INPUT_LINE: Color = srgb_from_u32(0xdfb2d9ff);

const fn srgb_from_u32(color: u32) -> Color {
    let r = ((color >> 24) & 0xff) as f32 / 255.0;
    let g = ((color >> 16) & 0xff) as f32 / 255.0;
    let b = ((color >> 8) & 0xff) as f32 / 255.0;
    let a = ((color) & 0xff) as f32 / 255.0;

    Color::srgba(r, g, b, a)
}

pub const LAYER_ROCKET: Layer = Layer(1.0);
pub const LAYER_OFFSET_ROCKET_FIN_BG: Layer = Layer(-0.6);
pub const LAYER_OFFSET_ROCKET_PLUME: Layer = Layer(-0.5);
pub const LAYER_OFFSET_ROCKET_FIN_FG: Layer = Layer(0.1);

pub const LAYER_STARS: Layer = Layer(-1.0);

pub const LAYER_PLAYER_INPUT: Layer = Layer(2.0);
