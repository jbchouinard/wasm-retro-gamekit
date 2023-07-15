use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

use crate::vector::vec2d::Vec2d;

#[derive(Clone, Debug)]
pub struct KeyEvent {
    pub key: String,
    pub alt: bool,
    pub ctrl: bool,
    pub meta: bool,
    pub shift: bool,
}

#[derive(Clone, Debug)]
pub struct MouseClickEvent {
    pub pos: Vec2d<f32>,
}

#[derive(Clone, Debug)]
pub struct WindowResizeEvent {
    pub width: usize,
    pub height: usize,
}

#[derive(Copy, Clone, Debug)]
pub enum EventType {
    KeyDown,
    KeyUp,
    MouseClick,
    WindowResize,
}

#[derive(Clone, Debug)]
pub enum Event {
    KeyDown(KeyEvent),
    KeyUp(KeyEvent),
    MouseClick(MouseClickEvent),
    WindowResize(WindowResizeEvent),
}

impl Event {
    pub fn event_type(&self) -> EventType {
        match self {
            Self::KeyDown(_) => EventType::KeyDown,
            Self::KeyUp(_) => EventType::KeyUp,
            Self::MouseClick(_) => EventType::MouseClick,
            Self::WindowResize(_) => EventType::WindowResize,
        }
    }
}

pub enum EventBusError {
    QueueFull,
}

#[derive(Clone, Debug)]
pub struct EventQueue {
    max_size: usize,
    queue: Rc<RefCell<VecDeque<Event>>>,
}

impl EventQueue {
    pub fn new(max_size: usize) -> Self {
        Self {
            max_size,
            queue: Rc::new(RefCell::new(VecDeque::new())),
        }
    }
    pub fn send<T: Into<Event>>(&self, event: T) -> Result<(), EventBusError> {
        let mut q = self.queue.borrow_mut();
        if self.max_size > 0 && q.len() >= self.max_size {
            Err(EventBusError::QueueFull)
        } else {
            q.push_front(event.into());
            Ok(())
        }
    }
    pub fn recv(&self) -> Option<Event> {
        let mut q = self.queue.borrow_mut();
        q.pop_back()
    }
}

pub trait EventListener {
    fn on_key_up(&mut self, _event: &KeyEvent) {}
    fn on_key_down(&mut self, _event: &KeyEvent) {}
    fn on_mouse_click(&mut self, _event: &MouseClickEvent) {}
    fn on_window_resize(&mut self, _event: &WindowResizeEvent) {}
    fn on_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown(kevent) => self.on_key_down(kevent),
            Event::KeyUp(kevent) => self.on_key_up(kevent),
            Event::MouseClick(mevent) => self.on_mouse_click(mevent),
            Event::WindowResize(wrevent) => self.on_window_resize(wrevent),
        }
    }
}

pub struct EventPipe(EventQueue);

impl EventPipe {
    pub fn new(queue: EventQueue) -> Self {
        Self(queue)
    }
}

impl EventListener for EventPipe {
    fn on_event(&mut self, event: &Event) {
        let _ = self.0.send(event.clone());
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ListenerHandle(usize);

pub struct EventRouter {
    source: EventQueue,
    listeners: Vec<Box<dyn EventListener>>,
    wants_key_up: HashSet<usize>,
    wants_key_down: HashSet<usize>,
    wants_mouse_click: HashSet<usize>,
    wants_window_resize: HashSet<usize>,
}

impl EventRouter {
    pub fn new(bus: EventQueue) -> Self {
        Self {
            source: bus,
            listeners: Vec::new(),
            wants_key_down: HashSet::new(),
            wants_key_up: HashSet::new(),
            wants_mouse_click: HashSet::new(),
            wants_window_resize: HashSet::new(),
        }
    }
    pub fn add_listener<T: EventListener + 'static>(
        &mut self,
        event_types: &[EventType],
        listener: T,
    ) -> ListenerHandle {
        let idx: usize = self.listeners.len();

        self.listeners.push(Box::new(listener));

        for event_type in event_types.iter() {
            match *event_type {
                EventType::KeyDown => self.wants_key_down.insert(idx),
                EventType::KeyUp => self.wants_key_up.insert(idx),
                EventType::MouseClick => self.wants_mouse_click.insert(idx),
                EventType::WindowResize => self.wants_window_resize.insert(idx),
            };
        }
        ListenerHandle(idx)
    }
    pub fn remove_listener(&mut self, handle: ListenerHandle) {
        let _idx = handle.0;
        todo!()
    }
    pub fn dispatch(&mut self) {
        while let Some(event) = self.source.recv() {
            let wants = match event.event_type() {
                EventType::KeyDown => &self.wants_key_down,
                EventType::KeyUp => &self.wants_key_up,
                EventType::MouseClick => &self.wants_mouse_click,
                EventType::WindowResize => &self.wants_window_resize,
            };
            for idx in wants.iter() {
                let listener = &mut self.listeners[*idx];
                listener.on_event(&event);
            }
        }
    }
}
