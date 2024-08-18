use bevy::{color::palettes::tailwind, math::{Vec2, Vec4}};
use binaries_ui::{components::{element::{self, AlignItems, ElementType}, rectangle, stacks::{hstack, vstack}, text}, traits::UIElement};

pub(crate) fn node_panel() -> impl UIElement {
    let items_h = (
        rectangle().size(Vec2::new(100., 200.)).color(tailwind::RED_100),
        rectangle().size(Vec2::new(100., 50.)).color(tailwind::GREEN_100),
        rectangle().size(Vec2::new(100., 100.)).color(tailwind::BLUE_500),
        rectangle().size(Vec2::new(100., 150.)).color(tailwind::YELLOW_200),
    );
    let items_v = (
        rectangle().size(Vec2::new(400., 50.)).color(tailwind::RED_100),
        rectangle().size(Vec2::new(100., 50.)).color(tailwind::GREEN_100),
        rectangle().size(Vec2::new(200., 50.)).color(tailwind::BLUE_500),
        rectangle().size(Vec2::new(350., 50.)).color(tailwind::YELLOW_200),
    );

    hstack(
        (
            vstack(
                (
                    hstack((
                        text("Vertical-Start").size(Vec2::new(150., 20.)),
                        hstack( items_h.clone()).vertical_alignment(AlignItems::Start),
                    )).margin(Vec4::splat(10.)).vertical_alignment(AlignItems::Center),
                    hstack((
                        text("Vertical-Center").size(Vec2::new(150., 20.)),
                        hstack( items_h.clone()).vertical_alignment(AlignItems::Center),
                    )).margin(Vec4::splat(10.)).vertical_alignment(AlignItems::Center),
                    hstack((
                        text("Vertical-End").size(Vec2::new(150., 20.)),
                        hstack( items_h.clone()).vertical_alignment(AlignItems::End),
                    )).margin(Vec4::splat(10.)).vertical_alignment(AlignItems::Center),
                )
            ),
            vstack(
                (
                    hstack((
                        text("Horizontal-Start").size(Vec2::new(150., 20.)),
                        vstack( items_v.clone()).horizontal_alignment(AlignItems::Start),
                    )).margin(Vec4::splat(10.)).vertical_alignment(AlignItems::Center),
                    hstack((
                        text("Horizontal-Center").size(Vec2::new(150., 20.)),
                        vstack( items_v.clone()).horizontal_alignment(AlignItems::Center),
                    )).margin(Vec4::splat(10.)).vertical_alignment(AlignItems::Center),
                    hstack((
                        text("Horizontal-End").size(Vec2::new(150., 20.)),
                        vstack( items_v.clone()).horizontal_alignment(AlignItems::End),
                    )).margin(Vec4::splat(10.)).vertical_alignment(AlignItems::Center),
                )
            ),
        )
    )
    .horizontal_alignment(AlignItems::Center)
    .vertical_alignment(AlignItems::Center)
}