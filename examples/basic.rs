use std::time::Duration;

use bevy::color::palettes::css::{
    BLUE, DIM_GRAY, RED, SEA_GREEN,
};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitSettings;
use bevy_vector_shapes::prelude::ShapePainter;
use bevy_vector_shapes::Shape2dPlugin;
use binaries_ui::layout::SDUILayouts;
use binaries_ui::components::{button, stack::*};

fn main() {
    App::new()
        .insert_resource(WinitSettings {
            focused_mode: bevy::winit::UpdateMode::Continuous,
            unfocused_mode: bevy::winit::UpdateMode::reactive_low_power(Duration::from_secs(10)),
        })
        .insert_resource(ClearColor(DIM_GRAY.into()))
        .add_plugins(DefaultPlugins)
        .insert_resource(SDUILayouts::new())
        .add_plugins(Shape2dPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, (layout_setup.before(ui_setup),ui_setup,setup.after(ui_setup)))
        .add_systems(Update, draw_gallery)
        .run();
}

fn setup(
    mut commands: Commands,
) {
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

fn draw_gallery(
    mut painter: ShapePainter,
    mut layouts: ResMut<SDUILayouts>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    painter.origin = Some(Vec3::new(-window.width() * 0.5, window.height() * 0.5, 0.));

    for event in cursor_moved_events.read() {
        layouts.update((event.position.x, event.position.y), &mut painter);
    }
    layouts.draw(&mut painter);
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

fn ui_setup(
    mut layouts: ResMut<SDUILayouts>,
) {
    stack((
        button().color(RED).size(Vec2::new(20., 100.)),
        button().color(SEA_GREEN).size(Vec2::new(30., 100.)),
        button().color(BLUE).size(Vec2::new(50., 100.)),
        button().color(BLUE).size(Vec2::new(50., 100.))
    )).push_to_layout(&mut layouts);
}

