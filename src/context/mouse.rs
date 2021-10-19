#![allow(dead_code)]

use std::collections::HashMap;

pub type MouseButton = glfw::MouseButton;

pub struct Mouse {
    state: HashMap<MouseButton, bool>,
    last_state: HashMap<MouseButton, bool>,
    position: (f32, f32),
}

impl Mouse {
    pub fn new(window: &mut glfw::Window) -> Mouse {
        window.set_mouse_button_polling(true);
        window.set_cursor_pos_polling(true);

        Mouse {
            state: HashMap::new(),
            last_state: HashMap::new(),
            position: (0.0, 0.0),
        }
    }

    pub fn process_event(&mut self, event: &glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::MouseButton(button, action, _) => {
                use glfw::Action;
                let is_pressed = *action == Action::Press || *action == Action::Repeat;

                if self.state.contains_key(&button) {
                    let value = self.state.get_mut(&button).unwrap();
                    *value = is_pressed;
                } else {
                    self.state.insert(*button, is_pressed);
                    self.last_state.insert(*button, is_pressed);
                }
            }
            glfw::WindowEvent::CursorPos(x, y) => {
                self.position = ((*x as f32), (*y as f32));
            }
            _ => {}
        }
    }

    pub fn update(&mut self) {
        for (k, v) in self.state.iter() {
            *(self.last_state.get_mut(k).unwrap()) = *v;
        }
    }

    pub fn get_button(&self, button: MouseButton) -> bool {
        *self.state.get(&button).unwrap_or(&false)
    }

    pub fn get_button_down(&self, button: MouseButton) -> bool {
        let curr = *self.state.get(&button).unwrap_or(&false);
        let last = *self.last_state.get(&button).unwrap_or(&false);
        return curr && curr != last;
    }

    pub fn get_button_up(&self, button: MouseButton) -> bool {
        let curr = *self.state.get(&button).unwrap_or(&false);
        let last = *self.last_state.get(&button).unwrap_or(&false);
        return last && curr != last;
    }

    pub fn position(&self) -> (f32, f32) {
        self.position
    }
}
