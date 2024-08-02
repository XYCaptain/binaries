

use bevy::{ color::{palettes::css::SEA_GREEN, Srgba}, math::{Vec2, Vec3, Vec4}};
use bevy_vector_shapes::{prelude::ShapePainter, shapes::RectPainter};
use taffy::{prelude::length, Dimension, Rect, Size, Style};
use crate::{layout::Context, traits::UIElement};

use super::UIMouse;


#[derive(Clone)]
pub struct Element<F>
{
    tile: String,
    color: Srgba,
    size: Vec2,
    state: UIMouse,
    position: Vec3,
    isready: bool,
    margin: Vec4,
    padding: Vec4,
    shape: Option<Shape>,
    action: Option<F>,
}

#[derive(Clone)]
struct Shape{
    pub round: Vec4,
}

// F: Fn(&mut State, PointerButton) -> MessageResult<Action> + Send + Sync + 'static,
impl<F> Element<F>
where  F: Fn(&mut Context) + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            tile: "Button".to_string(),
            color: SEA_GREEN,
            size: Vec2::new(10.0, 10.0),
            position: Vec3::new(0.0, 0.0,0.0),
            state: UIMouse::Release,
            isready: false,
            action: None,
            shape: None,
            margin: Vec4::ZERO,
            padding: Vec4::ZERO,
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

    pub fn action(mut self, action: F) -> Self {
        self.action = Some(action);
        self
    }

    pub fn margin(mut self, margin: Vec4) -> Self {
        self.margin = margin;
        self
    }

    pub fn round(mut self, round: f32) -> Self {
        self.shape = Some(Shape{round: Vec4::splat(round)});    
        self
    }

    // pub fn padding(mut self, padding: Vec4) -> Self {
    //     self.padding = padding;
    //     self
    // }
}


impl<F> UIElement for Element<F>
where  F: Fn(&mut Context) + Send + Sync + 'static,
{
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
        if let Some(shaope) = self.shape.as_ref() {
            painter.corner_radii = shaope.round;
        }

        painter.rect(self.size);
        
        painter.corner_radii = Vec4::ZERO;
    }

    fn style(&self) -> Style {
        Style {
            size: Size {
                width: Dimension::Length(self.size.x),
                height: Dimension::Length(self.size.y),
            },
            margin: Rect {
                left: length(self.margin.x),
                right: length(self.margin.z),
                top: length(self.margin.y),
                bottom: length(self.margin.w),
            },
            padding: Rect {
                left: length(self.padding.x),
                right: length(self.padding.z),
                top: length(self.padding.y),
                bottom: length(self.padding.w),
            },
            ..Default::default()
        }
    }
    
    fn size(&self) -> (f32, f32) {
        (self.size.x, self.size.y)
    }
    
    fn isready(&self) -> bool {
        self.isready
    }

    fn setready(&mut self) {
        self.isready = true;
    }

    fn update(
        &mut self,
        cursor: (f32, f32),
        painter: &mut ShapePainter,
        layout:&taffy::Layout) {
        self.position = bevy::prelude::Vec3::new(painter.origin.unwrap().x + layout.location.x + self.size.x / 2., painter.origin.unwrap().y - self.size.y / 2. - layout.location.y,0.);
        let curo_screen = Vec2::new(cursor.0 + painter.origin.unwrap().x, cursor.1 - painter.origin.unwrap().y);

        if self.insection(curo_screen) {
            self.state = UIMouse::Hover;
        }
        else {
            self.state = UIMouse::Release;
        }
    }
    
    fn update_input_state(&mut self, state: UIMouse)  {
        match state {
            UIMouse::Click => {
                if self.state == UIMouse::Hover {
                    self.state = UIMouse::Click;
                }
            }
            UIMouse::Drag => {
                if self.state == UIMouse::Click {
                    self.state = UIMouse::Release;
                }
            }
            _=> {}
        }
    }

    fn exc(&mut self, context:&mut Context) {
        match self.state {
            UIMouse::Hover => {},
            UIMouse::Click => { self.action.as_ref().unwrap()(context);  self.state = UIMouse::Release;}, 
            UIMouse::Release => {},
            UIMouse::DoubleClick => todo!(),
            UIMouse::Drag => todo!(),
        }
    }
}