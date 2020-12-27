use crate::math::vec::Vec2;
use glfw::{Key, Action, MouseButton};
use crate::misc::camera::Camera;
use crate::misc::camera::Camera_Movement::*;

pub struct input_state {
    pub left_mouse_button_down: bool,
    pub mouse_position: Vec2,
    pub forward_key_down: bool,
    pub back_key_down: bool,
    pub left_key_down: bool,
    pub right_key_down: bool,
    pub esc_button_down: bool
}
impl input_state {
    pub fn default() -> input_state {
        return input_state{
            left_mouse_button_down: false,
            mouse_position: Vec2::default(),
            forward_key_down: false,
            back_key_down: false,
            left_key_down: false,
            right_key_down: false,
            esc_button_down: false
        }
    }
}

pub fn calculate_input(window: &mut glfw::Window) -> input_state {
    let mut state: input_state = input_state::default();
    state.forward_key_down =  window.get_key(Key::W) == Action::Press;
    state.back_key_down =  window.get_key(Key::S) == Action::Press;
    state.left_key_down =  window.get_key(Key::A) == Action::Press;
    state.right_key_down =  window.get_key(Key::D) == Action::Press;
    state.esc_button_down =  window.get_key(Key::Escape) == Action::Press;
    state.left_mouse_button_down = window.get_mouse_button(MouseButton::Button1) == Action::Press;
    state.mouse_position =  Vec2::new_tuple_f64(window.get_cursor_pos());
    state
}

pub unsafe fn process_input(window: &mut glfw::Window, delta_time: f32, camera: &mut Camera, state:input_state) {
    if state.esc_button_down {
        window.set_should_close(true)
    }
    if state.forward_key_down {
        camera.ProcessKeyboard(FORWARD, delta_time);
    }
    if state.back_key_down {
        camera.ProcessKeyboard(BACKWARD, delta_time);
    }
    if state.left_key_down {
        camera.ProcessKeyboard(LEFT, delta_time);
    }
    if state.right_key_down {
        camera.ProcessKeyboard( RIGHT, delta_time);
    }

    static mut prev_frame_mouse_button_down: bool = false;
    static mut old_mouse_position:Vec2 = Vec2::default();
    if(state.left_mouse_button_down) {
        if(prev_frame_mouse_button_down) {
            let delta = Vec2::new_tuple_f32((state.mouse_position.x  - old_mouse_position.x, -1.0 * (state.mouse_position.y - old_mouse_position.y)));
            camera.ProcessMouseMovement(delta.x, delta.y, false);
        }
        old_mouse_position = state.mouse_position;
    }
    prev_frame_mouse_button_down = state.left_mouse_button_down;
}

