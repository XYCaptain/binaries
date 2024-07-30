
use bevy::{ color::{palettes::css::SEA_GREEN, Srgba}, math::{Vec2, Vec3, Vec4}};
use bevy_vector_shapes::{prelude::ShapePainter, shapes::RectPainter};
use taffy::{prelude::length, Dimension, Rect, Size, Style};
use crate::traits::UIElement;

use super::UIMouse;

pub struct Element<T: UIElement>{
    pub id: u32,
    pub name: Option<String>,
    pub color: Srgba,
    pub element: T,
}

pub struct Button {
    pub text: String,
    pub color: Srgba,
    pub size: Vec2,
    pub position: Vec2,
    pub entity: Option<Box<dyn UIElement>>,
}
#[derive(Clone)]
pub struct SDButton{
    tile: String,
    color: Srgba,
    size: Vec2,
    state: UIMouse,
    pub position: Vec3,
}

impl SDButton {
    pub fn new() -> Self {
        Self {
            tile: "Button".to_string(),
            color: SEA_GREEN,
            size: Vec2::new(10.0, 10.0),
            position: Vec3::new(0.0, 0.0,0.0),
            state: UIMouse::Release
        }
    }

    pub fn insection(&self, point: Vec2) -> bool {
        if point.x > self.position.x - self.size.x / 2. && point.x < self.position.x + self.size.x / 2. {
            if point.y > -self.position.y - self.size.y / 2. && point.y < -self.position.y + self.size.y / 2. {
                return true;
            }
        }
        return false;
    }
    
    pub fn color(mut self, color: Srgba) -> Self {
        self.color = color;
        self
    }

    pub fn tile(mut self, tile: String) -> Self {
        self.tile = tile;
        self
    }

    pub fn size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }
}

impl UIElement for SDButton {
    fn draw(&self, painter: &mut ShapePainter) {
        match self.state {
            UIMouse::Hover => {
                painter.set_color(self.color + Srgba::WHITE * 0.25);
            }
            UIMouse::Click => {
                painter.set_color(self.color + Srgba::WHITE * 0.5);
            }
            UIMouse::Release => {
                painter.set_color(self.color);
            }
            _ => {
                painter.set_color(self.color);
            }
        }
        painter.set_translation(self.position);
        painter.corner_radii = Vec4::ZERO;
        painter.rect(self.size);
    }

    fn style(&self) -> Style {
        Style {
            size: Size {
                width: Dimension::Length(self.size.x),
                height: Dimension::Length(self.size.y),
            },
            margin: Rect {
                left: length(10f32),
                right: length(15f32),
                top: length(20f32),
                bottom: length(30f32),
            },
            ..Default::default()
        }
    }
    
    fn size(&self) -> (f32, f32) {
        (self.size.x, self.size.y)
    }
    
    fn update(
        &mut self,
        cursor: (f32, f32),
        painter: &mut ShapePainter,
        layout:&taffy::Layout) {
        self.position = bevy::prelude::Vec3::new(painter.origin.unwrap().x + layout.location.x + self.size.x/2., painter.origin.unwrap().y - self.size.y /2. + layout.location.y,0.);
        let curo_screen = Vec2::new(cursor.0 + painter.origin.unwrap().x, cursor.1 - painter.origin.unwrap().y);

        if self.insection(curo_screen) {
            self.state = UIMouse::Hover;
        }
        else {
            self.state = UIMouse::Release;
        }
    }
}