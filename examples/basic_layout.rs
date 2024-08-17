use std::time::Duration;

use bevy::color::palettes::css::{BLUE, BROWN, DIM_GRAY, GREEN, RED};
use bevy::color::palettes::tailwind::{YELLOW_100, YELLOW_400};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::winit::WinitSettings;
use binaries_ui::components::element::ElementType;
use binaries_ui::components::{circle, ngon, rectangle};
use binaries_ui::components::stacks::{hstack, vstack};
use binaries_ui::layout::UILayouts;
use binaries_ui::UIPlugin;

fn main() {
    App::new()
        .insert_resource(WinitSettings {
            focused_mode: bevy::winit::UpdateMode::Continuous,
            unfocused_mode: bevy::winit::UpdateMode::reactive_low_power(Duration::from_secs(10)),
        })
        .insert_resource(ClearColor(DIM_GRAY.into()))
        .add_plugins((DefaultPlugins,UIPlugin))
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(
            Startup,
            (
                ui_setup,
                setup.after(ui_setup),
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        camera: Camera {
            order: 1,
            ..default()
        },
        transform: Transform::from_xyz(-20.0, 20., 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}


fn ui_setup(mut layouts: ResMut<UILayouts>) {
    let stk_second = 
    hstack(
        (
            vstack(
                (
                    rectangle()
                        .size(Vec2::new(100., 100.))
                        .color(BLUE).margin(Vec4::new(10., 20., 30., 40.)).title("button1"),
                    rectangle()
                        .color(RED).margin(Vec4::new(20., 20., 30., 40.)).title("button2"),
                )
            )
            .color(Srgba::new(1.0, 0.0, 1.0, 0.8)),
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
            .color(Srgba::new(1.0, 1.0, 0.0, 0.8)),
        )
    )
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
            .color(Srgba::new(1.0, 0.0, 1.0, 0.8)),
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
            .color(Srgba::new(1.0, 1.0, 0.0, 0.8))
        )
    )
    .color(Srgba::new(0.0, 1.0, 1.0, 1.0));

   hstack((
        vstack(
            (
                stk_first.clone().color(YELLOW_100).round(30.),
                stk_second.clone().color(YELLOW_400).round(20.),
                (||circle().size(Vec2::new(100., 100.)).title("circle").color(YELLOW_400))(),
            )
        ),
        rectangle().color(GREEN).element_type(ElementType::Debug)
        ),
    )
    .round(40.)
    .push_to_layout(&mut layouts);
}
