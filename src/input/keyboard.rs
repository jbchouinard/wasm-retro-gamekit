use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub use super::keycodes::{InvalidKeyCode, KeyCode};
use crate::event::{Events, Filter, KeyEvent, KeyEventKind, Source};

pub struct KeyMap<T> {
    map: HashMap<KeyCode, T>,
}

impl<T> KeyMap<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn set(&mut self, code: KeyCode, mapped: &T) {
        self.map.insert(code, mapped.clone());
    }
    pub fn get(&self, code: &KeyCode) -> Option<T> {
        self.map.get(code).cloned()
    }
}

impl<T> std::fmt::Display for KeyMap<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (code, mapped) in self.map.iter() {
            writeln!(f, "{:?}:  {:?}", mapped, code)?;
        }
        Ok(())
    }
}

pub struct MappedKeyEvent<T> {
    pub kind: KeyEventKind,
    pub key: T,
    pub ts: f32,
}

impl<T> Filter<KeyEvent, MappedKeyEvent<T>> for KeyMap<T>
where
    T: Clone,
{
    fn filter(&mut self, t: KeyEvent) -> Option<MappedKeyEvent<T>> {
        let kind = t.kind;
        let ts = t.ts;
        self.get(&t.code)
            .map(|key| MappedKeyEvent { kind, key, ts })
    }
}

/// KeyState keeps track of key presses in two different
/// ways: it keeps track of which keys are currently pressed
/// (and since when it became pressed), and keep counters
/// of how many times each key was pressed.
///
/// KeyState::update must be called periodically to let
/// make KeyState process KeyEvents.
pub struct Keyboard<T> {
    key_down_counters: HashMap<T, u32>,
    last_key_down_t: HashMap<T, f32>,
    events: Source<MappedKeyEvent<T>>,
}

impl<T> Keyboard<T>
where
    T: PartialEq + Eq + Hash + Clone,
{
    pub fn new(events: Source<MappedKeyEvent<T>>) -> Self {
        Self {
            key_down_counters: HashMap::new(),
            last_key_down_t: HashMap::new(),
            events,
        }
    }
    pub fn update(&mut self) {
        while let Some(event) = self.events.recv() {
            match event.kind {
                KeyEventKind::Down => {
                    self.register_down(event.key, event.ts);
                },
                KeyEventKind::Up => {
                    self.register_up(event.key);
                },
            }
        }
    }
    fn register_up(&mut self, key: T) {
        self.last_key_down_t.remove(&key);
    }
    fn register_down(&mut self, key: T, ts: f32) {
        // Ignore repeated down events
        if !self.last_key_down_t.contains_key(&key) {
            self.last_key_down_t.insert(key.clone(), ts);
            *self.key_down_counters.entry(key).or_insert(0) += 1;
        }
    }
    /// is_down returns whether a key is currently pressed
    pub fn is_down(&self, key: &T) -> bool {
        self.last_key_down_t.contains_key(key)
    }
    // is_down_since returns the timestamp since when a key
    // has been held down, or None if it's currently up
    pub fn is_down_since(&self, key: &T) -> Option<f32> {
        self.last_key_down_t.get(key).copied()
    }
    /// take_presses returns the current value for the
    /// key press counter for that key, and resets its value to zero
    pub fn take_press_counter(&mut self, key: &T) -> u32 {
        self.key_down_counters.remove(key).unwrap_or(0)
    }
    /// peek_presses returns the current value of the key press
    /// counter for the key, without resetting it
    pub fn peek_press_counter(&self, key: &T) -> u32 {
        self.key_down_counters.get(key).copied().unwrap_or(0)
    }
}

pub fn attach_keyboard<T>(events: &mut Events, keymap: KeyMap<T>) -> Keyboard<T>
where
    T: PartialEq + Eq + Hash + Clone + 'static,
{
    let t_events = events.mapped_key_events(keymap);
    Keyboard::new(t_events)
}
