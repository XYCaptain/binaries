use std::collections::HashMap;

use bevy::prelude::Resource;
use bevy_vector_shapes::prelude::ShapePainter;
use taffy::{
    prelude::TaffyMaxContent, Dimension, JustifyContent, NodeId, Size, Style, TaffyTree,
};

use super::traits::UIElement;


#[derive(Resource)]
pub struct SDUILayouts {
    hash_elements: HashMap<NodeId, Box<dyn UIElement>>,
    taffy: TaffyTree<()>,
    root: NodeId,
}

// impl UILayout for SDUILayouts {
//     fn push(&mut self, element: Box<dyn UIElement>) {
//         let child = self
//             .taffy
//             .new_leaf(element.style()).unwrap();
//         self.taffy.add_child(self.root, child).unwrap();
//         self.hash_elements.insert(child, element);
//         self.taffy.compute_layout(self.root, Size::MAX_CONTENT).expect("msg");
//     }
// }

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

    pub fn push(&mut self, element: impl UIElement + 'static) {
        let child = self
            .taffy
            .new_leaf(element.style()).unwrap();
        self.taffy.add_child(self.root, child).unwrap();
        self.hash_elements.insert(child, Box::new(element));
        self.taffy.compute_layout(self.root, Size::MAX_CONTENT).expect("msg");
    }
    
    pub fn push_element(&mut self, element: Box<dyn UIElement>) {
        let child = self
            .taffy
            .new_leaf(element.style()).unwrap();
        self.taffy.add_child(self.root, child).unwrap();
        self.hash_elements.insert(child, element);
        self.taffy.compute_layout(self.root, Size::MAX_CONTENT).expect("msg");
    }

    pub fn init(&mut self, painter: &mut ShapePainter) {
        for (nodeid, element) in self.hash_elements.iter_mut() {
            let layout = self.taffy.layout(*nodeid).expect("布局错误");
            println!("{:?}", layout);
            element.update((-100.,-100.), painter, layout);
        }
    }

    pub fn update(&mut self, cursor: (f32, f32), painter: &mut ShapePainter) {
        for (nodeid, element) in self.hash_elements.iter_mut() {
            let layout = self.taffy.layout(*nodeid).expect("布局错误");
            element.update(cursor, painter, layout);
        }
    }

    pub fn draw(&mut self, painter: &mut ShapePainter) {
        for (nodeid, element) in self.hash_elements.iter_mut() {
            if !element.isready() {
                let layout = self.taffy.layout(*nodeid).expect("布局错误");
                element.update((-100.,-100.), painter, layout);
                element.setready();
            }
            element.draw(painter);
        }
    }

    pub fn print_tree(&mut self) {
        self.taffy.print_tree(self.root);
    }
}