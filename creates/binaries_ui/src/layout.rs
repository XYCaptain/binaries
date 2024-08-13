use std::{collections::HashMap, sync::{Arc, RwLock}};

use bevy::{color::palettes::{css::BLACK, tailwind::{GREEN_200, RED_400}}, math::{Vec2, Vec3, Vec4}, prelude::Resource};
use bevy_vector_shapes::prelude::ShapePainter;
use taffy::{
    prelude::TaffyMaxContent, Dimension, JustifyContent, NodeId, Size, Style, TaffyTree, TraversePartialTree
};

use crate::components::{element::{AlignItems, FlexDirection}, rectangle, UIMouseState};

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
    pub taffy: TaffyTree<()>,
    pub root: NodeId,
    pub debug_root: NodeId
}

impl SDUILayouts {
    pub fn new() -> Self {
        let mut taffy: TaffyTree<()> = TaffyTree::new();
        let node = taffy
            .new_leaf(
                Style {
                    justify_content: Some(JustifyContent::Start),
                    ..Default::default()
                },
            ).expect("");
        Self {
            taffy,
            hash_elements: HashMap::new(),
            root: node,
            debug_root: NodeId::new(0u64)
        }
    }
    
    pub fn clear_node(&mut self, node: NodeId)
    {
        let mut nodes_to_remove = Vec::new();
        self.traverse_node(node, &mut nodes_to_remove);
        for node_to_remove in nodes_to_remove{
            self.taffy.remove(node_to_remove).expect("node_to_remove");
        }
    }

    pub fn iter(&mut self) -> impl Iterator<Item = &mut Box<dyn UIElement>> {
        self.hash_elements.values_mut()
    }

    // pub fn push(&mut self, element: impl UIElement + 'static) -> NodeId {
    //     let child = self
    //         .taffy
    //         .new_leaf(element.style()).unwrap();
    //     self.taffy.add_child(self.root, child).unwrap();
    //     self.hash_elements.insert(child, Box::new(element));
    //     self.taffy.compute_layout(self.root, Size::MAX_CONTENT).expect("msg");
    //     child
    // }

    ///WIP:olny one debug-node. Need to Update to multi-debug-nodes
    pub fn push_element(&mut self, element: Box<dyn UIElement>) -> NodeId {
        let child = self
            .taffy
            .new_leaf(element.style()).unwrap();
        match element.get_element_type() {
            crate::components::element::ElementType::Content => {},
            crate::components::element::ElementType::Debug => {
                self.debug_root = child
            },
        }
        self.taffy.add_child(self.root, child).unwrap();
        self.hash_elements.insert(child, element);
        self.taffy.compute_layout(self.root, Size::MAX_CONTENT).expect("msg");
        child
    }

    pub fn push_element_with_id(&mut self, element: Box<dyn UIElement>, id: NodeId) -> NodeId {
        let child = self
            .taffy
            .new_leaf(element.style()).unwrap();
        match element.get_element_type() {
            crate::components::element::ElementType::Content => {},
            crate::components::element::ElementType::Debug => {
                self.debug_root = child
            },
        }
        self.taffy.add_child(id, child).unwrap();
        self.hash_elements.insert(child, element);
        self.taffy.compute_layout(
            self.root,
            Size::MAX_CONTENT
        ).expect("msg");
        child
    }

    pub fn init(&mut self, painter: &mut ShapePainter) {
        self.gen_debug_tree();
        self.taffy.compute_layout(self.root, taffy::Size::MAX_CONTENT).expect("");
        self.traverse_init(self.root,painter,Vec3::new(0.,0.,0.), (-100.,-100.));
    }
    
    pub fn update(&mut self, cursor: (f32, f32), painter: &mut ShapePainter) {
        //setup win size
        {
            let old_style = self.taffy.style(self.root).expect("");
            self.taffy.set_style(self.root, Style{
                size:Size {
                    width: Dimension::Length(painter.origin.unwrap().x * -2.),
                    height: Dimension::Length(painter.origin.unwrap().y * 2.),
                },
                ..old_style.clone()
            }).expect("msg");
        }

        {
            let content_node = self.taffy.get_child_id(self.root, 0);
            let old_style = self.taffy.style(self.root).expect("");
            self.taffy.set_style(content_node, Style{
                size:Size {
                    width: Dimension::Length(painter.origin.unwrap().x * -2.),
                    height: Dimension::Length(painter.origin.unwrap().y * 2.),
                },
                ..old_style.clone()
            }).expect("msg");
        }
        self.taffy.compute_layout(self.root, taffy::Size::MAX_CONTENT).expect("");
        self.traverse_update(self.root,painter,Vec3::new(0.,0.,0.), cursor,None);
    }

