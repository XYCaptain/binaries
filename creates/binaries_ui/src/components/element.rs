use std::sync::{Arc, RwLock};

use super::UIMouse;
use crate::shape::ShapeTrait;
use crate::{layout::Context, traits::UIElement};
use bevy::{
    color::Srgba,
    math::{Vec2, Vec3, Vec4},
};
use bevy_vector_shapes::prelude::ShapePainter;
use taffy::{prelude::length, Dimension, Rect, Size, Style};

#[derive(Clone, Debug)]
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

pub(crate) type Callback = Arc<dyn Fn(&mut Context) + Send + Sync + 'static>;
pub(crate) type Shpe = Arc<RwLock<dyn ShapeTrait>>;

#[derive(Clone)]
pub struct Action {
    pub(crate) hover: Option<Callback>,
    pub(crate) click: Option<Callback>,
}

impl Default for Action {
    fn default() -> Self {
        Self {
            hover: None,
            click: None,
        }
    }
}

#[derive(Clone)]
pub struct Element {
    zorder: i32,
    pub(crate) tile: String,
    color: Srgba,
    size: Vec2,
    action_state: UIMouse,
    render_state: UIMouse,
    render_block: bool,
    position: Vec3,
    isready: bool,
    margin: Vec4,
    padding: Vec4,
    shape: Option<Shpe>,
    action: Action,
    draw: Option<Callback>,
    direction: FlexDirection,
}

impl Element {
    pub fn new() -> Self {
        Self {
            tile: "element".to_string(),
            color: Srgba::new(0.0, 0.0, 0.0, 0.0),
            size: Vec2::new(0.0, 0.0),
            position: Vec3::new(0.0, 0.0, 0.0),
            action_state: UIMouse::Release,
            isready: false,
            action: Action::default(),
            draw: None,
            shape: None,
            margin: Vec4::ZERO,
            padding: Vec4::ZERO,
            zorder: 1,
            direction: FlexDirection::Row,
            render_state: UIMouse::Release,
            render_block: false,
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
        if let Some(shape) = self.shape.as_ref() {
            shape.write().unwrap().set_size(size);
        }
        self
    }

    pub fn click(mut self, action: impl Fn(&mut Context) + Send + Sync + 'static) -> Self {
        self.action.click = Some(Arc::new(action));
        self
    }

    pub fn hover(mut self, action: impl Fn(&mut Context) + Send + Sync + 'static) -> Self {
        self.action.hover = Some(Arc::new(action));
        self
    }

    pub fn primatives(mut self, draw: Option<Callback>) -> Self {
        self.draw = draw;
        self
    }

    pub fn margin(mut self, margin: Vec4) -> Self {
        self.margin = margin;
        self
    }

    pub fn round(self, round: f32) -> Self {
        if let Some(shape) = self.shape.as_ref() {
            shape.write().unwrap().set_round(Vec4::splat(round));
        }
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

    pub fn shape(mut self, shape: impl ShapeTrait) -> Self {
        self.shape = Some(Arc::new(RwLock::new(shape)));
        self
    }

    pub fn render_block(mut self, is_blcok: bool) -> Self {
        self.render_block = is_blcok;
        self
    }
}

impl UIElement for Element {
    fn draw(&self, painter: &mut ShapePainter) {
        let mut color = self.color;
        match self.render_state {
            UIMouse::Hover | UIMouse::Click  => {
                if self.action.hover.is_some()
                {
                    color = color * 0.75 + Srgba::WHITE * 0.25;
                }
            }
            UIMouse::Release => {
                // painter.set_color(self.color);
            }
            _ => {
                // painter.set_color(self.color);
            }
        }
        painter.set_color(color);
        painter.set_translation(self.position);

        if let Some(shape) = self.shape.as_ref() {
            shape.read().unwrap().draw(painter);
        }

        painter.corner_radii = Vec4::ZERO;
    }

    fn exc(&mut self, context: &mut Context) {
        match self.action_state {
            UIMouse::Hover => {
                match &self.action.hover {
                    Some(action) => action(context),
                    None => {},
                }
            }
            UIMouse::Click => {
                println!("{} click",self.tile);
                match &self.action.click {
                    Some(action) => action(context),
                    None => {}
                }
                self.action_state = UIMouse::Release;
            }
            UIMouse::Release => {}
            UIMouse::DoubleClick => todo!(),
            UIMouse::Drag => todo!(),
            UIMouse::NoneBlock => {}
        }
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

    fn update(
        &mut self,
        cursor: (f32, f32),
        origin: Vec3,
        layout: &taffy::Layout,
        org: Vec3,
    ) {
        self.position = bevy::prelude::Vec3::new(
            origin.x + layout.location.x + self.size.x / 2. + org.x,
            origin.y - self.size.y / 2. - layout.location.y - org.y,
            0.,
        );

        let curo_screen = Vec2::new(
            cursor.0 + origin.x,
            cursor.1 - origin.y,
        );

        if cursor.0 < 0. {
            return;
        }

        if self.insection(curo_screen) {
            self.action_state = UIMouse::Hover;
            self.render_state = UIMouse::Hover;
        } else {
            self.action_state = UIMouse::Release;
            self.render_state = UIMouse::Release;
        }
    }

    fn set_action_state(&mut self, state: UIMouse) {
        match state {
            UIMouse::Click => {
                if self.action_state == UIMouse::Hover {
                    self.action_state = UIMouse::Click;
                    self.render_state = UIMouse::Click;
                }
            }
            UIMouse::Drag => {
                if self.action_state == UIMouse::Click {
                    self.action_state = UIMouse::Release;
                    self.render_state = UIMouse::Release;
                }
            }
            _ => {}
        }
    }

    fn get_z_order(&self) -> i32 {
        self.zorder
    }

    fn set_z_order(&mut self, z_order: i32) -> i32 {
        self.zorder = z_order;
        self.zorder
    }

    fn get_children(&self) -> Option<Vec<Box<dyn UIElement>>> {
        None
    }

    fn get_input_state(&mut self) -> UIMouse {
        self.action_state.clone()
    }

    fn get_render_state(&mut self) -> UIMouse {
        self.render_state.clone()
    }

    fn set_render_state(&mut self, state: UIMouse) {
        self.render_state = state;
    }

    fn block_render_state(&mut self) -> bool {
        self.render_block
    }
}
