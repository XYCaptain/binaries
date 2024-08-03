use crate::layout::SDUILayouts;
use bevy::color::Srgba;
use bevy::math::{Vec2, Vec3};
use bevy::utils::all_tuples;

use super::element::Element;
use super::UIMouse;
use crate::{layout::Context, traits::UIElement};
use bevy_vector_shapes::prelude::ShapePainter;
use taffy::Style;
use crate::components::element::FlexDirection;

pub fn stack<K,F>(children: K,action: F) -> Stack<K,F>
where
    K: ElementTuple,
    F: Fn(&mut Context) + Send + Sync + 'static,
{
    Stack::new(children).action(action)
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

    pub fn size(mut self,size:Vec2) -> Self {
        self.element =  self.element.size(size);
        self
    }

    pub fn color(mut self,color:Srgba) -> Self {
        self.element =  self.element.color(color);
        self
    }

    pub fn title(mut self,tile:&str) -> Self {
        self.element =  self.element.tile(tile.to_string());
        self
    }

    pub fn action(mut self,action:F) -> Self {
        self.element =  self.element.action(Some(action));
        self
    }

    pub fn direction(mut self, direction:FlexDirection) -> Self {
        self.element = self.element.direction(direction);
        self
    }

    pub fn push_to_layout(mut self, layout: &mut SDUILayouts) {
        let mut zorder =  self.element.set_z_order(0);
        let node_id = layout.push_element(Box::new(self.element));
        let mut children = Vec::new();
        self.children.foreach_view(&mut |element| {
            children.push((node_id,element));
        });

        while !children.is_empty() {
            zorder = zorder + 1;
            let mut new_children = Vec::new();
            let mut new_pairs = Vec::new();
            for  (p_id,mut child) in children {
                child.set_z_order(zorder);
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

    fn update(&mut self, cursor: (f32, f32), painter: &mut ShapePainter, layout: &taffy::Layout, org: Vec3) {
        self.element.update(cursor, painter, layout,org);
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
    
    fn get_children(&self) -> Option<Vec<Box<dyn UIElement>>> {
        let mut children = Vec::new();
        self.children.foreach_view(&mut |child| {
            children.push(child);
        });
        children.into()
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
    use crate::components::button;

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