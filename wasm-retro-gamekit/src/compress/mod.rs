pub mod rle;
use bincode::{Decode, Encode};

use self::rle::RleVecIter;
use crate::compress::rle::RleVec;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "bins", derive(clap::ValueEnum))]
pub enum Compression {
    None,
    Rle8,
    Rle16,
}

#[derive(Encode, Decode)]
pub enum Data<T: 'static + Eq + Clone> {
    Decomp(Vec<T>),
    Rle8(RleVec<u8, T>),
    Rle16(RleVec<u16, T>),
}

enum DataIter<'a, T: 'static + Eq + Clone> {
    Decomp(std::slice::Iter<'a, T>),
    Rle8(RleVecIter<'a, u8, T>),
    Rle16(RleVecIter<'a, u16, T>),
}

impl<'a, T: 'static + Eq + Clone> Iterator for DataIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Decomp(it) => it.next(),
            Self::Rle8(it) => it.next(),
            Self::Rle16(it) => it.next(),
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
            Compression::Rle8 => Self::Rle8(RleVec::new()),
            Compression::Rle16 => Self::Rle16(RleVec::new()),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Decomp(v) => v.len(),
            Self::Rle8(rv) => rv.len(),
            Self::Rle16(rv) => rv.len(),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Self::Decomp(v) => std::mem::size_of::<T>() * v.len(),
            Self::Rle8(rv) => (std::mem::size_of::<T>() + std::mem::size_of::<u8>()) * rv.len(),
            Self::Rle16(rv) => (std::mem::size_of::<T>() + std::mem::size_of::<u16>()) * rv.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Decomp(v) => v.is_empty(),
            Self::Rle8(rv) => rv.is_empty(),
            Self::Rle16(rv) => rv.is_empty(),
        }
    }
    pub fn from_iter<I: IntoIterator<Item = T>>(iter: I, compression: Compression) -> Self {
        match compression {
            Compression::None => Self::Decomp(iter.into_iter().collect()),
            Compression::Rle8 => Self::Rle8(iter.into_iter().collect()),
            Compression::Rle16 => Self::Rle16(iter.into_iter().collect()),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        match self {
            Self::Decomp(v) => DataIter::Decomp(v.iter()),
            Self::Rle8(rv) => DataIter::Rle8(rv.iter()),
            Self::Rle16(rv) => DataIter::Rle16(rv.iter()),
        }
    }

    pub fn compression(&self) -> Compression {
        match self {
            Self::Decomp(_) => Compression::None,
            Self::Rle8(_) => Compression::Rle8,
            Self::Rle16(_) => Compression::Rle16,
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
            Self::Rle8(rv) => rv.push(val),
            Self::Rle16(rv) => rv.push(val),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self {
            Self::Decomp(v) => v.pop(),
            Self::Rle8(rv) => rv.pop(),
            Self::Rle16(rv) => rv.pop(),
        }
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        match self {
            Self::Decomp(v) => v.get(idx),
            Self::Rle8(rv) => rv.get(idx),
            Self::Rle16(rv) => rv.get(idx),
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

        data.compress(Compression::Rle16);
        got = data.iter().cloned().collect();
        assert_eq!(want, got);

        data.decompress();
        got = data.iter().cloned().collect();
        assert_eq!(want, got);
    }
}
