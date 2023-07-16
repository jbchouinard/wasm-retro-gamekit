use crate::vector::vec2d::Vec2d;

#[derive(Clone, Copy, Debug)]
pub enum KeyEventKind {
    Up,
    Down,
}

#[derive(Clone, Debug)]
pub struct KeyEvent {
    pub kind: KeyEventKind,
    pub key: String,
    pub alt: bool,
    pub ctrl: bool,
    pub meta: bool,
    pub shift: bool,
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum MouseButton {
    Left = 0,
    Middle = 1,
    Right = 2,
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
    pub pos: Vec2d<f32>,
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
