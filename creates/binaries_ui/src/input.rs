use bevy::{input::{gestures::{DoubleTapGesture, PinchGesture, RotationGesture}, mouse::{MouseButtonInput, MouseMotion, MouseWheel}}, log::info, prelude::EventReader, window::CursorMoved};

pub fn print_mouse_events_system(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut pinch_gesture_events: EventReader<PinchGesture>,
    mut rotation_gesture_events: EventReader<RotationGesture>,
    mut double_tap_gesture_events: EventReader<DoubleTapGesture>,
) {
    for event in mouse_button_input_events.read() {
        info!("{:?}", event);
    }

    for event in mouse_motion_events.read() {
        info!("{:?}", event);
    }

    for event in cursor_moved_events.read() {
        info!("{:?}", event);
    }

    for event in mouse_wheel_events.read() {
        info!("{:?}", event);
    }

    // This event will only fire on macOS
    for event in pinch_gesture_events.read() {
        info!("{:?}", event);
    }

    // This event will only fire on macOS
    for event in rotation_gesture_events.read() {
        info!("{:?}", event);
    }

    // This event will only fire on macOS
    for event in double_tap_gesture_events.read() {
        info!("{:?}", event);
    }
}