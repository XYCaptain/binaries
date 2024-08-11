use std::time::Duration;

use bevy::color::palettes::css::{BLUE, BROWN, DIM_GRAY, GREEN, RED};
use bevy::color::palettes::tailwind::{YELLOW_100, YELLOW_400};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitSettings;
use bevy_vector_shapes::prelude::ShapePainter;
use bevy_vector_shapes::Shape2dPlugin;
use binaries_ui::components::{circle, ngon, rectangle};
use binaries_ui::components::stacks::{hstack, vstack};
use binaries_ui::input::print_mouse_events_system;
use binaries_ui::layout::{Context, SDUILayouts};

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
    let stk_second = 
    hstack(
        (
            vstack(
                (
                    rectangle()
                        .size(Vec2::new(100., 100.))
                        .color(BLUE).margin(Vec4::new(10., 20., 30., 40.)).title("button1"),
                    rectangle()
                        .size(Vec2::new(100., 100.))
                        .color(RED).margin(Vec4::new(20., 20., 30., 40.)).title("button2"),
                )
            )
            .size(Vec2::new(300., 500.))
            .color(Srgba::new(1.0, 0.0, 1.0, 0.8))
            .title("stack0"),
            vstack(
                (
                    rectangle()
                        .size(Vec2::new(100., 100.))
                        .color(GREEN).margin(Vec4::new(10., 20., 30., 40.)).title("button3"),
                        rectangle()
                        .size(Vec2::new(100., 100.))
                        .color(BROWN).margin(Vec4::new(20., 20., 30., 40.)).title("button4"),
                )
            )
            .size(Vec2::new(300., 300.))
            .color(Srgba::new(1.0, 1.0, 0.0, 0.8))
            .title("stack2"),
        )
    )
    .title("stack3")
    .size(Vec2::new(1500., 500.))
    .color(Srgba::new(0.0, 1.0, 1.0, 1.0));


    let stk_first = 
    hstack(
        (
            hstack(
                (
                    circle()
                        .size(Vec2::new(100., 100.))
                        .color(BLUE).margin(Vec4::new(10., 20., 30., 40.)).title("button1"),
                    ngon(5.5)
                        .size(Vec2::new(100., 100.))
                        .color(RED).margin(Vec4::new(20., 20., 30., 40.)).title("button2"),
                ),
            )
            .round(50.)
            .size(Vec2::new(300., 200.))
            .color(Srgba::new(1.0, 0.0, 1.0, 0.8))
            .title("stack0"),
            hstack(
                (
                    rectangle()
                        .size(Vec2::new(100., 100.))
                        .color(GREEN).margin(Vec4::new(10., 20., 30., 40.)).title("button3"),
                    rectangle()
                        .size(Vec2::new(100., 100.))
                        .color(BROWN).margin(Vec4::new(20., 20., 30., 40.)).title("button4"),
                )
            )
            .round(100.)
            .size(Vec2::new(300., 300.))
            .color(Srgba::new(1.0, 1.0, 0.0, 0.8))
            .title("stack2"),
        )
    )
    .title("stack3")
    .size(Vec2::new(1500., 400.))
    .color(Srgba::new(0.0, 1.0, 1.0, 1.0));

   vstack((
        stk_first.clone().color(YELLOW_100).round(30.),
        stk_second.clone().color(YELLOW_400).round(20.),
        (
            ||circle().size(Vec2::new(100., 100.)).title("1").color(YELLOW_400))(),
        ),
    )
    .size(Vec2::new(3000., 2000.))
    .round(40.)
    .color(Srgba::new(1., 0.3, 0.3, 1.0))
    .title("stack4")
    .push_to_layout(&mut layouts);
}
