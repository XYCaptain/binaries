use std::f32::consts::PI;

use bevy::{math::{Vec2, Vec4}, scene::ron::de};
use bevy_vector_shapes::{prelude::ShapePainter, shapes::{RectPainter, RegularPolygonPainter}};

#[derive(Clone,Debug)]
pub struct Rectangle {
    pub round: Vec4,
    pub size: Vec2,
}

pub trait ShapeTrait: Send + Sync + 'static {
    fn draw(&self, painter: &mut ShapePainter);
    fn set_size(&mut self, size: Vec2);
}

impl ShapeTrait for Rectangle {
    fn draw(&self, painter: &mut ShapePainter) {
        painter.corner_radii = self.round;
        painter.rect(self.size);
    }
    
    fn set_size(&mut self, size: Vec2) {
        self.size = size;
    }
}

#[derive(Clone,Debug)]
pub struct Ngon {
    pub round: Vec4,
    pub sides: f32,
    pub radius: f32,
    pub rotation: f32,
}

impl Default for Ngon {
    fn default() -> Self {
        Self {
            round: Vec4::splat(10.0),
            sides: 3.,
            radius: 1.,
            rotation: 0.,
        }
    }
}

impl ShapeTrait for Ngon {
    fn draw(&self, painter: &mut ShapePainter) {
        painter.rotate_z(self.rotation/180. * PI);
        painter.corner_radii = self.round;
        painter.ngon(self.sides, self.radius);
    }
    
    fn set_size(&mut self, size: Vec2) {
        self.radius = size.x.min(size.y);
    }
}
