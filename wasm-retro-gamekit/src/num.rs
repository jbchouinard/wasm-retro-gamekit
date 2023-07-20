use bincode::{Decode, Encode};
pub use num_traits::bounds::Bounded;
pub use num_traits::{FromPrimitive, One, ToPrimitive, Zero};

pub trait Float:
    num_traits::Float + FromPrimitive + ToPrimitive + Encode + Decode + Copy + Clone
{
}

pub trait UInt:
    num_integer::Integer + FromPrimitive + ToPrimitive + Encode + Decode + Copy + Clone + Eq + Bounded
{
}

impl Float for f32 {}
impl Float for f64 {}

impl UInt for usize {}
impl UInt for u8 {}
impl UInt for u16 {}
impl UInt for u32 {}
impl UInt for u64 {}
impl UInt for u128 {}
