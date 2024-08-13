pub mod input;
pub mod traits;
pub mod components;
pub mod utils;
pub mod layout;
pub mod shape;

use bevy::{app::{App, Plugin, Startup, Update}, math::Vec3, prelude::{default, Camera, Camera2dBundle, Commands, Query, ResMut, With}, window::{PrimaryWindow, Window}};
use bevy_vector_shapes::{prelude::ShapePainter, Shape2dPlugin};
use input::print_mouse_events_system;
use layout::{Context, UILayouts};
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(Context::default())
        .insert_resource(UILayouts::new())
        .add_plugins(Shape2dPlugin::default())
        .add_systems(
            Startup,
            (
                camera_setup,
            ),
        )
        .add_systems(Update, print_mouse_events_system);
    }
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 0,
            ..default()
        },
        ..Default::default()
    });
}