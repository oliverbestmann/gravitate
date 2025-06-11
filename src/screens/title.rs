//! The title screen that appears after the splash screen.

use bevy::prelude::*;

use crate::ui::widget;
use crate::{menus::Menu, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Assets>();

    app.add_systems(OnEnter(Screen::Title), open_main_menu);
    app.add_systems(OnEnter(Screen::Title), spawn_background_image);
    app.add_systems(OnExit(Screen::Title), close_menu);
}

fn open_main_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn spawn_background_image(mut commands: Commands, assets: Res<Assets>) {
    commands.spawn((
        StateScoped(Screen::Title),
        widget::ui_root("Background"),
        GlobalZIndex(1),
        children![(
            Node {
                margin: UiRect::all(Val::Auto),
                width: Val::Percent(100.0),
                ..default()
            },
            ImageNode::new(assets.background.clone()),
        )],
    ));
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

#[derive(Clone, Resource, Asset, TypePath)]
struct Assets {
    background: Handle<Image>,
}

impl FromWorld for Assets {
    fn from_world(world: &mut World) -> Self {
        let server = world.resource_mut::<AssetServer>();

        Self {
            background: server.load("images/loader.png"),
        }
    }
}
