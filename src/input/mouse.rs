use crate::event::{Events, Filter, MouseButton, MouseEvent, MouseEventKind, Plumbing, Source};
use crate::vector::v2::V2;

pub enum MouseInteractionKind {
    Click,
    Drag,
    Drop,
}

pub struct MouseInteraction {
    pub kind: MouseInteractionKind,
    pub button: MouseButton,
    pub pos: V2<f32>,
    pub ts: f32,
}

#[derive(Clone)]
struct ButtonState {
    button: MouseButton,
    last_down_ts: f32,
    last_down_pos: V2<f32>,
    is_down: bool,
    is_drag: bool,
    sent_drag: bool,
}

impl ButtonState {
    fn new(button: MouseButton) -> Self {
        Self {
            button,
            last_down_ts: 0.0,
            last_down_pos: V2::zero(),
            is_down: false,
            is_drag: false,
            sent_drag: false,
        }
    }

    fn update(
        &mut self,
        event: &MouseEvent,
        max_click_delay: f32,
        max_click_distance: f32,
    ) -> Option<MouseInteraction> {
        let ts = event.ts;
        let pos = event.pos;
        let delay = ts - self.last_down_ts;
        let distance = (pos - self.last_down_pos).mag().abs();
        self.is_drag = delay > max_click_delay || distance > max_click_distance;
        match event.kind {
            MouseEventKind::Up(_) => {
                let kind: Option<MouseInteractionKind> = match (self.is_drag, self.sent_drag) {
                    (true, true) => Some(MouseInteractionKind::Drop),
                    (false, false) => Some(MouseInteractionKind::Click),
                    _ => None,
                };
                self.is_down = false;
                self.is_drag = false;
                self.sent_drag = false;
                kind.map(|k| MouseInteraction {
                    kind: k,
                    button: self.button,
                    pos,
                    ts,
                })
            },
            MouseEventKind::Down(_) => {
                self.last_down_pos = pos;
                self.last_down_ts = ts;
                self.is_down = true;
                None
            },
            MouseEventKind::Move => {
                if !self.is_down {
                    return None;
                }
                if self.is_drag && !self.sent_drag {
                    self.sent_drag = true;
                    Some(MouseInteraction {
                        kind: MouseInteractionKind::Drag,
                        button: self.button,
                        pos,
                        ts,
                    })
                } else {
                    None
                }
            },
        }
    }
}

pub struct MouseFilter {
    max_click_delay: f32,
    max_click_distance: f32,
    last_move_update: u8,
    buttons: [ButtonState; 3],
}

impl MouseFilter {
    pub fn new(max_click_delay: f32, max_click_distance: f32) -> Self {
        Self {
            max_click_delay,
            max_click_distance,
            last_move_update: 0,
            buttons: [
                ButtonState::new(MouseButton::Left),
                ButtonState::new(MouseButton::Middle),
                ButtonState::new(MouseButton::Right),
            ],
        }
    }
}

impl Filter<MouseEvent, MouseInteraction> for MouseFilter {
    fn filter(&mut self, me: MouseEvent) -> Option<MouseInteraction> {
        let update_idx: u8 = match me.kind {
            MouseEventKind::Down(button) => button.into(),
            MouseEventKind::Up(button) => button.into(),
            MouseEventKind::Move => {
                self.last_move_update = (self.last_move_update + 1) % 3;
                self.last_move_update
            },
        };
        let button = &mut self.buttons[update_idx as usize];
        button.update(&me, self.max_click_delay, self.max_click_distance)
    }
}

pub struct Mouse {
    pos: V2<f32>,
    interactions: Source<MouseInteraction>,
}

impl Mouse {
    pub fn relpos(&self) -> &V2<f32> {
        &self.pos
    }
    pub fn interactions(&self) -> &Source<MouseInteraction> {
        &self.interactions
    }
}

pub fn attach_mouse(events: &mut Events, max_click_delay: f32, max_click_distance: f32) -> Mouse {
    let m_source = events.mouse_events();
    let (mint_sink, mint_source) = events.plumbing().pipe::<MouseInteraction>();
    let plumbing: &mut Plumbing = events.plumbing();
    plumbing.filter(
        m_source,
        mint_sink,
        MouseFilter::new(max_click_delay, max_click_distance),
    );
    Mouse {
        pos: V2::zero(),
        interactions: mint_source,
    }
}
