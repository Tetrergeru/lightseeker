use std::collections::HashSet;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum ControlKey {
    Forward,
    Back,
    Right,
    Left,
    Jump,
    Crouch,
    Extra1,
    Extra2,
    Extra3,
    Extra4,
    Extra5,
    Extra6,
    Unknown,
}

impl ControlKey {
    pub fn from_string(str: &str) -> Self {
        use ControlKey::*;
        match str {
            "KeyW" => Forward,
            "KeyS" => Back,
            "KeyA" => Left,
            "KeyD" => Right,
            "ShiftLeft" => Crouch,
            "Space" => Jump,
            "ArrowDown" => Extra1,
            "ArrowUp" => Extra2,
            "ArrowLeft" => Extra3,
            "ArrowRight" => Extra4,
            "Digit1" => Extra5,
            "Digit2" => Extra6,
            _ => Unknown,
        }
    }
}

pub struct Controls {
    keys_down: HashSet<ControlKey>,
}

impl Controls {
    pub fn new() -> Self {
        Self {
            keys_down: HashSet::new(),
        }
    }

    pub fn keys_down(&self) -> impl Iterator<Item=ControlKey> + '_ {
        self.keys_down.iter().cloned()
    }

    pub fn down(&mut self, key: ControlKey) {
        self.keys_down.insert(key);
    }

    pub fn up(&mut self, key: ControlKey) {
        self.keys_down.remove(&key);
    }
}

impl Default for Controls {
    fn default() -> Self {
        Self::new()
    }
}
