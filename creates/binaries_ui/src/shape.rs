use std::f32::consts::PI;

use bevy::math::{Vec2, Vec3, Vec4};
use bevy_vector_shapes::{prelude::ShapePainter, shapes::{DiscPainter, LinePainter, RectPainter, RegularPolygonPainter}};

pub trait ShapeTrait: Send + Sync + 'static {
    fn draw(&self, painter: &mut ShapePainter);
    fn set_size(&mut self, size: Vec2);
    fn set_round(&mut self,round:Vec4);
}

#[derive(Clone,Debug)]
pub struct Rectangle {
    pub round: Vec4,
    pub size: Vec2,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            round: Vec4::splat(0.0),
            size: Vec2::splat(0.0)
        }
    }
}

impl ShapeTrait for Rectangle {
    fn draw(&self, painter: &mut ShapePainter) {
        painter.corner_radii = self.round;
        painter.rect(self.size);
    }
    
    fn set_size(&mut self, size: Vec2) {
        self.size = size;
    }
    
    fn set_round(&mut self,round:Vec4) {
        self.round = round;
    }
}

#[derive(Clone,Debug)]
pub struct Ngon {
    pub round: Vec4,
    pub sides: f32,
    pub radius: f32,
    pub rotation: f32,
}
impl Ngon {
    pub fn sides(mut self, sides:f32) -> Self{
        self.sides = sides;
        self
    }
}

impl Default for Ngon {
    fn default() -> Self {
        Self {
            round: Vec4::splat(0.0),
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
        self.radius = size.x.min(size.y) * 0.5;
    }

    fn set_round(&mut self,round:Vec4) {
        self.round = round;
    }
}


#[derive(Clone,Debug)]
pub struct Circle {
    pub round: Vec4,
    pub radius: f32,
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            round: Vec4::splat(0.0),
            radius: 1.,
        }
    }
}

impl ShapeTrait for Circle {
    fn draw(&self, painter: &mut ShapePainter) {
        painter.translate(Vec3::Z);
        painter.circle(self.radius);
    }
    
    fn set_size(&mut self, size: Vec2) {
        self.radius = size.x.min(size.y) * 0.5;
    }

    fn set_round(&mut self,round:Vec4) {
        self.round = round;
    }
}

#[derive(Clone,Debug)]
pub struct Curve {
    pub round: Vec4,
    pub star: Vec3,
    pub end: Vec3,
    pub thickness: f32,
}

impl Default for Curve {
    fn default() -> Self {
        Self {
            round: Vec4::splat(0.0),
            star: Vec3::ZERO,
            end: Vec3::ZERO,
            thickness: 5.,
        }
    }
}

impl Curve {
    pub fn new(star:Vec3,end:Vec3)->Self{
        Self{
            star,
            end,
            ..Default::default()
        }
    }
}

impl ShapeTrait for Curve {
    fn draw(&self, painter: &mut ShapePainter) {
        painter.translate(Vec3::Z);
        painter.thickness = self.thickness;
        let delta_h=  self.end - self.star;
        painter.line(self.star, self.star + Vec3::Y * 0.5 * delta_h.y);
        painter.line(self.star + Vec3::Y * 0.5 * delta_h.y, self.end - Vec3::Y * 0.5 * delta_h.y);
        painter.line(self.end - Vec3::Y * 0.5 * delta_h.y, self.end);
    }
    
    fn set_size(&mut self, size: Vec2) {
        self.thickness = size.x.min(size.y) * 0.5;
    }

    fn set_round(&mut self,round:Vec4) {
        self.round = round;
    }
}