use std::sync::RwLockWriteGuard;

use bevy::math::Vec3;
use bevy_vector_shapes::prelude::ShapePainter;
use taffy::{Dimension, Size, Style};

use crate::{components::{element::{Element, ElementType}, UIMouseState, UIRenderMode}, context::MemState, layout::UILayouts};



pub trait UIElement: Sync + Send + 'static {
    fn draw(&self, painter: &mut ShapePainter);

    fn size(&self) -> (f32, f32);

    fn style(&self) -> Style {
        Style {
            size: Size { width: Dimension::Percent(0.5), height: Dimension::Auto },
            ..Default::default()
        }
    }

    fn get_ready(&self) -> bool {
        false
    }

    fn set_ready(&mut self);

    fn update_layout(&mut self, layout: &taffy::Layout, origin: Vec3, inherit_origin: Vec3, cxt:&mut RwLockWriteGuard<MemState>);

    fn get_action_state(&mut self)-> UIMouseState;

    fn set_action_state(&mut self, state: UIMouseState);

    fn get_render_state(&mut self)->  Option<UIMouseState>;

    fn set_render_state(&mut self,state: UIMouseState);

    fn update_render_state(&mut self, cursor: (f32, f32), origin:Vec3);

    fn block_render_state(&mut self)-> UIRenderMode;

    fn execute(&mut self, ctx:&mut RwLockWriteGuard<MemState>);

    fn get_z_order(&self) -> i32 {0}

    fn set_z_order(&mut self,z_order:i32) -> i32;

    fn children(&self) -> Option<Vec<Box<dyn UIElement>>>;

    fn add_to_layout(&self, layout: &mut UILayouts) {
        let _ = layout;
        todo!()
    }

    fn get_element_type(&self) -> ElementType;

    fn get_element(&self) -> Element;
}

pub trait UILayout {
    fn push(&mut self, element: Box<dyn UIElement>);
}