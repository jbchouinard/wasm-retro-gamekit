mod events;
mod plumbing;
mod queue;

pub use self::events::*;
pub use self::plumbing::*;
// TODO: use mpsc::channel instead if not feature = "js"
pub use self::queue::*;
use crate::input::keyboard::{KeyMap, MappedKeyEvent};

pub struct Events {
    mains: Joint<Event>,
    plumbing: Plumbing,
}

impl Events {
    pub fn new() -> (Self, Sink<Event>) {
        let plumbing = Plumbing::new();
        let (sink, source) = plumbing.pipe::<Event>();
        let mut mains = Joint::new();
        mains.add_source(source);
        (Self { plumbing, mains }, sink)
    }

    pub fn plumbing(&mut self) -> &mut Plumbing {
        &mut self.plumbing
    }

    pub fn mouse_events(&mut self) -> Source<MouseEvent> {
        let (e_sink, e_source) = self.plumbing.pipe::<Event>();
        let (m_sink, m_source) = self.plumbing.pipe::<MouseEvent>();
        self.mains.add_sink(e_sink);
        self.plumbing
            .filter(e_source, m_sink, FnFilter(filter_mouse_events));
        m_source
    }

    pub fn key_events(&mut self) -> Source<KeyEvent> {
        let (e_sink, e_source) = self.plumbing.pipe::<Event>();
        let (k_sink, k_source) = self.plumbing.pipe::<KeyEvent>();
        self.mains.add_sink(e_sink);
        self.plumbing
            .filter(e_source, k_sink, FnFilter(filter_key_events));
        k_source
    }

    pub fn mapped_key_events<T>(&mut self, km: KeyMap<T>) -> Source<MappedKeyEvent<T>>
    where
        T: Clone + 'static,
    {
        let k_source = self.key_events();
        let (m_sink, m_source) = self.plumbing.pipe::<MappedKeyEvent<T>>();
        self.plumbing.filter(k_source, m_sink, km);
        m_source
    }

    pub fn window_resize_events(&mut self) -> Source<WindowResizeEvent> {
        let (e_sink, e_source) = self.plumbing.pipe::<Event>();
        let (w_sink, w_source) = self.plumbing.pipe::<WindowResizeEvent>();
        self.mains.add_sink(e_sink);
        self.plumbing
            .filter(e_source, w_sink, FnFilter(filter_resize_events));
        w_source
    }
}

impl Pump for Events {
    fn pump(&mut self) {
        self.mains.pump();
        self.plumbing.pump();
    }
}

fn filter_mouse_events(e: Event) -> Option<MouseEvent> {
    match e {
        Event::Mouse(mevent) => Some(mevent),
        _ => None,
    }
}

fn filter_key_events(e: Event) -> Option<KeyEvent> {
    match e {
        Event::Key(kevent) => Some(kevent),
        _ => None,
    }
}

fn filter_resize_events(e: Event) -> Option<WindowResizeEvent> {
    match e {
        Event::WindowResize(wevent) => Some(wevent),
        _ => None,
    }
}
