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

    pub plume: [Handle<Image>; 3],
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

            plume: [
                server.load("images/plume1.png"),
                server.load("images/plume2.png"),
                server.load("images/plume3.png"),
            ],
        }
    }
}
