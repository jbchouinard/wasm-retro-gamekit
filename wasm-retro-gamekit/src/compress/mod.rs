pub mod rle;
use bincode::{Decode, Encode};

use self::rle::RleVecIter;
use crate::compress::rle::RleVec;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Compression {
    None,
    Rle,
}

#[derive(Encode, Decode)]
pub enum Data<T: 'static + Eq + Clone> {
    Decomp(Vec<T>),
    Rle(RleVec<T>),
}

enum DataIter<'a, T: 'static + Eq + Clone> {
    Decomp(std::slice::Iter<'a, T>),
    Rle(RleVecIter<'a, T>),
}

impl<'a, T: 'static + Eq + Clone> Iterator for DataIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Decomp(it) => it.next(),
            Self::Rle(it) => it.next(),
        }
    }
}

#[derive(Debug)]
pub struct DataError(String);

pub type Result<T> = std::result::Result<T, DataError>;

impl<T: Eq + Clone> Data<T> {
    pub fn new(compression: Compression) -> Self {
        match compression {
            Compression::None => Self::Decomp(Vec::new()),
            Compression::Rle => Self::Rle(RleVec::new()),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Decomp(v) => v.len(),
            Self::Rle(rv) => rv.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Decomp(v) => v.is_empty(),
            Self::Rle(rv) => rv.is_empty(),
        }
    }
    pub fn from_iter<I: IntoIterator<Item = T>>(iter: I, compression: Compression) -> Self {
        match compression {
            Compression::None => Self::Decomp(iter.into_iter().collect()),
            Compression::Rle => Self::Rle(iter.into_iter().collect()),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        match self {
            Self::Decomp(v) => DataIter::Decomp(v.iter()),
            Self::Rle(rv) => DataIter::Rle(rv.iter()),
        }
    }

    pub fn compression(&self) -> Compression {
        match self {
            Self::Decomp(_) => Compression::None,
            Self::Rle(_) => Compression::Rle,
        }
    }

    pub fn is_compressed(&self) -> bool {
        matches!(self.compression(), Compression::None)
    }

    pub fn compress(&mut self, compression: Compression) {
        if self.compression() != compression {
            let compressed = Self::from_iter(self.iter().cloned(), compression);
            *self = compressed;
        }
    }

    pub fn decompress(&mut self) {
        self.compress(Compression::None)
    }

    pub fn into_vec(mut self) -> Vec<T> {
        self.decompress();
        match self {
            Self::Decomp(v) => v,
            _ => panic!("failed to decompress?"),
        }
    }

    pub fn push(&mut self, val: T) {
        match self {
            Self::Decomp(v) => v.push(val),
            Self::Rle(rv) => rv.push(val),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self {
            Self::Decomp(v) => v.pop(),
            Self::Rle(rv) => rv.pop(),
        }
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        match self {
            Self::Decomp(v) => v.get(idx),
            Self::Rle(rv) => rv.get(idx),
        }
    }

    pub fn get_mut(&mut self, idx: usize) -> Result<Option<&mut T>> {
        match self {
            Self::Decomp(v) => Ok(v.get_mut(idx)),
            _ => Err(DataError(
                "cannot get mutable ref to compressed datum".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_rle_de_compress() {
        let want: Vec<u8> = vec![1, 1, 1, 2, 3, 1, 2, 2, 3, 2, 2, 2, 2, 2];
        let mut data = Data::from_iter(want.iter().cloned(), Compression::None);

        let mut got: Vec<u8> = data.iter().cloned().collect();
        assert_eq!(want, got);

        data.compress(Compression::Rle);
        got = data.iter().cloned().collect();
        assert_eq!(want, got);

        data.decompress();
        got = data.iter().cloned().collect();
        assert_eq!(want, got);
    }
}
