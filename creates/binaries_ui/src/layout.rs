use std::{collections::HashMap, sync::{Arc, RwLock}};

use bevy::{math::Vec3, prelude::Resource};
use bevy_vector_shapes::prelude::ShapePainter;
use taffy::{
    prelude::TaffyMaxContent, Dimension, JustifyContent, NodeId, Size, Style, TaffyTree, TraversePartialTree
};

use crate::components::UIMouse;

use super::traits::UIElement;

#[derive(Clone,Resource)]
pub struct Context(Arc<RwLock<SDUILayouts>>);

impl Default for Context {
    fn default() -> Self {
        Self(Arc::new(RwLock::new(SDUILayouts::new())))
    }
}

#[derive(Resource)]
pub struct SDUILayouts {
    hash_elements: HashMap<NodeId, Box<dyn UIElement>>,
    taffy: TaffyTree<()>,
    root: NodeId,
}

impl SDUILayouts {
    pub fn new() -> Self {
        let mut taffy: TaffyTree<()> = TaffyTree::new();
        taffy.disable_rounding();
        let node = taffy
            .new_with_children(
                Style {
                    size: Size {
                        width: Dimension::Auto,
                        height: Dimension::Auto,
                    },
                    justify_content: Some(JustifyContent::Start),
                    ..Default::default()
                },
                &[],
            ).unwrap();
        Self {
            taffy: taffy,
            hash_elements: HashMap::new(),
            root: node,
        }
    }

    pub fn iter(&mut self) -> impl Iterator<Item = &mut Box<dyn UIElement>> {
        self.hash_elements.values_mut()
    }

    pub fn push(&mut self, element: impl UIElement + 'static) -> NodeId {
        let child = self
            .taffy
            .new_leaf(element.style()).unwrap();
        self.taffy.add_child(self.root, child).unwrap();
        self.hash_elements.insert(child, Box::new(element));
        self.taffy.compute_layout(self.root, Size::MAX_CONTENT).expect("msg");
        child
    }
    
    pub fn push_element(&mut self, element: Box<dyn UIElement>) -> NodeId {
        let child = self
            .taffy
            .new_leaf(element.style()).unwrap();
        self.taffy.add_child(self.root, child).unwrap();
        self.hash_elements.insert(child, element);
        self.taffy.compute_layout(self.root, Size::MAX_CONTENT).expect("msg");
        child
    }

    pub fn push_element_with_id(&mut self, element: Box<dyn UIElement>, id: NodeId) -> NodeId {
        let child = self
            .taffy
            .new_leaf(element.style()).unwrap();
        self.taffy.add_child(id, child).unwrap();
        self.hash_elements.insert(child, element);
        self.taffy.compute_layout(
            self.root,
            Size::MAX_CONTENT
        ).expect("msg");
        child
    }

    pub fn init(&mut self, painter: &mut ShapePainter) {
        self.traverse_init(self.root,painter,Vec3::new(0.,0.,0.), (-100.,-100.));
    }

    pub fn update(&mut self, cursor: (f32, f32), painter: &mut ShapePainter) {
        self.traverse_update(self.root,painter,Vec3::new(0.,0.,0.), cursor,None);
    }

    pub fn draw(&mut self, painter: &mut ShapePainter) {
        self.traverse_draw(self.root,painter,Vec3::new(0.,0.,0.));
    }

    //TODO: needed to optimize
    fn traverse_init(&mut self, node: NodeId,painter: &mut ShapePainter, origin:Vec3, cursor: (f32, f32)) {
        let children:Vec<NodeId> =  self.taffy.child_ids(node).collect();
        for child in children.iter() {
            let layout = self.taffy.layout(*child).expect("布局错误");
            let element = self.hash_elements.get_mut(child).unwrap();
            
            element.update(cursor, painter, layout, origin);

            let origin_new = Vec3::new(layout.location.x,layout.location.y,0.) + origin;
            self.traverse_init(*child, painter, origin_new, cursor);
        }
    }
    
    //TODO: needed to optimize
    fn traverse_update(&mut self, node: NodeId,painter: &mut ShapePainter, origin:Vec3, cursor: (f32, f32), render_state: Option<UIMouse>) {
        let children:Vec<NodeId> =  self.taffy.child_ids(node).collect();
        for child in children.iter() {
            let layout = self.taffy.layout(*child).expect("布局错误");
            let element = self.hash_elements.get_mut(child).unwrap();
            let mut blockstate = None;

            if element.get_input_state() != UIMouse::NoneBlock  {
                
                element.update(cursor, painter, layout, origin);
                if render_state.is_some()
                {
                    element.set_render_state(render_state.unwrap());
                    element.set_input_state(UIMouse::Release);
                }

                if element.block_render_state()
                {
                    blockstate = Some(element.get_render_state());
                }
            }
            
            // println!("layout: {:?} {:?}", layout.location,origin);
            let origin_new = Vec3::new(layout.location.x,layout.location.y,0.) + origin;
            self.traverse_update(*child, painter, origin_new, cursor, blockstate);

        }
    }
    
    //TODO: needed to optimize
    fn traverse_draw(&mut self, node: NodeId,painter: &mut ShapePainter, origin:Vec3) {
        let children:Vec<NodeId> =  self.taffy.child_ids(node).collect();
        for child in children.iter() {
            let layout = self.taffy.layout(*child).expect("布局错误");
            let element = self.hash_elements.get_mut(child).unwrap();
            
            element.update((-100.,-100.), painter, layout, origin);
            let origin_new = Vec3::new(layout.location.x,layout.location.y,0.) + origin;
            element.draw(painter);
            self.traverse_draw(*child, painter, origin_new);
        }
    }
    
    pub fn print_tree(&mut self) {
        self.taffy.print_tree(self.root);
    }

    pub fn update_input_state(&mut self, state: UIMouse) {
        for (_, element) in self.hash_elements.iter_mut() {
            element.set_input_state(state.clone());
        }
    }

    pub fn test(&mut self) {
        println!("test");
    }
}