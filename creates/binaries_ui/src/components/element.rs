use std::sync::{Arc, RwLock};

use super::{UIMouseState, UIRenderMode};
use crate::shape::ShapeTrait;
use crate::{layout::Context, traits::UIElement};
use bevy::color::palettes::css::{BLACK, BLUE_VIOLET};
use bevy::color::palettes::tailwind::RED_900;
use bevy::math::{Quat, Vec3, Vec4, VectorSpace};
use bevy::{color::Srgba,math::Vec2};
use bevy_vector_shapes::prelude::ShapePainter;
use bevy_vector_shapes::shapes::RectPainter;
use taffy::prelude::auto;
use taffy::Position;
use taffy::{prelude::length, Dimension, Rect, Size, Style};

#[derive(Clone, Debug)]
pub enum ElementType {
    Content,
    Debug
}

#[derive(Clone, Debug)]
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

#[derive(Clone, Debug)]
pub enum AlignItems {
    Start,
    End,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
    NotSet
}

// #[derive(Clone, Debug)]
// pub enum AlignContent {
//     Start,
//     End,
//     FlexStart,
//     FlexEnd,
//     Center,
//     Stretch,
//     SpaceBetween,
//     SpaceEvenly,
//     SpaceAround,
// }


pub(crate) type Callback = Arc<dyn Fn(&mut Context) + Send + Sync + 'static>;
pub(crate) type Renderback = Arc<dyn Fn(&mut ShapePainter) + Send + Sync + 'static>;
pub(crate) type Shape = Arc<RwLock<dyn ShapeTrait>>;

#[derive(Clone)]
pub struct RenderAction {
    pub(crate) hover: Option<Renderback>,
    pub(crate) click: Option<Renderback>,
}

impl Default for RenderAction {
    fn default() -> Self {
        let fun = Arc::new(
            |painter: &mut ShapePainter|{
                let mut color = BLUE_VIOLET;
                // color.alpha = 1.0;
                painter.set_color(color);
            }
        );
        Self {
            hover: Some(fun.clone()),
            click: Some(fun.clone()),
        }
    }
}

impl RenderAction {
    fn empty() -> Self {
        Self {
            hover: None,
            click: None,
        }
    }
}

#[derive(Clone)]
pub struct IunputAction {
    pub(crate) hover: Option<Callback>,
    pub(crate) click: Option<Callback>,
}

impl Default for IunputAction {
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
    background_color: Srgba,
    size: Vec2,
    pub(crate) content_size: Vec2,
    action_state: UIMouseState,
    render_state: UIMouseState,
    render_block: UIRenderMode,
    pub(crate) position: Vec3,
    isready: bool,
    margin: Vec4,
    padding: Vec4,
    pub(crate) shape: Option<Shape>,
    action: IunputAction,
    render: RenderAction,
    draw: Option<Callback>,
    pub(crate) direction: FlexDirection,
    pub(crate) main_axis_alignment: AlignItems,
    pub(crate) cors_axis_alignment: AlignItems,
    pub(crate) self_main_axis_alignment: AlignItems,
    pub(crate) self_cors_axis_alignment: AlignItems,
    element_type: ElementType
}

