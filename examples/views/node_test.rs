use bevy::{color::{palettes::css::{GREEN, WHITE_SMOKE}, Srgba}, math::{Vec2, Vec4}};
use binaries_ui::{components::{element, stacks::{stack, hstack, vstack}, UIRenderMode}, shape::Ngon, traits::UIElement};

pub fn node_test_view() -> impl UIElement
{
    let node_element = 
    vstack((
                hstack(
                    element()
                            .title("ngon")
                            .size(Vec2::new(60., 60.))
                            .color(WHITE_SMOKE)
                            .shape(Ngon {
                                round: Vec4::splat(10.0),
                                sides: 3.,
                                radius: 12.,
                                rotation: -90.,
                            })
                )
                .size(Vec2::new(60., 60.))
                .title("hstack")
                .render_mode(UIRenderMode::Group)
                .round(30.)
                .margin(Vec4::splat(10.))
                .color(GREEN),
        )
    )
    .title("stack")
    .margin(Vec4::splat(10.))
    .size(Vec2::new(240., 80.))
    .round(40.)
    .color(Srgba::new(0.8, 0.8, 0.8,0.8));

    hstack(
        (vstack((
                node_element.clone(),
                node_element.clone(),
                hstack(vec![
                    node_element.clone(),
                    node_element.clone(),
                ]),
                hstack(vec![
                    stack(vec![
                        node_element.clone(),
                        node_element.clone(),
                    ]),
                    hstack(vec![
                        node_element.clone(),
                        node_element.clone(),
                    ]),
                ]),
            ))
            .horizontal_alignment(element::AlignContent::Center),
            element().element_type(element::ElementType::Debug),
        )
    )
}