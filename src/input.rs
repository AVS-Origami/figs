use minifb::Key;

pub struct InputManager {
    pub keys: Vec<Key>,
}

impl InputManager {
    /// Construct a new InputManager.
    pub fn new() -> Self {
        InputManager {
            keys: vec![],
        }
    }

    /// Add `key` to the list of keys to check for.
    pub fn add(&mut self, key: Key) {
        self.keys.push(key);
    }

    /// Shortcut to register WASD as keybinds.
    pub fn wasd(&mut self) {
        self.keys.push(Key::W);
        self.keys.push(Key::A);
        self.keys.push(Key::S);
        self.keys.push(Key::D);
    }

    /// Shortcut to register arrow keys as keybinds.
    pub fn arrows(&mut self) {
        self.keys.push(Key::Up);
        self.keys.push(Key::Left);
        self.keys.push(Key::Right);
        self.keys.push(Key::Down);
    }
}