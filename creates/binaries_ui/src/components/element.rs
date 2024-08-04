use std::{any::Any, f64::consts::E, rc::Rc, sync::Arc};

use crate::{layout::Context, traits::UIElement,shape::Rectangle};
use bevy::{
    color::{palettes::css::SEA_GREEN, Srgba},
    math::{i32, Vec2, Vec3, Vec4},
};
use bevy_vector_shapes::{prelude::ShapePainter, shapes::RectPainter};
use taffy::{prelude::length, Dimension, Rect, Size, Style};
use crate::shape::ShapeTrait;
use super::UIMouse;

#[derive(Clone,Debug)]
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

#[derive(Clone)]
pub struct Element<F> {
    zorder: i32,
    tile: String,
    color: Srgba,
    size: Vec2,
    state: UIMouse,
    position: Vec3,
    isready: bool,
    margin: Vec4,
    padding: Vec4,
    shape: Option<Arc<dyn ShapeTrait>>,
    action: Option<F>,
    draw: Option<F>,
    direction: FlexDirection
}

struct Action<F> {
    f: F,
}

pub trait ActionFn {
    fn call(&self, cx: &mut Context);
}

impl<F> Element<F>
where
    F: Fn(&mut Context) + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            tile: "Button".to_string(),
            color: Srgba::new(0.0, 0.0, 0.0, 0.0),
            size: Vec2::new(0.0, 0.0),
            position: Vec3::new(0.0, 0.0, 0.0),
            state: UIMouse::Release,
            isready: false,
            action: None,
            draw: None,
            shape: None,
            margin: Vec4::ZERO,
            padding: Vec4::ZERO,
            zorder: 1,
            direction: FlexDirection::Row,
        }
    }

    pub fn insection(&self, point: Vec2) -> bool {
        if point.x > self.position.x - self.size.x / 2.
            && point.x < self.position.x + self.size.x / 2.
        {
            if point.y > -self.position.y - self.size.y / 2.
                && point.y < -self.position.y + self.size.y / 2.
            {
                return true;
            }
        }
        return false;
    }

    pub fn color(mut self, color: Srgba) -> Self {
        self.color = color;
        self
    }

    pub fn title(mut self, tile: &str) -> Self {
        self.tile = tile.to_string();
        self
    }

    pub fn size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }

    pub fn action(mut self, action: Option<F>) -> Self {
        self.action = action;
        self
    }

    pub fn primatives(mut self, draw: Option<F>) -> Self {
        self.draw = draw;
        self
    }

    pub fn margin(mut self, margin: Vec4) -> Self {
        self.margin = margin;
        self
    }

    pub fn round(mut self, round: f32) -> Self {
        self.shape = Some(Arc::new(Rectangle {round: Vec4::splat(round), size: self.size }));
        println!("rounding {}",self.size);
        self
    }
 
    pub fn get_size(&self) -> Vec2 {
        self.size
    }

    pub fn order(mut self, zorder: i32) -> Self {
        self.zorder = zorder;
        self
    }

    pub fn direction(mut self, direction: FlexDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn padding(mut self, padding: Vec4) -> Self {
        self.padding = padding;
        self
    }

    pub fn shape(mut self, shape: Arc<dyn ShapeTrait>   ) -> Self {
        self.shape = Some(shape);
        self
    }
}

impl<F> UIElement for Element<F>
where
    F: Fn(&mut Context) + Send + Sync + 'static,
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

        if let Some(shape) = self.shape.as_ref() {
             shape.draw(painter);
        }
        
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
            flex_direction: match self.direction {
                FlexDirection::Row => taffy::FlexDirection::Row,
                FlexDirection::Column => taffy::FlexDirection::Column,
                FlexDirection::RowReverse => taffy::FlexDirection::RowReverse,
                FlexDirection::ColumnReverse => taffy::FlexDirection::ColumnReverse,
            },
            ..Default::default()
        }
    }

    fn size(&self) -> (f32, f32) {
        (self.size.x, self.size.y)
    }

    fn is_ready(&self) -> bool {
        self.isready
    }

    fn set_ready(&mut self) {
        self.isready = true;
    }

    fn update(&mut self, cursor: (f32, f32), painter: &mut ShapePainter, layout: &taffy::Layout, org:Vec3) {

        self.position = bevy::prelude::Vec3::new(
            painter.origin.unwrap().x + layout.location.x + self.size.x / 2. + org.x,
            painter.origin.unwrap().y - self.size.y / 2. - layout.location.y - org.y,
            0.,
        );

        let curo_screen = Vec2::new(
            cursor.0 + painter.origin.unwrap().x,
            cursor.1 - painter.origin.unwrap().y,
        );

        if cursor.0 < 0.{
            return;
        }

        if self.insection(curo_screen) {
            self.state = UIMouse::Hover;
        } else {
            self.state = UIMouse::Release;
        }
    }

    fn update_input_state(&mut self, state: UIMouse) {
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
            _ => {}
        }
    }

    fn exc(&mut self, context: &mut Context) {
        match self.state {
            UIMouse::Hover => {
            }
            UIMouse::Click => {
                match self.action.as_ref() {
                    Some(action) => {
                        action(context);
                    }
                    None => {
                        println!("{} no action",self.tile);
                    }
                }
                self.state = UIMouse::Release;
            }
            UIMouse::Release => {}
            UIMouse::DoubleClick => todo!(),
            UIMouse::Drag => todo!(),
        }
    }

    fn get_z_order(&self) -> i32 {
        self.zorder
    }

    fn set_z_order(&mut self,z_order:i32) -> i32 {
        self.zorder = z_order;
        self.zorder
    }
    
    fn get_children(&self) -> Option<Vec<Box<dyn UIElement>>> {
        None
    }
}
