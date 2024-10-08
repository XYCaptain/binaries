use std::f32::consts::PI;

use bevy::{color::Color, math::{Vec2, Vec3, Vec4}, prelude::{Commands, Transform}, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use bevy_vector_shapes::{prelude::ShapePainter, shapes::{DiscPainter, LinePainter, RectPainter, RegularPolygonPainter}};

use crate::Config;

pub trait ShapeTrait: Send + Sync + 'static {
    fn draw(&self, painter: &mut ShapePainter);
    fn set_size(&mut self, size: Vec2);
    fn get_size(&self) -> Option<Vec2> {
        None
    }
    fn set_round(&mut self,round:Vec4);
    fn update(&mut self,config: &mut Config, commands: &mut Commands, offset: Vec3);
    fn hits(&self,cursor: Vec2)->bool {
        let _ = cursor;
        false
    }
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

impl Rectangle {
    pub fn new(size:Vec2)->Self{
        Self{
            size,
            ..Default::default()
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
    
    fn update(&mut self,config: &mut Config, commands: &mut Commands, offset: Vec3){
        let _ = offset;
        let _ = commands;
        let _ = config;
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

    fn update(&mut self,config: &mut Config, commands: &mut Commands, offset: Vec3){
        let _ = offset;
        let _ = commands;
        let _ = config;
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

    fn update(&mut self,config: &mut Config, commands: &mut Commands, offset: Vec3){
        let _ = offset;
        let _ = commands;
        let _ = config;
    }

    fn hits(&self,cursor:Vec2)-> bool {
        cursor.length() < self.radius
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
            thickness: 2.5,
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

    fn update(&mut self,config: &mut Config, commands: &mut Commands, offset: Vec3){
        let _ = offset;
        let _ = commands;
        let _ = config;
    }
}

#[derive(Clone,Debug)]
pub struct Text {
    pub content: String,
    pub round: Vec4,
    pub size:Vec2,
    pub font_size : f32,
    pub content_size:Vec2,
    pub mesh: Vec<Vec<Vec2>>,
    pub entity: Option<bevy::ecs::entity::Entity>,
    pub screen_position: Vec2,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            content:"".to_string(),
            round:Vec4::ZERO,
            size:Vec2::ZERO,
            font_size: 1.,
            mesh:Vec::new(),
            content_size: Vec2::ZERO,
            entity: None,
            screen_position: Vec2::ZERO,
        }
    }
}

impl Text {
    pub fn new(content:String)->Self{
        Self{
            content,
            ..Default::default()
        }
    }
}

impl ShapeTrait for Text {
    fn draw(&self, painter: &mut ShapePainter) {
        painter.rect(self.size);
        //    painter.translate(Vec3::new(-self.content_size.x * 0.5,self.content_size.y * 0.5,0.));
        //    for p in &self.mesh {
        //      painter.triangle(p[0],p[1],p[2]);
        //    }
    }
    
    fn set_size(&mut self, size: Vec2) {
        self.size = size;
        self.font_size = size.y;
    }

    fn get_size(&self) -> Option<Vec2> {
       Some(self.content_size)
    }

    fn set_round(&mut self,round:Vec4) {
        let _ = round;
    }

    fn update(&mut self,config: &mut Config, commands: &mut Commands, offset: Vec3){
        match self.entity {
            Some(entity_id) => {
                commands.entity(entity_id).insert(
                    Transform::from_xyz(offset.x -self.content_size.x * 0.5,offset.y + self.content_size.y * 0.5,1.)
                );
            },
            None => {
                let text =  crate::text::TextShape::builder().size(self.font_size).build(config.default_font.font(), &self.content);
                self.content_size = Vec2::new(text.witdh, text.height);
                let text_meshes = Mesh2dHandle(config.meshes.add(text.bevy_mesh()));
                let origin =  offset;
                self.entity = Some(commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: text_meshes.clone(),
                        transform: Transform::from_xyz(origin.x -self.content_size.x * 0.5,origin.y + self.content_size.y * 0.5,1.),
                        material: config.materials.add(Color::srgb(1., 1., 1.)),    
                        ..Default::default()
                    },
                )).id());
            },
        }
    }
}