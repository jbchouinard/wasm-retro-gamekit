use crate::input::keyboard::KeyCode;
use crate::vector::v2::V2;

#[derive(Clone, Copy, Debug)]
pub enum KeyEventKind {
    Up,
    Down,
}

#[derive(Clone, Debug)]
pub struct KeyEvent {
    pub kind: KeyEventKind,
    pub code: KeyCode,
    pub ts: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MouseButton {
    Left = 0,
    Middle = 1,
    Right = 2,
}

impl From<MouseButton> for u8 {
    fn from(value: MouseButton) -> Self {
        match value {
            MouseButton::Left => 0,
            MouseButton::Middle => 1,
            MouseButton::Right => 2,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum MouseEventKind {
    Up(MouseButton),
    Down(MouseButton),
    Move,
}

#[derive(Clone, Debug)]
pub struct MouseEvent {
    pub kind: MouseEventKind,
    pub pos: V2<f32>,
    pub ts: f32,
}

#[derive(Clone, Debug)]
pub struct WindowResizeEvent {
    pub width: usize,
    pub height: usize,
}

#[derive(Copy, Clone, Debug)]
pub enum EventType {
    Key,
    Mouse,
    WindowResize,
}

#[derive(Clone, Debug)]
pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    WindowResize(WindowResizeEvent),
}

impl Event {
    pub fn event_type(&self) -> EventType {
        match self {
            Self::Key(_) => EventType::Key,
            Self::Mouse(_) => EventType::Mouse,
            Self::WindowResize(_) => EventType::WindowResize,
        }
    }
}
