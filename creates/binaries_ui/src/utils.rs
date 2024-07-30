use bevy::color::{Alpha, Srgba};

pub trait Pastel {
    fn pastel(&self) -> Srgba;
}

impl Pastel for Srgba {
    fn pastel(&self) -> Srgba {
        let mut out = *self + Srgba::WHITE * 0.25;
        out.set_alpha(1.0);
        out
    }
}