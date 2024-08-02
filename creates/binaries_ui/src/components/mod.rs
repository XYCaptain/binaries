

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

pub fn button<F:Fn(&mut Context) + Send + Sync + 'static>(action: F) -> Element<F>
{
    Element::new().color(Srgba::WHITE).action(action)
}

#[cfg(test)]
mod tests {
    use bevy::log::trace;
    use stack::ElementTuple;

    use super::*;

    #[test]
    fn test_sd_button() {
        (button(|_|{ }),button(|_|{})).foreach_view(&mut |child| {
            trace!("{:?}", child.style());
        });
    }
}