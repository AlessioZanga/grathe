use crate::directions::DirectionalTrait;
use crate::storages::StorageTrait;

/// The base graph trait.
pub trait GraphTrait: DirectionalTrait + StorageTrait {}
