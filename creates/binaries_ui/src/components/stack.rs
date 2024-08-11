use crate::layout::SDUILayouts;
use crate::shape::ShapeTrait;
use bevy::color::Srgba;
use bevy::math::{Vec2, Vec3, Vec4};
use bevy::utils::all_tuples;

use super::element::Element;
use super::{UIMouseState, UIRenderMode};
use crate::{layout::Context, traits::UIElement,shape::Rectangle};
use bevy_vector_shapes::prelude::ShapePainter;
use taffy::Style;
use crate::components::element::FlexDirection;

pub fn vstack<K>(children: K) -> Stack<K>
where
    K: ElementTuple,
{
    stack(children).direction(FlexDirection::Column)
}

pub fn hstack<K>(children: K) -> Stack<K>
where
    K: ElementTuple,
{
    stack(children).direction(FlexDirection::Row)
}

pub fn stack<K>(children: K) -> Stack<K>
where
    K: ElementTuple,
{
    Stack::new(children).shape(Rectangle::default()).render_mode(UIRenderMode::WithoutSelf)
}

pub trait ElementTuple {
    fn foreach_view<F: FnMut(Box<dyn UIElement>)>(&self, f: &mut F);
    fn is_empty(&self) -> bool {
        false
    }
}

#[derive(Clone)]
pub struct Stack<K>
where
    K: ElementTuple,
{
    children: K,
    element: Element,
}

impl<K> Stack<K>
where
    K: ElementTuple,
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

    pub fn title(mut self,title:&str) -> Self {
        self.element =  self.element.title(title);
        self
    }

    pub fn click(mut self, action: impl Fn(&mut Context) + Send + Sync + 'static) -> Self {
        self.element =  self.element.click(action);
        self
    }

    pub fn hover(mut self, action: impl Fn(&mut Context) + Send + Sync + 'static) -> Self {
        self.element =  self.element.hover(action);
        self
    }

    pub fn direction(mut self, direction:FlexDirection) -> Self {
        self.element = self.element.direction(direction);
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

    pub fn push_to_layout(self, layout: &mut SDUILayouts) {
        let node_id = layout.push_element(Box::new(self.element));
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
                let id = layout.push_element_with_id(child, p_id);
                for grand_child in new_children.drain(..) {
                    new_pairs.push((id,grand_child));
                }
            }
            children = new_pairs;
        }
    }
}

impl<K> UIElement for Stack<K>
where
    K: ElementTuple + Send + Sync + 'static
{
    fn draw(&self, painter: &mut ShapePainter) {
        self.element.draw(painter);
    }

    fn style(&self) -> Style {
        self.element.style()
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

    fn update(&mut self, cursor: (f32, f32), origin: Vec3, layout: &taffy::Layout, org: Vec3) {
        self.element.update(cursor, origin, layout,org);
    }

    fn exc(&mut self, context: &mut Context) {
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
}

macro_rules! impl_view_tuples{
    ($($element:ident),*) => {
        impl<$($element),*> ElementTuple for ($($element,)*)
        where
            $($element: UIElement + Clone + 'static),*
        {
            #[allow(non_snake_case, unused_variables)]
            #[track_caller]
            fn foreach_view<FN: FnMut(Box<dyn UIElement>)>(&self, f: &mut FN) {
                let ($($element,)*) = self;
                $(f(Box::new($element.clone()) as Box<dyn UIElement>);)*
            }
        }
    }
}

all_tuples!(impl_view_tuples, 0, 128, T);

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