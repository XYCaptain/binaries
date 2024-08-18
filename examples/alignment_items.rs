mod views;
use std::time::Duration;

use bevy::color::palettes::css::DIM_GRAY;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::winit::WinitSettings;
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
        .add_plugins((DefaultPlugins,UIPlugin))
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup,ui_setup)
        .run();
}

fn ui_setup(mut layouts: ResMut<UILayouts>) {
    views::layout_gallery::node_panel().add_to_layout(&mut layouts);
    layouts.print_tree();
}