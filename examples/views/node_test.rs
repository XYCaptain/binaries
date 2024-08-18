use bevy::{
    color::{
        palettes::{css::{BLACK, DARK_GREEN, GRAY, GREEN, WHITE, WHITE_SMOKE, YELLOW}, tailwind::{GRAY_400, GRAY_500, GRAY_600, GRAY_900, GREEN_100, GREEN_200, GREEN_900, RED_900}}, Gray, Srgba
    },
    math::{Vec2, Vec3, Vec4, VectorSpace},
};
use binaries_ui::{
    components::{
        circle, element::ElementType, rectangle, stacks::{hstack, stack, vstack}, text, UIRenderMode
    },
    shape::{Ngon, Rectangle},
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
        )),
        element().element_type(element::ElementType::Debug),
    ))
}

pub(crate) fn node_panel() -> impl UIElement {
    hstack(
        (hstack(
            vstack
                (
                    (
                        header("header"),
                        vstack(
                            (
                                lable_input("title1"),
                                lable_input("title2"),
                                lable_output("title3"),
                                lable_output("title4"),
                            )
                        ),
                    )
                )
                .horizontal_alignment(element::AlignItems::Center)
                .background_color(GRAY_400).round(5.)
                .title("panel")
        ).vertical_alignment(element::AlignItems::Center).margin(Vec4::splat(10.)),
        rectangle().color(GREEN).element_type(ElementType::Debug)
    )
    )
   .title("view")
}

fn lable_input(content:&str) -> impl UIElement + Clone {
    hstack(
        (
            circle(5.).color(GREEN_200).self_vertical_alignment(element::AlignItems::Center),
            hstack(
                text(content)
                    .size(Vec2::new(150., 20.))
                    .margin(Vec4::new(20., 2., 20., 2.))
                    .round(5.).background_color(GRAY)
            )
        )
    )
   .background_color(GRAY_500)
}

fn lable_output(content:&str) -> impl UIElement + Clone {
    hstack(
        (
            hstack(
                text(content)
                    .size(Vec2::new(150., 20.))
                    .margin(Vec4::new(20., 2., 20., 2.))
                    .round(5.).background_color(GRAY)
            ),
            circle(5.).color(GREEN_200).self_vertical_alignment(element::AlignItems::Center)
        )
    )
   .background_color(GRAY_500)
}

fn header(content:&str) -> impl UIElement + Clone {
    hstack(
        text(content).size(Vec2::new(190., 20.))
    )
    .title("header")
    .background_color(DARK_GREEN)
    .round(5.)
}