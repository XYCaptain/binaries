pub mod input;
pub mod traits;
pub mod components;
pub mod utils;
pub mod layout;
pub mod shape;
pub mod text;

use bevy::{app::{App, Plugin, Startup, Update}, prelude::{default, Camera, Camera2dBundle, Commands}};
use bevy_vector_shapes::ShapePlugin;
use input::print_mouse_events_system;
use layout::{Context, UILayouts};
use text::DefaultFont;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(DefaultFont::default())
        .insert_resource(Context::default())
        .insert_resource(UILayouts::new())
        .add_plugins(ShapePlugin::default())
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