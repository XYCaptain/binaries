use bevy::{ input::{keyboard::KeyboardInput, mouse::{MouseButtonInput, MouseMotion}, ButtonState}, log::info, math::{Vec2, Vec3}, prelude::{Commands, EventReader, KeyCode, MouseButton, ResMut}, window::CursorMoved};
use bevy_vector_shapes::prelude::ShapePainter;

use crate::{components::UIMouseState, layout::UILayouts, Config};

pub fn logic_loop_system(
    mut painter: ShapePainter,
    mut layouts: ResMut<UILayouts>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut keyboard_input_evets: EventReader<KeyboardInput>,
    // mut mouse_wheel_events: EventReader<MouseWheel>,
    // mut pinch_gesture_events: EventReader<PinchGesture>,
    // mut rotation_gesture_events: EventReader<RotationGesture>,
    // mut double_tap_gesture_events: EventReader<DoubleTapGesture>,
    config: Config,
    commands: Commands,
) {
    let window = config.window.get_single().unwrap();
    let binding = config.context.storage();
    let mut cxt = binding.write().unwrap();

    painter.origin = Some(Vec3::new(-window.width() * 0.5, window.height() * 0.5, 0.));
    painter.set_2d();
    if painter.origin.is_none() {
        return;
    }

    cxt.mouse_delta = Vec2::ZERO;
    for event in cursor_moved_events.read() {
        cxt.mouse_position = event.position;
        match event.delta {
            Some(delta) => {
                cxt.mouse_delta = delta;
            },
            None => {
                cxt.mouse_delta = Vec2::ZERO;
            },
        }
    }

    for event in mouse_button_input_events.read() {
        match event {
            MouseButtonInput {
                button: MouseButton::Left,
                state: ButtonState::Pressed,
                ..
            } => {
                layouts.update_input_state(UIMouseState::Pressed);
                cxt.drag_delta.0 = cxt.mouse_position.clone();
                cxt.drag_delta.1 = cxt.mouse_position.clone();
                cxt.mouse_state = ButtonState::Pressed;
            }
            MouseButtonInput {
                button: MouseButton::Left,
                state: ButtonState::Released,
                ..
            } => {
                layouts.update_input_state(UIMouseState::Release);
                cxt.drag_delta.2 = cxt.drag_delta.1 - cxt.drag_delta.0;
                cxt.drag_delta.0 = cxt.mouse_position.clone();
                cxt.drag_delta.1 = cxt.mouse_position.clone(); 
                cxt.mouse_state = ButtonState::Released;
            }
            _ => {}
         }
    }

    for event in keyboard_input_evets.read() {
        match event {
            KeyboardInput{
                key_code:KeyCode::KeyM,
                state:ButtonState::Pressed,
                ..
            }=>{
                println!("ctx {:?}", cxt.user_input);
            }
            KeyboardInput{
                key_code:KeyCode::Escape,
                state:ButtonState::Pressed,
                ..
            }=>{
                cxt.user_input = UIMouseState::Release;
                println!("ctx {:?}", cxt.user_input);
            }
            _=>{}
        }
    }

    for event in mouse_motion_events.read() {
        info!("{:?} {:?}", event,cxt.mouse_position);
        match cxt.mouse_state {
            ButtonState::Pressed => {
                cxt.drag_delta.1 = cxt.mouse_position;
            },
            ButtonState::Released => {
                // cxt.drag_delta.0 = Vec2::ZERO;
                // cxt.drag_delta.1 = Vec2::ZERO;
            },
        }
    }

    layouts.update(&mut cxt, painter.origin.unwrap());
    layouts.update_shape(config, commands);

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
    layouts.exc_action(&mut cxt);
}