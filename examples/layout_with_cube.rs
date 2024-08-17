mod views;
use std::time::Duration;

use bevy::color::palettes::css::{BLUE, DIM_GRAY, GREEN, RED, YELLOW};
use bevy::color::palettes::tailwind::YELLOW_200;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::winit::WinitSettings;
use binaries_ui::components::element::ElementType;
use binaries_ui::components::{rectangle, text};
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
        .add_systems(Update,rotate)
        .run();
}

#[derive(Component)]
struct CubeExample;

fn setup(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>) {

    commands.spawn(Camera3dBundle {
        camera: Camera {
            order: 1,
            ..default()
        },
        transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // cube
    let mesh = Mesh::from(Cuboid::default());
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(mesh),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            material: materials.add(StandardMaterial{
                base_color : bevy::color::Color::WHITE.with_alpha(0.5),
                ..Default::default()
            }),
            ..default()
        },
        CubeExample
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(5.0, 5.0, 10.0),
        ..default()
    }); 
}

fn rotate(mut query: Query<&mut Transform, With<CubeExample>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.);
        transform.rotate_z(time.delta_seconds() / 2.);
    }
}

fn ui_setup(mut layouts: ResMut<UILayouts>,) {
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
        rectangle().color(GREEN).element_type(ElementType::Debug),
        hstack(
            text("hello, world!").size(Vec2::new(100., 20.))
        ).size(Vec2::new(100., 100.)).round(5.).color(BLUE)
    )).title("ui_layouts")
    .add_to_layout(&mut layouts);
}
