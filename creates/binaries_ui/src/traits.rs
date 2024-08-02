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

    fn isready(&self) -> bool {
        false
    }

    fn setready(&mut self);

    fn update(&mut self, cursor: (f32, f32),painter: &mut ShapePainter, layout: &Layout);

    fn update_input_state(&mut self, state: UIMouse);
    
    fn exc(&mut self, ctx:&mut Context);
}

pub trait UILayout {
    fn push(&mut self, element: Box<dyn UIElement>);
}