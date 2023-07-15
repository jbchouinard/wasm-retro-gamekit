use std::cmp::Ordering;

/// An unordered pair, e.g. Pair::new(a, b) == Pair::new(b, a);
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Pair<T>(T, T);

impl<T> Pair<T>
where
    T: PartialOrd + Ord,
{
    pub fn new(a: T, b: T) -> Self {
        match a.cmp(&b) {
            Ordering::Greater => Pair(b, a),
            _ => Pair(a, b),
        }
    }

    pub fn tuple(&self) -> (&T, &T) {
        (&self.0, &self.1)
    }
}
