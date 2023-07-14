use std::collections::HashMap;
use std::fmt::Debug;

pub trait Key {
    fn name(&self) -> String;
    fn value(&self) -> u8;
}

impl<T> Key for T
where
    T: Debug + Clone + Into<u8>,
{
    fn name(&self) -> String {
        format!("{:?}", self)
    }

    fn value(&self) -> u8 {
        self.clone().into()
    }
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
