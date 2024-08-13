mod views;
use std::time::Duration;

use bevy::color::palettes::css::DIM_GRAY;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::winit::WinitSettings;
use binaries_ui::layout::SDUILayouts;
use binaries_ui::traits::UIElement;
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

fn ui_setup(mut layouts: ResMut<SDUILayouts>) {
    views::node_test::node_test_view().add_to_layout(&mut layouts);
}