use std::clone;
use std::sync::RwLockWriteGuard;

use crate::context::MemState;
use crate::layout::UILayouts;
use crate::shape::ShapeTrait;
use bevy::color::Srgba;
use bevy::math::{Vec2, Vec3, Vec4};

use super::element::{ AlignItems, Element};
use super::element_set::ElementSet;
use super::{UIMouseState, UIRenderMode};
use crate::traits::UIElement;
use bevy_vector_shapes::prelude::ShapePainter;
use taffy::Style;
use crate::components::element::FlexDirection;

pub fn vstack<K>(children: K) -> Stack<K>
where
    K: ElementSet,
{
    stack(children).direction(FlexDirection::Column).title("vstack")
}

pub fn hstack<K>(children: K) -> Stack<K>
where
    K: ElementSet,
{
    stack(children).direction(FlexDirection::Row).title("hstack")
}

pub fn stack<K>(children: K) -> Stack<K>
where
    K: ElementSet,
{
    Stack::new(children).render_mode(UIRenderMode::WithoutSelf)
}



#[derive(Clone)]
pub struct Stack<K>
where
    K: ElementSet,
{
    children: K,
    element: Element,
}

impl<K> Stack<K>
where
    K: ElementSet,
{
    pub fn new(children: K) -> Self {
        Self 
        {
            children,
            element: Element::new(),
        }
    }

    pub fn size(mut self,size:Vec2) -> Self {
        self.element =  self.element.size(size);
        self
    }

    pub fn color(mut self,color:Srgba) -> Self {
        self.element =  self.element.color(color);
        self
    }

    pub fn background_color(mut self,color:Srgba) -> Self {
        self.element =  self.element.background_color(color);
        self
    }

    pub fn title(mut self,title:&str) -> Self {
        self.element =  self.element.title(title);
        self
    }

    pub fn click(mut self, action: impl Fn(&mut Element,&mut RwLockWriteGuard<MemState>) + Send + Sync + 'static) -> Self {
        self.element =  self.element.click(action);
        self
    }

    pub fn hover(mut self, action: impl Fn(&mut Element,&mut RwLockWriteGuard<MemState>) + Send + Sync + 'static) -> Self {
        self.element =  self.element.hover(action);
        self
    }

    pub fn direction(mut self, direction:FlexDirection) -> Self {
        self.element = self.element.direction(direction);
        self
    }

    pub fn padding(mut self, padding: Vec4) -> Self {
        self.element = self.element.padding(padding);
        self
    }

    pub fn offset(mut self, offset: Vec3) -> Self {
        self.element = self.element.offset(offset);
        self
    }
    
    pub fn round(mut self, round:f32) -> Self {
        self.element = self.element.round(round);
        self
    }

    pub fn shape(mut self, shape: impl ShapeTrait) -> Self {
        self.element = self.element.shape(shape);
        self
    }

    pub fn margin(mut self, margin:Vec4) -> Self {
        self.element = self.element.margin(margin);
        self
    }

    pub fn render_mode(mut self, is_blcok:UIRenderMode) -> Self {
        self.element = self.element.render_block(is_blcok);
        self
    }

    pub fn push_to_layout(&self, layout: &mut UILayouts) {
        let node_id = layout.push_element(self.element.clone());
        let mut children = Vec::new();
        self.children.foreach_view(&mut |element| {
            children.push((node_id,element));
        });

        while !children.is_empty() {
            let mut new_children = Vec::new();
            let mut new_pairs = Vec::new();
            for  (p_id,child) in children {
                if child.get_children().is_some() {
                    for grand_child in child.get_children().unwrap() {
                        new_children.push(grand_child);
                    }
                }
                let id = layout.push_element_with_id(child.get_element(), p_id);
                for grand_child in new_children.drain(..) {
                    new_pairs.push((id,grand_child));
                }
            }
            children = new_pairs;
        }
    }

    pub fn horizontal_alignment(mut self,align:AlignItems)->Self{
        match self.element.direction{
            FlexDirection::Row => {
                self.element.main_axis_alignment = align;
            },
            FlexDirection::Column => {
                self.element.cors_axis_alignment = align;
            }
            FlexDirection::RowReverse => {
                self.element.main_axis_alignment = align;
            },
            FlexDirection::ColumnReverse => {
                self.element.cors_axis_alignment = align;
            },
        }
        self
    }
    
    pub fn vertical_alignment(mut self,align:AlignItems)->Self{
        // self.vertical_alignment = align;
        match self.element.direction{
            FlexDirection::Row => {
                self.element.cors_axis_alignment = align;
            },
            FlexDirection::Column => {
                self.element.main_axis_alignment = align;
            }
            FlexDirection::RowReverse => {
                self.element.cors_axis_alignment = align;
            },
            FlexDirection::ColumnReverse => {
                self.element.main_axis_alignment = align;
            },
        }
        self
    }
}

impl<K> UIElement for Stack<K>
where
    K: ElementSet + Send + Sync + 'static
{
    fn style(&self) -> Style {
        self.element.style()
    }

    fn draw(&self, painter: &mut ShapePainter) {
        self.element.draw(painter);
    }

    fn size(&self) -> (f32, f32) {
        let size = self.element.get_size();
        (size.x, size.y)
    }

    fn is_ready(&self) -> bool {
        self.element.is_ready()
    }

    fn set_ready(&mut self) {
        self.element.set_ready();
    }

    fn update_layout(&mut self, layout: &taffy::Layout, origin: Vec3, inherit_origin: Vec3) {
        self.element.update_layout(layout, origin, inherit_origin);
    }

    fn update_render_state(&mut self, cursor: (f32, f32), origin:Vec3) {
        self.element.update_render_state(cursor, origin);
    }

    fn exc(&mut self, context: &mut RwLockWriteGuard<MemState>) {
        self.element.exc(context);
    }

    fn get_z_order(&self) -> i32 {
        self.element.get_z_order()
    }

    fn set_z_order(&mut self, z_order: i32) -> i32{
        self.element.set_z_order(z_order);
        self.get_z_order()
    }
    
    fn get_children(&self) -> Option<Vec<Box<dyn UIElement>>> {
        let mut children = Vec::new();
        self.children.foreach_view(&mut |child| {
            children.push(child);
        });
        children.into()
    }

    fn get_input_state(&mut self)-> UIMouseState {
        self.element.get_input_state()
    }

    fn set_action_state(&mut self, state: UIMouseState) {
        self.element.set_action_state(state);
    }
    
    fn get_render_state(&mut self)-> Option<UIMouseState> {
        self.element.get_render_state()
    }
    
    fn set_render_state(&mut self,state: UIMouseState ) {
        self.element.set_render_state(state);
    }

    fn block_render_state(&mut self)->UIRenderMode {
        self.element.block_render_state()
    }

    fn add_to_layout(&self, layout: &mut UILayouts) {
        self.push_to_layout(layout);
    }

    fn get_element_type(&self) -> super::element::ElementType {
        self.element.get_element_type()
    }

    fn get_element(&self) -> Element {
        self.element.clone()
    }
}



#[cfg(test)]
mod tests {

    use crate::components::element;

    use super::*;

    #[test]
    fn test_button() {

    }

    #[test]
    fn test_stack() {
        stack(
            (
                element(),
                element()
            )
        );
    }

    #[test]
    fn test_element_tuple() {
        (
            stack(
                (
                    element(),
                    element()
                )
            ),
            stack(
                (
                    element(),
                    element()
                )
            )
        ).foreach_view(&mut |child| {
              println!("{:?}", child.style());
        });
    }
}