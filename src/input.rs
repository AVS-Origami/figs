use std::collections::HashMap;

use minifb::Key;

pub struct InputManager<'a> {
    pub keybinds: HashMap<Key, Box<dyn FnMut() + 'a>>,
}

impl<'a> InputManager<'a> {
    pub fn new() -> Self {
        InputManager {
            keybinds: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: Key, callback: Box<dyn FnMut() + 'a>) {
        self.keybinds.insert(key, callback);
    }
}