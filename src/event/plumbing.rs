use std::cell::RefCell;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::rc::Rc;

use super::{Queue, Sink, Source};

pub trait Pump {
    fn pump(&mut self);
}

pub trait Filter<T, U> {
    fn filter(&mut self, t: T) -> Option<U>;
}

pub(super) struct FnFilter<F>(pub(super) F);

impl<F, T, U> Filter<T, U> for FnFilter<F>
where
    F: FnMut(T) -> Option<U>,
{
    fn filter(&mut self, t: T) -> Option<U> {
        (self.0)(t)
    }
}

struct FilterPump<F, T, U> {
    source: Source<T>,
    sink: Sink<U>,
    filter: F,
    t: PhantomData<T>,
    u: PhantomData<U>,
}

impl<F, T, U> Pump for FilterPump<F, T, U>
where
    F: Filter<T, U>,
{
    fn pump(&mut self) {
        while let Some(t_event) = self.source.recv() {
            if let Some(u_event) = self.filter.filter(t_event) {
                self.sink.send(u_event);
            }
        }
    }
}

pub(super) struct Joint<T> {
    sources: Vec<Source<T>>,
    sinks: Vec<Sink<T>>,
}

impl<T> Joint<T> {
    pub fn new() -> Self {
        Self {
            sources: vec![],
            sinks: vec![],
        }
    }

    pub fn add_source(&mut self, source: Source<T>) {
        self.sources.push(source);
    }
    pub fn add_sink(&mut self, sink: Sink<T>) {
        self.sinks.push(sink);
    }
}

impl<T> Pump for Joint<T>
where
    T: Clone,
{
    fn pump(&mut self) {
        for source in &self.sources {
            while let Some(e) = source.recv() {
                for sink in &self.sinks {
                    sink.send(e.clone());
                }
            }
        }
    }
}

pub struct Plumbing {
    pumps: Vec<Box<dyn Pump>>,
}

impl Plumbing {
    pub fn new() -> Self {
        Self { pumps: vec![] }
    }

    pub fn pipe<T>(&self) -> (Sink<T>, Source<T>) {
        {
            let q: Queue<T> = Queue(Rc::new(RefCell::new(VecDeque::new())));
            (Sink(q.clone()), Source(q))
        }
    }

    pub fn filter<F, T, U>(&mut self, source: Source<T>, sink: Sink<U>, filter: F)
    where
        F: Filter<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        let p = FilterPump {
            filter,
            source,
            sink,
            t: PhantomData,
            u: PhantomData,
        };
        self.pumps.push(Box::new(p));
    }
}

impl Pump for Plumbing {
    fn pump(&mut self) {
        for p in self.pumps.iter_mut() {
            p.pump();
        }
    }
}
