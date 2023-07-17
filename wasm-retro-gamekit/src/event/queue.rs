use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub(super) struct Queue<T>(pub(super) Rc<RefCell<VecDeque<T>>>);

impl<T> Clone for Queue<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub struct Source<T>(pub(super) Queue<T>);

impl<T> Source<T> {
    pub fn recv(&self) -> Option<T> {
        self.0 .0.borrow_mut().pop_back()
    }
}

pub struct Sink<T>(pub(super) Queue<T>);

impl<T> Sink<T> {
    pub fn send(&self, t: T) {
        self.0 .0.borrow_mut().push_front(t);
    }
}

impl<T> Clone for Sink<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
