use crate::asset_tracking::LoadResource;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.load_resource::<Assets>();
}

#[derive(Clone, Resource, Asset, TypePath)]
pub struct Assets {
    pub rocket_base: Handle<Image>,
    pub rocket_fin_bg: Handle<Image>,
    pub rocket_fin_fg: Handle<Image>,

    pub star_small: Handle<Image>,
    pub star_large: Handle<Image>,
    pub line: Handle<Image>,
    pub plume: [Handle<Image>; 3],

    pub planets: Vec<PlanetAssets>,
}

#[derive(Clone)]
pub struct PlanetAssets {
    pub images: Vec<Handle<Image>>,
}

impl PlanetAssets {
    pub fn new(images: impl Into<Vec<Handle<Image>>>) -> Self {
        Self {
            images: images.into(),
        }
    }
}

impl FromWorld for Assets {
    fn from_world(world: &mut World) -> Self {
        let server = world.resource_mut::<AssetServer>();

        Self {
            rocket_base: server.load("images/rocket-base.png"),
            rocket_fin_bg: server.load("images/rocket-fin-bg.png"),
            rocket_fin_fg: server.load("images/rocket-fin-fg.png"),

            star_small: server.load("images/star-small.png"),
            star_large: server.load("images/star-large.png"),

            line: server.load("images/line.png"),

            plume: [
                server.load("images/plume1.png"),
                server.load("images/plume2.png"),
                server.load("images/plume3.png"),
            ],

            planets: vec![
                PlanetAssets::new([
                    server.load("images/planet-earth-1.png"),
                    server.load("images/planet-earth-2.png"),
                    server.load("images/planet-earth-3.png"),
                    server.load("images/planet-earth-4.png"),
                ]),
                PlanetAssets::new([
                    server.load("images/planet-2-1.png"),
                    server.load("images/planet-2-2.png"),
                    server.load("images/planet-2-3.png"),
                    server.load("images/planet-2-4.png"),
                    server.load("images/planet-2-5.png"),
                    server.load("images/planet-2-6.png"),
                ]),
            ],
        }
    }
}
