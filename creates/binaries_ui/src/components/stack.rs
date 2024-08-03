use crate::layout::SDUILayouts;
use bevy::color::Srgba;
use bevy::math::{Vec2, Vec4};
use bevy::utils::all_tuples;

use super::element::Element;
use super::{button, UIMouse};
use crate::{layout::Context, traits::UIElement};
use bevy_vector_shapes::prelude::ShapePainter;
use taffy::Style;

pub fn stack<K,F>(children: K,action: F) -> Stack<K,F>
where
    K: ElementTuple,
    F: Fn(&mut Context) + Send + Sync + 'static,
{
    Stack::new(children).element(button(action))
}

pub trait ElementTuple {
    fn foreach_view<F: FnMut(Box<dyn UIElement>)>(&self, f: &mut F);
    fn is_empty(&self) -> bool {
        false
    }
}

#[derive(Clone)]
pub struct Stack<K,F>
where
    K: ElementTuple,
    F: Fn(&mut Context) + Send + Sync + 'static,
{
    children: K,
    element: Element<F>,
}

impl<K,F> Stack<K,F>
where
    K: ElementTuple,
    F: Fn(&mut Context) + Send + Sync + 'static,
{
    pub fn new(children: K) -> Self {
        Self 
        {
            children,
            element: Element::new(),
        }
    }

    pub fn element(mut self, element: Element<F>) -> Self {
        self.element = element;
        self
    }

    pub fn size(mut self,size:Vec2) -> Self {
        self.element =  self.element.size(size);
        self
    }

    pub fn color(mut self,color:Srgba) -> Self {
        self.element =  self.element.color(color);
        self
    }

    pub fn push_to_layout(mut self, layout: &mut SDUILayouts) {
        let zorder =  self.element.set_z_order(0);
        let node_id = layout.push_element(Box::new(self.element));
        self.children.foreach_view(&mut |mut element| {
            element.set_z_order(zorder+1);
            layout.push_element_with_id(element, node_id);
        });
    }
}

impl<K,F> UIElement for Stack<K,F>
where
    K: ElementTuple + Send + Sync,
    F: Fn(&mut Context) + Send + Sync + 'static,
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

    fn update(&mut self, cursor: (f32, f32), painter: &mut ShapePainter, layout: &taffy::Layout) {
        self.element
            .update(cursor, painter, layout);
    }

    fn update_input_state(&mut self, state: UIMouse) {
        self.element.update_input_state(state);
    }

    fn exc(&mut self, context: &mut Context) {
        self.element.exc(context);
    }

    fn z_order(&self) -> i32 {
        self.element.z_order()
    }

    fn set_z_order(&mut self, z_order: i32) -> i32{
        self.element.set_z_order(z_order);
        self.z_order()
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

    use super::*;

    #[test]
    fn test_button() {

    }

    #[test]
    fn test_stack() {
        stack(
            (
                button(|_: &mut Context| {}),
                button(|_: &mut Context| {})
            ),
            |_: &mut Context| {}
        );
    }

    #[test]
    fn test_element_tuple() {
        (
            stack(
                (
                    button(|_: &mut Context| {}),
                    button(|_: &mut Context| {})
                ),
                |_: &mut Context| {}
            ),
            stack(
                (
                    button(|_: &mut Context| {}),
                    button(|_: &mut Context| {})
                ),
                |_: &mut Context| {}
            )
        ).foreach_view(&mut |child| {
              println!("{:?}", child.style());
        });
    }
}