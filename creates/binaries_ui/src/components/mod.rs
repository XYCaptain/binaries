use bevy::color::Srgba;
use button::SDButton;

pub mod button;
pub mod stack;

#[derive(Clone)]
pub enum UIMouse {
    Hover,
    Click,
    Release,
    DoubleClick,
    Drag
}

pub fn button() -> SDButton {
    SDButton::new().color(Srgba::WHITE)
}


#[cfg(test)]
mod tests {
    use bevy::log::trace;
    use stack::ElementTuple;
    use super::*;

    #[test]
    fn test_sd_button() {
        (button(),button()).foreach_view(&mut |child| {
            trace!("{:?}", child.style());
        });
    }
}