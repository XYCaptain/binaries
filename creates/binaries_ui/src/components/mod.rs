
use bevy::{color::Srgba, math::VectorSpace};
use element::Element;

pub mod element;
pub mod stacks;
pub mod element_set;

#[derive(Clone,PartialEq,Debug,Copy)]
pub enum UIMouseState {
    Hover,
    Click,
    Release,
    DoubleClick,
    Drag,
    NoneBlock
}

#[derive(Clone,PartialEq,Debug,Copy)]
pub enum UIRenderMode {
    Individual,
    Group,
    WithoutSelf
}

use crate::shape::{Circle, Ngon, Rectangle, Text};

pub fn element() -> Element
{
    Element::new().color(Srgba::ZERO)
}

pub fn debug_tree()-> Element
{
    Element::new().title("debuge")
}

pub fn text(content:&str)->Element
{
    Element::new().color(Srgba::WHITE).shape(Text::new(content.to_string())).title("text content")
}

pub fn rectangle() -> Element
{
    Element::new().color(Srgba::WHITE).shape(Rectangle::default())
}

pub fn circle() -> Element
{
    Element::new().color(Srgba::WHITE).shape(Circle::default())
}

pub fn ngon(sides:f32) -> Element
{
    Element::new().color(Srgba::WHITE).shape(Ngon::default().sides(sides))
}

#[cfg(test)]
mod tests {
    use bevy::log::trace;
    use element_set::ElementSet;
    use crate::layout::Context;

    use super::*;

    #[test]
    fn test_button() {
        (
         element()
        ,element()
        )
        .foreach_view(&mut |child| {
            trace!("{:?}", child.style());
        });
    }

    #[test]
    fn test_element() {
        Element::new().click(|_:&mut Context|{});
    }

    #[test]
    fn test_element_tuple() {
        (
            Element::new().click(|_:&mut Context|{}),
            Element::new().click(|_:&mut Context|{})
        ).foreach_view(&mut |child| {
            trace!("{:?}", child.style());
        });
    }
}