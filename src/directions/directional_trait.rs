/// Directionality trait placeholder.
pub trait DirectionalTrait {
    /// Whether the graph directed or not.
    fn is_directed(&self) -> bool;

    /// Whether the graph is partially-directed or not.
    fn is_partially_directed(&self) -> bool;
}
