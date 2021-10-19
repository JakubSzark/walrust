#![allow(dead_code)]

use std::collections::HashMap;

pub type Key = glfw::Key;

pub struct Keyboard {
    state: HashMap<Key, bool>,
    last_state: HashMap<Key, bool>,
}

impl Keyboard {
    pub fn new(window: &mut glfw::Window) -> Keyboard {
        window.set_key_polling(true);

        Keyboard {
            state: HashMap::new(),
            last_state: HashMap::new(),
        }
    }

    pub fn process_event(&mut self, event: &glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::Key(key, _, action, _) => {
                use glfw::Action;
                let is_pressed = *action == Action::Press || *action == Action::Repeat;

                if self.state.contains_key(&key) {
                    let value = self.state.get_mut(&key).unwrap();
                    *value = is_pressed;
                } else {
                    self.state.insert(*key, is_pressed);
                    self.last_state.insert(*key, false);
                }
            }
            _ => {}
        }
    }

    pub fn update(&mut self) {
        for (k, v) in self.state.iter() {
            *(self.last_state.get_mut(k).unwrap()) = *v;
        }
    }

    pub fn get_key(&self, key: Key) -> bool {
        *self.state.get(&key).unwrap_or(&false)
    }

    pub fn get_key_down(&self, key: Key) -> bool {
        let curr = *self.state.get(&key).unwrap_or(&false);
        let last = *self.last_state.get(&key).unwrap_or(&false);
        return curr && curr != last;
    }

    pub fn get_key_up(&self, key: Key) -> bool {
        let curr = *self.state.get(&key).unwrap_or(&false);
        let last = *self.last_state.get(&key).unwrap_or(&false);
        return last && curr != last;
    }
}
