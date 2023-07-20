use bincode::{Decode, Encode};

use crate::num::UInt;

#[derive(Encode, Decode)]
pub struct RleVec<L: UInt + 'static, T: 'static> {
    length: usize,
    chunks: Vec<(L, T)>,
}

impl<L: UInt + 'static, T: Eq + Clone> RleVec<L, T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            chunks: vec![],
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn push(&mut self, val: T) {
        self.length += 1;
        match self.chunks.last_mut().map(|(n, v)| (n, *v == val)) {
            Some((_, false)) | None => self.chunks.push((L::one(), val)),
            Some((tail_n, true)) => {
                if *tail_n == L::max_value() {
                    self.chunks.push((L::one(), val));
                } else {
                    *tail_n = *tail_n + L::one();
                }
            },
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let (last, pop) = match self.chunks.last_mut() {
            Some((count, val)) => {
                *count = *count - L::one();
                (Some(val.clone()), *count == L::zero())
            },
            None => (None, false),
        };
        if pop {
            self.chunks.pop();
        }
        if last.is_some() {
            self.length -= 1;
        }
        last
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx >= self.length {
            return None;
        }
        let mut chunk_start_idx: usize = 0;
        for (chunk_len, chunk_val) in self.chunks.iter() {
            let cl = (*chunk_len).to_usize().unwrap();
            if idx >= chunk_start_idx && idx < (chunk_start_idx + cl) {
                return Some(chunk_val);
            }
            chunk_start_idx += cl;
        }
        panic!("RleVec length is corrupted?")
    }

    pub fn iter(&self) -> RleVecIter<L, T> {
        RleVecIter {
            rle_v: self,
            idx: 0,
            subidx: L::zero(),
        }
    }
}

impl<L: UInt, T: Eq + Clone> FromIterator<T> for RleVec<L, T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut rv = RleVec::new();
        for t in iter {
            rv.push(t);
        }
        rv
    }
}

pub struct RleVecIter<'a, L: UInt + 'static, T: 'static> {
    rle_v: &'a RleVec<L, T>,
    idx: usize,
    subidx: L,
}

impl<'a, L: UInt, T> Iterator for RleVecIter<'a, L, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.rle_v.chunks.len() {
            None
        } else {
            let item = Some(&self.rle_v.chunks[self.idx].1);
            self.subidx = self.subidx + L::one();
            if self.subidx >= self.rle_v.chunks[self.idx].0 {
                self.idx += 1;
                self.subidx = L::zero();
            }
            item
        }
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_rle_from_iter() {
        let rle_v: RleVec<u8, String> = ["foo", "foo", "bar", "baz", "baz", "baz"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        assert_eq!(
            rle_v.chunks,
            vec![
                (2, "foo".to_string()),
                (1, "bar".to_string()),
                (3, "baz".to_string())
            ]
        )
    }

    #[rstest]
    fn test_rle_to_iter() {
        let rle_v: RleVec<u8, &str> = RleVec {
            length: 5,
            chunks: vec![(3, "foo"), (2, "baz")],
        };
        let v: Vec<&str> = rle_v.iter().cloned().collect();

        assert_eq!(v, vec!["foo", "foo", "foo", "baz", "baz"])
    }

    #[rstest]
    fn test_rle_pop() {
        let mut rle_v: RleVec<u8, &str> = RleVec {
            length: 3,
            chunks: vec![(1, "foo"), (2, "baz")],
        };
        assert_eq!(rle_v.pop(), Some("baz"));
        assert_eq!(rle_v.pop(), Some("baz"));
        assert_eq!(rle_v.pop(), Some("foo"));
        assert_eq!(rle_v.pop(), None);
    }

    #[rstest]
    fn test_rle_get() {
        let rle_v: RleVec<u8, u8> = RleVec {
            length: 3,
            chunks: vec![(1, 12), (2, 17)],
        };
        assert_eq!(rle_v.get(0), Some(&12));
        assert_eq!(rle_v.get(1), Some(&17));
        assert_eq!(rle_v.get(2), Some(&17));
        assert_eq!(rle_v.get(3), None);
        assert_eq!(rle_v.get(usize::MAX), None);
    }
}
