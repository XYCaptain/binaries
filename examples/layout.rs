mod views;
use std::time::Duration;

use bevy::color::palettes::css::{BLUE, DIM_GRAY, GREEN, RED, YELLOW};
use bevy::color::palettes::tailwind::YELLOW_200;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitSettings;
use bevy_vector_shapes::prelude::ShapePainter;
use binaries_ui::components::{element, rectangle};
use binaries_ui::components::stacks::{hstack, vstack};
use binaries_ui::layout::UILayouts;
use binaries_ui::traits::UIElement;
use binaries_ui::UIPlugin;

fn main() {
    App::new()
        .insert_resource(WinitSettings {
            focused_mode: bevy::winit::UpdateMode::Continuous,
            unfocused_mode: bevy::winit::UpdateMode::reactive_low_power(Duration::from_secs(10)),
        })
        .insert_resource(ClearColor(DIM_GRAY.into()))
        .add_plugins((DefaultPlugins, UIPlugin))
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, (ui_setup, setup.after(ui_setup)))
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
    let contents =
    vstack(
        (
        rectangle().size(Vec2::new(100., 100.)).color(RED*0.5),
        rectangle().size(Vec2::new(100., 100.)).color(RED*0.5),
            hstack(
            (
                        rectangle().size(Vec2::new(100., 100.)).color(BLUE*0.5),
                        rectangle().size(Vec2::new(100., 100.)).color(BLUE*0.5),
                    )
            ).color(YELLOW_200),
        )
    )
    .title("vstack");
    hstack((
        rectangle().size(Vec2::new(100., 100.)).color(RED),
        rectangle().size(Vec2::new(100., 100.)).color(YELLOW),
        contents,
        rectangle().color(GREEN).element_type(element::ElementType::Debug)
    ))
    .add_to_layout(&mut layouts);

    layouts.print_tree();
}