    pub fn draw(&mut self, painter: &mut ShapePainter) {
        self.traverse_draw(self.root,painter,Vec3::new(0.,0.,0.));
    }

    fn traverse_node(&mut self,node: NodeId, nodes_to_remove: &mut Vec::<NodeId>)
    {
        let children:Vec<NodeId> =  self.taffy.child_ids(node).collect();
        nodes_to_remove.push(node);
        for child in children {
            self.traverse_node(child, nodes_to_remove);
        }
    }
    
    //TODO: needed to optimize
    fn traverse_init(&mut self, node: NodeId,painter: &mut ShapePainter, origin:Vec3, cursor: (f32, f32)) {
        let children:Vec<NodeId> =  self.taffy.child_ids(node).collect();
        for child in children.iter() {
            let layout = self.taffy.layout(*child).expect("布局错误");
            let element = self.hash_elements.get_mut(child).unwrap();
            
            element.update(cursor, painter.origin.unwrap().clone(), layout, origin);

            let origin_new = Vec3::new(layout.location.x,layout.location.y,0.) + origin;
            self.traverse_init(*child, painter, origin_new, cursor);
        }
    }
    
    //TODO: needed to optimize
    fn traverse_update(&mut self, node: NodeId,painter: &mut ShapePainter, origin:Vec3, cursor: (f32, f32), inherit_render_state: Option<UIMouseState>) {
        let children:Vec<NodeId> =  self.taffy.child_ids(node).collect();
        for child in children.iter() {
            let layout = self.taffy.layout(*child).expect("布局错误");
            let element = self.hash_elements.get_mut(child).unwrap();
            let mut blockstate = None;

            {
                //Update state
                element.update(cursor, painter.origin.unwrap().clone(), layout, origin);
                
                if inherit_render_state.is_some()
                {
                    // inherit render state
                    element.set_render_state(inherit_render_state.unwrap());
                    // blocak input action
                    element.set_action_state(UIMouseState::Release);
                }
                else {
                    // render as group
                    match  element.block_render_state() {
                        crate::components::UIRenderMode::Group => {
                            blockstate = element.get_render_state();
                        }
                        _ => {
                            blockstate = None
                        }
                    }
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
            element.update((-100.,-100.), painter.origin.unwrap().clone(), layout, origin);
            let origin_new = Vec3::new(layout.location.x,layout.location.y,0.) + origin;
            element.draw(painter);
            self.traverse_draw(*child, painter, origin_new);
        }
    }

    pub fn gen_debug_tree(&mut self){
        if u64::from(self.debug_root) > 0{
            self.traverse_graph_layout(self.root, self.debug_root);
        }
    }

    // to debug tree
    fn traverse_graph_layout(&mut self, node: NodeId, p_node:NodeId) {
        if  u64::from(node) == u64::from(self.debug_root)
        {
            return;
        }
        let children:Vec<NodeId> =  self.taffy.child_ids(node).collect();
        let mut v_node = p_node;

        let mut self_element = rectangle()
            .round(5.).size(Vec2::new(100., 50.))
            .margin(Vec4::new(10.,10.,10.,10.))
            .color(BLACK);

        if children.len() >  0 {
            let v_stack = rectangle()
                .round(5.)
                .color(RED_400)
                .direction(FlexDirection::Column);
            v_node = self.push_element_with_id(Box::new(v_stack), p_node);
            self_element = self_element.horizontal_alignment(AlignItems::Center);
        }

        self.push_element_with_id(Box::new(self_element), v_node);

        if children.len() ==  0{
            return;
        }
        
        let h2_stack = rectangle()
            .round(5.)
            .color(GREEN_200).horizontal_alignment(AlignItems::Center);
       
        let h2: NodeId = self.push_element_with_id(Box::new(h2_stack), v_node);

        for child in children.iter() {
            self.traverse_graph_layout(*child,h2);
        }
    }



    pub fn print_tree(&mut self) {
        self.taffy.print_tree(self.root);
    }

    pub fn update_input_state(&mut self, state: UIMouseState) {
        for (_, element) in self.hash_elements.iter_mut() {
            element.set_action_state(state.clone());
        }
    }

    pub fn test(&mut self) {
        println!("test");
    }
}