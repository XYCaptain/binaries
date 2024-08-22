pub mod input;
pub mod traits;
pub mod components;
pub mod utils;
pub mod layout;
pub mod shape;
pub mod text;
mod storage;
mod context;

use bevy::{app::{App, Plugin, Startup, Update}, asset::Assets, ecs::system::SystemParam, prelude::{default, Camera, Camera2dBundle, Commands, Mesh, Query, Res, ResMut, With}, sprite::ColorMaterial, time::Time, window::{PrimaryWindow, Window}};
use bevy_vector_shapes::ShapePlugin;
use context::Context;
use input::logic_loop_system;
use layout::UILayouts;
use storage::{Node, StoragePlugin};
use text::DefaultFont;

#[derive(SystemParam)]
pub struct Config<'w,'s> {
   pub time: Res<'w,Time>,
   pub default_font: ResMut<'w,DefaultFont>,
   pub meshes: ResMut<'w,Assets<Mesh>>,
   pub materials: ResMut<'w,Assets<ColorMaterial>>,
   pub window: Query<'w,'s,&'static Window, With<PrimaryWindow>>,
   pub context: ResMut<'w,Context>,
}
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(DefaultFont::default())
        .insert_resource(Context::default())
        .insert_resource(UILayouts::new())
        .add_plugins((ShapePlugin::default(),StoragePlugin))
        .add_systems(
            Startup,
            (
                camera_setup,
            ),
        )
        .add_systems(Update, logic_loop_system);
    }
}

fn camera_setup(mut commands: Commands,  context: ResMut<Context>) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 0,
            ..default()
        },
        ..Default::default()
    }); 
    //增加node
    context.storage().write().unwrap().node.insert(123, Node{
        id:123,
        name: "123".to_string(),
        offset_x: 100,
        offset_y: -200,
    });
}