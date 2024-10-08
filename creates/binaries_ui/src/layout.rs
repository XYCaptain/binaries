use std::{collections::HashMap, sync::RwLockWriteGuard};

use bevy::{color::palettes::{css::{BLACK, GREEN}, tailwind::{GREEN_200, RED_400}}, math::{Vec2, Vec3, Vec4}, prelude::{Commands, Resource}};
use bevy_vector_shapes::prelude::ShapePainter;
use taffy::{
    prelude::TaffyMaxContent, Dimension, JustifyContent, NodeId, Size, Style, TaffyTree, TraversePartialTree
};

use crate::{components::{element::{AlignItems, Element, FlexDirection}, rectangle, text, UIMouseState}, context::MemState, shape::{Curve, ShapeTrait}, Config};

use super::traits::UIElement;

#[derive(Resource)]
pub struct UILayouts {
    elements: HashMap<NodeId, Element>,
    debuge_relations: HashMap<NodeId, NodeId>,
    pub taffy: TaffyTree<()>,
    pub root: NodeId,
    pub debug_root: NodeId
}

impl UILayouts {
    pub fn new() -> Self {
        let mut taffy: TaffyTree<()> = TaffyTree::new();
        let node = taffy
            .new_leaf(
                Style {
                    justify_content: Some(JustifyContent::Start),
                    ..Default::default()
                },
            ).expect("");
        let mut elements = HashMap::new();
        elements.insert(node, Element::new().title("root").background_color(GREEN));
        Self {
            taffy,
            elements: elements,
            root: node,
            debug_root: NodeId::new(0u64),
            debuge_relations: HashMap::new(),
        }
    }
    
    pub fn clear_node(&mut self, node: NodeId){
        let mut nodes_to_remove = Vec::new();
        self.traverse_node(node, &mut nodes_to_remove);
        for node_to_remove in nodes_to_remove{
            self.taffy.remove(node_to_remove).expect("node_to_remove");
        }
    }

    pub fn iter(&mut self) -> impl Iterator<Item = &mut Element> {
        self.elements.values_mut()
    }

    ///WIP:olny one debug-node. Need to Update to multi-debug-nodes
    pub fn push_element(&mut self, element: Element) -> NodeId {
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
        self.elements.insert(child, element);
        self.taffy.compute_layout(self.root, Size::MAX_CONTENT).expect("msg");
        child
    }

    pub fn push_element_with_id(&mut self, element: Element, id: NodeId) -> NodeId {
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
        self.elements.insert(child, element);
        self.taffy.compute_layout(
            self.root,
            Size::MAX_CONTENT
        ).expect("msg");
        child
    }

    pub fn update_shape(&mut self, mut config: Config, mut commands: Commands) {
        for element in self.elements.values_mut() {
            if let Some(shape) = element.shape.as_ref() {
                let mut shape = shape.write().unwrap();
                shape.update(&mut config,&mut commands,element.layout_anchor);
                
            }
        }
    }
    
    pub fn update(&mut self,context: &mut RwLockWriteGuard<MemState>, screen_layout_origin:Vec3) {
        //setup dom tree
        if u64::from(self.debug_root) > 0u64 && self.taffy.child_count(self.debug_root) == 0{
            self.gen_debug_elements_tree();
        }
        
        //setup win size for root
        {
            let old_style = self.taffy.style(self.root).expect("");
            self.taffy.set_style(self.root, Style{
                size:Size {
                    width: Dimension::Length(screen_layout_origin.x * -2.),
                    height: Dimension::Length(screen_layout_origin.y * 2.),
                },
                ..old_style.clone()
            }).expect("msg");
            //println!("root style:{:?}",self.taffy.style(self.root).expect("").size);
        }

        //setup win size for content_node
        {
            let content_node = self.taffy.get_child_id(self.root, 0);
            let old_style = self.taffy.style(content_node).expect("");
            self.taffy.set_style(content_node, Style{
                size:Size {
                    width: Dimension::Length(screen_layout_origin.x * -2.),
                    height: Dimension::Length(screen_layout_origin.y * 2.),
                },
                ..old_style.clone()
            }).expect("msg");
            //println!("root style:{:?}",self.taffy.style(content_node).expect("").size);
        }
        
        self.taffy.compute_layout(self.root, taffy::Size::MAX_CONTENT).expect("");
        self.traverse_update(self.root,screen_layout_origin,Vec3::new(0.,0.,0.), context,None);

        for (element,debug_element) in self.debuge_relations.iter() {
            let render_state = self.elements.get_mut(element).unwrap().get_render_state();
            if render_state.is_some(){
                self.elements.get_mut(debug_element).unwrap().set_render_state(render_state.unwrap());
            }
        }
    }

    pub fn draw(&mut self, painter: &mut ShapePainter) {
        self.traverse_draw(self.root,painter,Vec3::new(0.,0., 0.));
        self.draw_tree(painter);
    }

