pub trait Float: num::Float + num::FromPrimitive + num::ToPrimitive + num::Zero + num::One {}

impl Float for f32 {}
impl Float for f64 {}
