

use bevy::color::Srgba;
use element::{Callback, Element};


pub mod element;
pub mod stack;

#[derive(Clone,PartialEq,Debug,Copy)]
pub enum UIMouse {
    Hover,
    Click,
    Release,
    DoubleClick,
    Drag,
    NoneBlock
}


use crate::{layout::Context, traits::UIElement};

pub fn button() -> Element
{
    Element::new().color(Srgba::WHITE)
}

#[cfg(test)]
mod tests {
    use bevy::log::trace;
    use stack::ElementTuple;

    use super::*;

    #[test]
    fn test_button() {
        (
         button()
        ,button()
        )
        .foreach_view(&mut |child| {
            trace!("{:?}", child.style());
        });
    }

    #[test]
    fn test_element() {
        Element::new().action(|_:&mut Context|{});
    }

    #[test]
    fn test_element_tuple() {
        (
            Element::new().action(|_:&mut Context|{}),
            Element::new().action(|_:&mut Context|{})
        ).foreach_view(&mut |child| {
            trace!("{:?}", child.style());
        });
    }
}