    pub fn exc_action(&mut self, context: &mut RwLockWriteGuard<MemState>) {
        for element in self.elements.values_mut() {
            element.execute( context);
        }
    }

    fn draw_tree(&mut self, painter: &mut ShapePainter) {
        painter.set_translation(Vec3::ZERO);
        painter.set_color(BLACK);
        for (element_id,debuge_element_id) in self.debuge_relations.iter(){
            let pareant_p = self.elements.get(debuge_element_id).unwrap().layout_anchor;
            for child_element_id in  self.taffy.child_ids(*element_id){
                let child_debug_element_id = self.debuge_relations.get(&child_element_id);
                if child_debug_element_id.is_some(){
                    let child_p = self.elements.get(child_debug_element_id.unwrap()).unwrap().layout_anchor;
                    Curve::new(pareant_p + Vec3::NEG_Z,child_p + Vec3::NEG_Z).draw(painter);
                }
            }
        }
        painter.set_translation(Vec3::ZERO);
    }

    fn traverse_node(&mut self,node: NodeId, nodes_to_remove: &mut Vec::<NodeId>) {
        let children:Vec<NodeId> =  self.taffy.child_ids(node).collect();
        nodes_to_remove.push(node);
        for child in children {
            self.traverse_node(child, nodes_to_remove);
        }
    }
    
    //TODO: needed to optimize
    fn traverse_update(&mut self, node: NodeId,screen_layout_origin:Vec3, inhert_origin:Vec3, cxt:&mut RwLockWriteGuard<MemState>, inherit_render_state: Option<UIMouseState>) {
        let children:Vec<NodeId> =  self.taffy.child_ids(node).collect();
        for child in children.iter() {
            let layout = self.taffy.layout(*child).expect("布局错误");
            let element = self.elements.get_mut(child).unwrap();
            let mut blockstate = None;

            {
                //Update state
                element.update_layout(layout, screen_layout_origin.clone(), inhert_origin,cxt);
                element.update_render_state((cxt.mouse_position.x,cxt.mouse_position.y), screen_layout_origin.clone());
                
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
            let origin_new = Vec3::new(layout.location.x +  element.anchor_offset.x  + element.rubber_offset.x,layout.location.y -  element.anchor_offset.y - element.rubber_offset.y,0.) + inhert_origin;
            self.traverse_update(*child, screen_layout_origin, origin_new, cxt, blockstate);
        }
    }
    
    //TODO: needed to optimize
    fn traverse_draw(&mut self, node: NodeId,painter: &mut ShapePainter, origin:Vec3) {
        let children:Vec<NodeId> =  self.taffy.child_ids(node).collect();
        for child in children.iter() {
            let layout = self.taffy.layout(*child).expect("布局错误");
            let element = self.elements.get_mut(child).unwrap();
            let origin_new = Vec3::new(layout.location.x,layout.location.y,0.) + origin;
            element.draw(painter);
            self.traverse_draw(*child, painter, origin_new);
        }
    }

    pub fn gen_debug_elements_tree(&mut self){
        if u64::from(self.debug_root) > 0{
            self.traverse_gen_debug_element(self.root, self.debug_root);
        }
    }

    // to debug tree
    fn traverse_gen_debug_element(&mut self, node: NodeId, p_node:NodeId) {
        if u64::from(node) == u64::from(self.debug_root)
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
            let v_stack = 
            rectangle()
                .round(5.)
                .color(RED_400)
                .direction(FlexDirection::Column);
            v_node = self.push_element_with_id(v_stack, p_node);
            self_element = self_element.self_horizontal_alignment(AlignItems::Center).self_vertical_alignment(AlignItems::Center);
        }

        let self_node = self.push_element_with_id(self_element, v_node);
        self.debuge_relations.insert(node,self_node);

        let tile = self.elements.get(&node).unwrap().tile.as_str();

        let text_content = text(tile).size(Vec2::new(100., 20.)).self_horizontal_alignment(AlignItems::Center).self_vertical_alignment(AlignItems::Center);
        self.push_element_with_id(text_content, self_node);

        if children.len() ==  0{
            return;
        }
        
        let h2_stack = rectangle()
            .round(5.)
            .color(GREEN_200).horizontal_alignment(AlignItems::Center);
       
        let h2: NodeId = self.push_element_with_id(h2_stack, v_node);

        for child in children.iter() {
            self.traverse_gen_debug_element(*child,h2);
        }
    }

    pub fn print_tree(&mut self) {
        self.taffy.print_tree(self.root);
    }

    pub fn update_input_state(&mut self, state: UIMouseState) {
        for (_, element) in self.elements.iter_mut() {
            element.set_action_state(state.clone());
        }
    }

    pub fn test(&mut self) {
        println!("test");
    }
}