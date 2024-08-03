use bevy::math::Vec3;
use bevy_vector_shapes::prelude::ShapePainter;
use taffy::{Dimension, Layout, Size, Style};

use crate::{components::UIMouse, layout::Context};

pub trait UIElement: Sync + Send {
    fn draw(&self, painter: &mut ShapePainter);

    fn size(&self) -> (f32, f32);

    fn style(&self) -> Style {
        Style {
            size: Size { width: Dimension::Percent(0.5), height: Dimension::Auto },
            ..Default::default()
        }
    }

    fn is_ready(&self) -> bool {
        false
    }

    fn set_ready(&mut self);

    fn update(&mut self, cursor: (f32, f32),painter: &mut ShapePainter, layout: &Layout,org:Vec3);

    fn update_input_state(&mut self, state: UIMouse);
    
    fn exc(&mut self, ctx:&mut Context);

    fn z_order(&self) -> i32 {0}

    fn set_z_order(&mut self,z_order:i32) -> i32;

    fn get_children(&self) -> Option<Vec<Box<dyn UIElement>>>;

}

pub trait UILayout {
    fn push(&mut self, element: Box<dyn UIElement>);
}