

use bevy::color::Srgba;
use element::Element;


pub mod element;
pub mod stack;

#[derive(Clone,PartialEq,Debug)]
pub enum UIMouse {
    Hover,
    Click,
    Release,
    DoubleClick,
    Drag
}

use crate::layout::Context;

pub fn button<F>(action: F) -> Element<F>
where F: Fn(&mut Context) + Send + Sync + 'static
{
    Element::new().color(Srgba::WHITE).action(Some(action))
}

#[cfg(test)]
mod tests {
    use bevy::log::trace;
    use stack::ElementTuple;

    use super::*;

    #[test]
    fn test_button() {
        (
         button(|_|{})
        ,button(|_|{}))
        .foreach_view(&mut |child| {
            trace!("{:?}", child.style());
        });
    }

    #[test]
    fn test_element() {
        Element::new().action(Some(|_:&mut Context|{}));
    }

    #[test]
    fn test_element_tuple() {
        (
            Element::new().action(Some(|_:&mut Context|{})),
            Element::new().action(Some(|_:&mut Context|{}))
        ).foreach_view(&mut |child| {
            trace!("{:?}", child.style());
        });
    }
}