use crate::asset_tracking::LoadResource;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.load_resource::<Assets>();
}

#[derive(Clone, Resource, Asset, TypePath)]
pub struct Assets {
    pub player: Handle<Image>,
}

impl FromWorld for Assets {
    fn from_world(world: &mut World) -> Self {
        let server = world.resource_mut::<AssetServer>();

        Self {
            player: server.load("images/star.png"),
        }
    }
}
