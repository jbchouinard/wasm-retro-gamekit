use bincode::{Decode, Encode};

#[derive(Encode, Decode)]
pub struct RleVec<T: 'static> {
    length: usize,
    chunks: Vec<(u8, T)>,
}

impl<T: Eq + Clone> RleVec<T> {
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
            Some((_, false)) | Some((&mut u8::MAX, _)) | None => self.chunks.push((1, val)),
            Some((tail_n, true)) => {
                *tail_n += 1;
            },
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let (last, pop) = match self.chunks.last_mut() {
            Some((count, val)) => {
                *count -= 1;
                (Some(val.clone()), *count == 0)
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
            if idx >= chunk_start_idx && idx < (chunk_start_idx + *chunk_len as usize) {
                return Some(chunk_val);
            }
            chunk_start_idx += *chunk_len as usize;
        }
        panic!("RleVec length is corrupted?")
    }

    pub fn iter(&self) -> RleVecIter<T> {
        RleVecIter {
            rle_v: self,
            idx: 0,
            subidx: 0,
        }
    }
}

impl<T: Eq + Clone> FromIterator<T> for RleVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut rv = RleVec::new();
        for t in iter {
            rv.push(t);
        }
        rv
    }
}

pub struct RleVecIter<'a, T: 'static> {
    rle_v: &'a RleVec<T>,
    idx: usize,
    subidx: u8,
}

impl<'a, T> Iterator for RleVecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.rle_v.chunks.len() {
            None
        } else {
            let item = Some(&self.rle_v.chunks[self.idx].1);
            self.subidx += 1;
            if self.subidx >= self.rle_v.chunks[self.idx].0 {
                self.idx += 1;
                self.subidx = 0;
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
        let rle_v: RleVec<String> = ["foo", "foo", "bar", "baz", "baz", "baz"]
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
        let rle_v: RleVec<&str> = RleVec {
            length: 5,
            chunks: vec![(3, "foo"), (2, "baz")],
        };
        let v: Vec<&str> = rle_v.iter().cloned().collect();

        assert_eq!(v, vec!["foo", "foo", "foo", "baz", "baz"])
    }

    #[rstest]
    fn test_rle_pop() {
        let mut rle_v: RleVec<&str> = RleVec {
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
        let rle_v: RleVec<u8> = RleVec {
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
