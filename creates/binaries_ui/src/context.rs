use std::sync::{Arc, RwLock};

use bevy::{input::ButtonState, math::Vec2, prelude::Resource, utils::HashMap};
use idgenerator::{IdGeneratorOptions, IdInstance};

use crate::{components::UIMouseState, storage::Node};



#[derive(Clone,Resource)]
pub struct Context(Arc<RwLock<MemState>>);

pub struct MemState{
    pub node: HashMap<i64,Node>,
    pub mouse_state: ButtonState,
    pub user_input: UIMouseState,
    pub selection_group: Vec::<i64>,
    pub selection_current: i64,
    pub drag_delta: (Vec2,Vec2,Vec2),
    pub mouse_position:Vec2,
    pub mouse_delta: Vec2,
}

impl Default for Context {
    fn default() -> Self {
        let options = IdGeneratorOptions::new().worker_id(1).worker_id_bit_len(6);
        let _ = IdInstance::init(options).expect("IdInstance init error");

        Self(Arc::new(RwLock::new(MemState{
            node: HashMap::new(),
            mouse_state: ButtonState::Released,
            user_input: UIMouseState::Release,
            selection_group: Vec::new(),
            selection_current: -1,
            drag_delta: (Vec2::ZERO,Vec2::ZERO,Vec2::ZERO),
            mouse_position: Vec2::ONE * -100.,
            mouse_delta: Vec2::ZERO,
        })))
    }
}

impl Context {
    pub fn storage(&self) -> Arc<RwLock<MemState>>{
        self.0.clone()
    }
}