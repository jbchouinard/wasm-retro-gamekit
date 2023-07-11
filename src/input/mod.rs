use std::collections::HashMap;

pub trait Key {
    fn name(&self) -> String;
    fn value(&self) -> u8;
}

pub struct KeyMap {
    map: HashMap<String, u8>,
    names: HashMap<u8, String>,
}

impl KeyMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            names: HashMap::new(),
        }
    }
    pub fn set_key_mapping<T: Key>(&mut self, key: &str, mapped: T) {
        let n: u8 = mapped.value();
        self.map.insert(key.to_string(), n);
        self.names.insert(n, mapped.name());
    }
    pub fn get(&self, key: &str) -> Option<u8> {
        self.map.get(key).cloned()
    }
}

impl std::fmt::Display for KeyMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, n) in self.map.iter() {
            let name = self.names.get(n).unwrap();
            writeln!(f, "{}:  {}", name, key)?;
        }
        Ok(())
    }
}

pub struct InputState {
    keymap: KeyMap,
    pressed: [bool; 255],
}

impl InputState {
    pub fn new() -> Self {
        Self {
            keymap: KeyMap::new(),
            pressed: [false; 255],
        }
    }
    pub fn register_key_up(&mut self, key: &str) {
        if let Some(x) = self.keymap.get(key) {
            self.pressed[x as usize] = false;
        }
    }
    pub fn register_key_down(&mut self, key: &str) {
        if let Some(x) = self.keymap.get(key) {
            self.pressed[x as usize] = true;
        }
    }
    pub fn keymap(&self) -> &KeyMap {
        &self.keymap
    }
    pub fn set_keymap(&mut self, keymap: KeyMap) {
        self.keymap = keymap;
    }
    pub fn is_pressed<T: Key>(&self, key: T) -> bool {
        let idx: u8 = key.value();
        self.pressed[idx as usize]
    }
}
