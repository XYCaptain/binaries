use bevy::{asset::Assets, input::{gestures::{DoubleTapGesture, PinchGesture, RotationGesture}, mouse::{MouseButtonInput, MouseMotion, MouseWheel}, ButtonState}, log::{info, tracing_subscriber::reload::Handle}, math::Vec3, prelude::{Commands, EventReader, Mesh, MouseButton, Query, ResMut, With}, window::{CursorMoved, PrimaryWindow, Window}};
use bevy_vector_shapes::prelude::ShapePainter;

use crate::{components::UIMouseState, layout::{Context, UILayouts}, text::Config};

pub fn print_mouse_events_system(
    mut painter: ShapePainter,
    mut layouts: ResMut<UILayouts>,
    mut context: ResMut<Context>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    config: Config,
    commands: Commands
    // mut mouse_motion_events: EventReader<MouseMotion>,
    // mut mouse_wheel_events: EventReader<MouseWheel>,
    // mut pinch_gesture_events: EventReader<PinchGesture>,
    // mut rotation_gesture_events: EventReader<RotationGesture>,
    // mut double_tap_gesture_events: EventReader<DoubleTapGesture>,
) {
    let window = window_query.get_single().unwrap();
    painter.origin = Some(Vec3::new(-window.width() * 0.5, window.height() * 0.5, 0.));
    painter.set_2d();
    if painter.origin.is_none() {
        return;
    }

    layouts.update((-100., -100.), &mut painter);
    layouts.update_shape(config, commands);
    
    for event in cursor_moved_events.read() {
        //todo: update state
        layouts.update((event.position.x, event.position.y), &mut painter);
    }

    for event in mouse_button_input_events.read() {
        match event {
            MouseButtonInput {
                button: MouseButton::Left,
                state: ButtonState::Pressed,
                ..
            } => {
                layouts.update_input_state(UIMouseState::Click);
            }
            _ => {}
         }
    }

    // for event in mouse_motion_events.read() {
    //     // info!("{:?}", event);
    // }

    // for event in mouse_wheel_events.read() {
    //     // info!("{:?}", event);
    // }

    // // This event will only fire on macOS
    // for event in pinch_gesture_events.read() {
    //     info!("{:?}", event);
    // }

    // // This event will only fire on macOS
    // for event in rotation_gesture_events.read() {
    //     // info!("{:?}", event);
    // }

    // This event will only fire on macOS
    // for event in double_tap_gesture_events.read() {
    //     // layouts.update_input_state(UIMouse::Click);
    // }

    layouts.draw(&mut painter);
    layouts.exc_action(&mut context);
}