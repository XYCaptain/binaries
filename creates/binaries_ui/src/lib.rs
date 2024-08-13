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
                layout_setup,
                camera_setup,
            ),
        )
        .add_systems(Update, print_mouse_events_system);
    }
}

fn layout_setup(
    mut painter: ShapePainter,
    mut layouts: ResMut<UILayouts>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    painter.origin = Some(Vec3::new(-window.width() * 0.5, window.height() * 0.5, 0.));
    layouts.init(&mut painter);
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