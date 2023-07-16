pub use num_traits::{FromPrimitive, One, ToPrimitive, Zero};

pub trait Float: num_traits::Float + FromPrimitive + ToPrimitive + Zero + One {}

impl Float for f32 {}
impl Float for f64 {}
