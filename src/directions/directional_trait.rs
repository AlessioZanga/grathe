use crate::directions::UndirectedTrait;

/// Directionality trait placeholder.
pub trait DirectionalTrait {}

impl<T> DirectionalTrait for T where T: UndirectedTrait {}