impl Element {
    pub fn new() -> Self {
        Self {
            tile: "element".to_string(),
            color: Srgba::ZERO,
            background_color: Srgba::ZERO,
            size: Vec2::ZERO,
            content_size: Vec2::ZERO,
            position: Vec3::ZERO,
            action_state: UIMouseState::Release,
            isready: false,
            action: IunputAction::default(),
            render: RenderAction::default(),
            draw: None,
            shape: None,
            margin: Vec4::ZERO,
            padding: Vec4::ZERO,
            zorder: 1,
            direction: FlexDirection::Row,
            render_state: UIMouseState::Release,
            render_block: UIRenderMode::Individual,
            main_axis_alignment: AlignItems::Start,
            cors_axis_alignment: AlignItems::Start,
            self_main_axis_alignment: AlignItems::NotSet,
            self_cors_axis_alignment: AlignItems::NotSet,
            element_type: ElementType::Content
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

    pub fn background_color(mut self, color: Srgba) -> Self {
        self.background_color = color;
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

    pub fn render_block(mut self, ui_traverse: UIRenderMode) -> Self {
        self.render_block = ui_traverse;
        self
    }

    pub fn set_position(mut self,pos:Vec3) -> Self{
        self.position = pos;
        self
    }

    pub fn horizontal_alignment(mut self,align:AlignItems)->Self{
        match self.direction{
            FlexDirection::Row => {
                self.main_axis_alignment = align;
            },
            FlexDirection::Column => {
                self.cors_axis_alignment = align;
            }
            FlexDirection::RowReverse => {
                self.main_axis_alignment = align;
            },
            FlexDirection::ColumnReverse => {
                self.cors_axis_alignment = align;
            },
        }
        self
    }

    pub fn vertical_alignment(mut self,align:AlignItems)->Self{
        match self.direction{
            FlexDirection::Row => {
                self.cors_axis_alignment = align;
            },
            FlexDirection::Column => {
                self.main_axis_alignment = align;
            }
            FlexDirection::RowReverse => {
                self.cors_axis_alignment = align;
            },
            FlexDirection::ColumnReverse => {
                self.main_axis_alignment = align;
            },
        }
        self
    }

    pub fn self_horizontal_alignment(mut self,align:AlignItems)->Self{
        match self.direction{
            FlexDirection::Row => {
                self.self_main_axis_alignment = align;
            },
            FlexDirection::Column => {
                self.self_cors_axis_alignment = align;
            }
            FlexDirection::RowReverse => {
                self.self_main_axis_alignment = align;
            },
            FlexDirection::ColumnReverse => {
                self.cors_axis_alignment = align;
            },
        }
        self
    }

    pub fn self_vertical_alignment(mut self,align:AlignItems)->Self{
        match self.direction{
            FlexDirection::Row => {
                self.self_cors_axis_alignment = align;
            },
            FlexDirection::Column => {
                self.self_main_axis_alignment = align;
            }
            FlexDirection::RowReverse => {
                self.self_cors_axis_alignment = align;
            },
            FlexDirection::ColumnReverse => {
                self.self_main_axis_alignment = align;
            },
        }
        self
    }

    pub fn element_type(mut self, element_type: ElementType) -> Self{
        self.element_type = element_type;
        self
    }
}

impl UIElement for Element {
    fn draw(&self, painter: &mut ShapePainter) {
        painter.set_color(self.color);
        if self.render_block != UIRenderMode::WithoutSelf
        {
            match self.render_state {
                UIMouseState::Hover  => {
                    if let Some(action) = self.render.hover.clone()
                    {
                        action(painter);
                    }
                }
                UIMouseState::Click => {
                    if let Some(action) = self.render.hover.clone()
                    {
                        action(painter);
                    }
                }
                _ => {}
            }
        }
        
        painter.set_translation(self.position);

        if let Some(shape) = self.shape.as_ref() {
            shape.read().unwrap().draw(painter);
        }

        if self.background_color != Srgba::ZERO 
        {
            painter.set_color(self.color * 0.5);
            painter.rect(self.size);
            painter.set_color(self.background_color * 0.5);
            painter.rect(self.content_size);
        }

        painter.corner_radii = Vec4::ZERO;
        painter.set_rotation(Quat::IDENTITY);
    }

    fn exc(&mut self, context: &mut Context) {
        match self.action_state {
            UIMouseState::Hover => {
                match &self.action.hover {
                    Some(action) => action(context),
                    None => {},
                }
            }
            UIMouseState::Click => {
                match &self.action.click {
                    Some(action) => action(context),
                    None => {}
                }
                self.action_state = UIMouseState::Release;
            }
            UIMouseState::Release => {}
            UIMouseState::DoubleClick => todo!(),
            UIMouseState::Drag => todo!(),
            UIMouseState::NoneBlock => {}
        }
    }

    fn style(&self) -> Style {
        let mut def = Style {
            size: match (self.size.x, self.size.y) {
                (0.,0.) => Size::auto(),
                _ => Size {
                    width: Dimension::Length(self.size.x),
                    height: Dimension::Length(self.size.y),
                }
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
            align_items: match self.cors_axis_alignment {
                AlignItems::Start => Some(taffy::AlignSelf::Start),
                AlignItems::End => Some(taffy::AlignSelf::End),
                AlignItems::FlexStart => Some(taffy::AlignSelf::FlexStart),
                AlignItems::FlexEnd => Some(taffy::AlignSelf::FlexEnd),
                AlignItems::Center => Some(taffy::AlignSelf::Center),
                AlignItems::Baseline => Some(taffy::AlignSelf::Baseline),
                AlignItems::Stretch => Some(taffy::AlignSelf::Stretch),
                AlignItems::NotSet => Some(taffy::AlignSelf::Start),
            },
            justify_content: match self.main_axis_alignment {
                AlignItems::Start => Some(taffy::JustifyContent::Start),
                AlignItems::End => Some(taffy::JustifyContent::End),
                AlignItems::FlexStart => Some(taffy::JustifyContent::FlexStart),
                AlignItems::FlexEnd => Some(taffy::JustifyContent::FlexEnd),
                AlignItems::Center => Some(taffy::JustifyContent::Center),
                AlignItems::Baseline => Some(taffy::JustifyContent::Center),
                AlignItems::Stretch => Some(taffy::JustifyContent::Stretch),
                AlignItems::NotSet => Some(taffy::JustifyContent::Start),
            },
            align_content: match self.cors_axis_alignment {
                AlignItems::Start => Some(taffy::AlignContent::Start),
                AlignItems::End => Some(taffy::AlignContent::End),
                AlignItems::FlexStart => Some(taffy::AlignContent::FlexStart),
                AlignItems::FlexEnd => Some(taffy::AlignContent::FlexEnd),
                AlignItems::Center => Some(taffy::AlignContent::Center),
                AlignItems::Baseline => Some(taffy::AlignContent::Center),
                AlignItems::Stretch => Some(taffy::AlignContent::Stretch),
                AlignItems::NotSet => Some(taffy::AlignContent::Start),
            },
            align_self: match self.self_cors_axis_alignment {
                AlignItems::Start => Some(taffy::AlignSelf::Start),
                AlignItems::End => Some(taffy::AlignSelf::End),
                AlignItems::FlexStart => Some(taffy::AlignSelf::FlexStart),
                AlignItems::FlexEnd => Some(taffy::AlignSelf::FlexEnd),
                AlignItems::Center => Some(taffy::AlignSelf::Center),
                AlignItems::Baseline => Some(taffy::AlignSelf::Baseline),
                AlignItems::Stretch => Some(taffy::AlignSelf::Stretch),
                AlignItems::NotSet => None,
            },
            justify_self: match self.self_main_axis_alignment {
                AlignItems::Start => Some(taffy::AlignSelf::Start),
                AlignItems::End => Some(taffy::AlignSelf::End),
                AlignItems::FlexStart => Some(taffy::AlignSelf::FlexStart),
                AlignItems::FlexEnd => Some(taffy::AlignSelf::FlexEnd),
                AlignItems::Center => Some(taffy::AlignSelf::Center),
                AlignItems::Baseline => Some(taffy::AlignSelf::Baseline),
                AlignItems::Stretch => Some(taffy::AlignSelf::Stretch),
                AlignItems::NotSet => None,
            },
            ..Default::default()
        };
        match self.element_type {
            ElementType::Debug => {
                def = Style{
                    position: Position::Absolute,
                    inset: Rect { left: auto(), right: length(0.0), top: length(0.0), bottom: auto() },
                    justify_self: Some(taffy::JustifySelf::Center),
                    ..def
                }
            },
            ElementType::Content => {},
        }
        def
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

    fn update_layout(&mut self, layout: &taffy::Layout, origin: Vec3, inherit_origin: Vec3) {
        self.size.x = layout.size.width;
        self.size.y = layout.size.height;
        self.content_size.x = layout.content_size.width;
        self.content_size.y = layout.content_size.height;

        self.position = bevy::prelude::Vec3::new(
            origin.x + layout.location.x + self.size.x / 2. + inherit_origin.x,
            origin.y - self.size.y / 2. - layout.location.y - inherit_origin.y,
            0.,
        );
    }
    
    /// update position and insection state
    fn update_state(
        &mut self,
        cursor: (f32, f32),
        origin: Vec3,
    ) {
        let curo_screen = Vec2::new(
            cursor.0 + origin.x,
            cursor.1 - origin.y,
        );

        if cursor.0 < 0. {
            return;
        }

        if self.insection(curo_screen) {
            self.action_state = UIMouseState::Hover;
            self.render_state = UIMouseState::Hover;
        } else {
            self.action_state = UIMouseState::Release;
            self.render_state = UIMouseState::Release;
        }
    }

    fn set_action_state(&mut self, state: UIMouseState) {
        match state {
            UIMouseState::Click => {
                if self.action_state == UIMouseState::Hover {
                    self.action_state = UIMouseState::Click;
                    self.render_state = UIMouseState::Click;
                }
            }
            UIMouseState::Drag => {
                if self.action_state == UIMouseState::Click {
                    self.action_state = UIMouseState::Release;
                    self.render_state = UIMouseState::Release;
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

    fn get_input_state(&mut self) -> UIMouseState {
        self.action_state.clone()
    }

    fn get_render_state(&mut self) -> Option<UIMouseState> {
        match (self.render_state, self.render.hover.is_some()) {
            (UIMouseState::Hover ,true) | (UIMouseState::Click, true) => {
                return  Some(self.render_state.clone());
            },
            // (UIMouse::Release, true) => todo!(),
            // (UIMouse::DoubleClick, true) => todo!(),
            // (UIMouse::Drag, true) => todo!(),
            // (UIMouse::NoneBlock, true) => todo!(),
            _=> { return None; }
        }
    }

    fn set_render_state(&mut self, state: UIMouseState) {
        self.render_state = state;
    }

    fn block_render_state(&mut self) -> UIRenderMode {
        self.render_block
    }
    
    fn get_element_type(&self) -> ElementType {
        self.element_type.clone()
    }
    
    fn get_element(&self) -> Element {
        self.clone()
    }
}
