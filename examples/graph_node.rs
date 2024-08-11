mod views;
use std::time::Duration;

use bevy::color::palettes::css::DIM_GRAY;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitSettings;
use bevy_vector_shapes::prelude::ShapePainter;
use bevy_vector_shapes::Shape2dPlugin;
use binaries_ui::input::print_mouse_events_system;
use binaries_ui::layout::{Context, SDUILayouts};
use binaries_ui::traits::UIElement;

fn main() {
    App::new()
        .insert_resource(WinitSettings {
            focused_mode: bevy::winit::UpdateMode::Continuous,
            unfocused_mode: bevy::winit::UpdateMode::reactive_low_power(Duration::from_secs(10)),
        })
        .insert_resource(ClearColor(DIM_GRAY.into()))
        .add_plugins(DefaultPlugins)
        .insert_resource(Context::default())
        .insert_resource(SDUILayouts::new())
        .add_plugins(Shape2dPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(
            Startup,
            (
                layout_setup.before(ui_setup),
                ui_setup,
                setup.after(ui_setup),
            ),
        )
        .add_systems(Update, print_mouse_events_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 0,
            ..default()
        },
        ..Default::default()
    });

    commands.spawn(Camera3dBundle {
        camera: Camera {
            order: 1,
            ..default()
        },
        transform: Transform::from_xyz(-20.0, 20., 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn layout_setup(
    mut painter: ShapePainter,
    mut layouts: ResMut<SDUILayouts>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    painter.origin = Some(Vec3::new(-window.width() * 0.5, window.height() * 0.5, 0.));
    layouts.init(&mut painter);
}


fn ui_setup(mut layouts: ResMut<SDUILayouts>) {
    views::graph_node_test::node_test_view().add_to_layout(&mut layouts);
}