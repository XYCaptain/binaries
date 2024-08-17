use bevy::{
    color::{
        palettes::{css::{GREEN, WHITE_SMOKE}, tailwind::{GRAY_400, GRAY_900, GREEN_900, RED_900}},
        Srgba,
    },
    math::{Vec2, Vec4},
};
use binaries_ui::{
    components::{
        element::ElementType, rectangle, stacks::{hstack, stack, vstack}, text, UIRenderMode
    },
    shape::Ngon,
    traits::UIElement,
};

use binaries_ui::components::element;

pub(crate) fn node_test_view() -> impl UIElement {
    let node_element = vstack((hstack(
        element()
            .title("ngon")
            .size(Vec2::new(60., 60.))
            .color(WHITE_SMOKE)
            .shape(Ngon {
                round: Vec4::splat(10.0),
                sides: 3.,
                radius: 12.,
                rotation: -90.,
            }),
    )
    .size(Vec2::new(60., 60.))
    .title("hstack")
    .render_mode(UIRenderMode::Group)
    .round(30.)
    .margin(Vec4::splat(10.))
    .color(GREEN),))
    .title("stack")
    .margin(Vec4::splat(10.))
    .size(Vec2::new(240., 80.))
    .round(40.)
    .color(Srgba::new(0.8, 0.8, 0.8, 0.8));

    hstack((
        vstack((
            node_element.clone(),
            node_element.clone(),
            hstack(vec![node_element.clone(), node_element.clone()]),
            hstack(vec![
                stack(vec![node_element.clone(), node_element.clone()]),
                hstack(vec![node_element.clone(), node_element.clone()]),
            ]),
        ))
        .horizontal_alignment(element::AlignContent::Center),
        element().element_type(element::ElementType::Debug),
    ))
}

pub(crate) fn node_panel() -> impl UIElement {
    hstack(
        (vstack
            (
                (
                    header("header"),
                    vstack(
                        (
                            lable("title1"),
                            lable("title2"),
                            lable("title3"),
                            lable("title4"),
                        )
                    ).color(GRAY_900).title("panel").margin(Vec4::splat(2.)),
                )
            ),
            rectangle().color(GREEN).element_type(ElementType::Debug)
        )
    ).title("view")
}

fn lable(content:&str) -> impl UIElement + Clone {
    hstack(
        text(content).size(Vec2::new(200., 20.))
    )
    .color(GRAY_400).round(5.).margin(Vec4::splat(2.))
}

fn header(content:&str) -> impl UIElement + Clone {
    hstack(
        text(content).size(Vec2::new(200., 20.))
    ).color(GREEN_900).title("header")
}