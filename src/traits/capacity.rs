/// The graph capacity trait.
pub trait Capacity {
    /// Returns the capacity.
    ///
    /// Returns the number of vertex the graph can hold.
    /// Depending on the underlying storage, this could avoid reallocations.
    ///
    /// # Examples
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// // Build a graph with a specific capacity.
    /// let g = Graph::with_capacity(3);
    ///
    /// // Capacity constraits is soft-enforced.
    /// assert_le!(g.capacity(), 3);
    ///
    /// // The order is still zero.
    /// assert_eq!(g.order(), 0);
    ///
    /// // The size is still zero.
    /// assert_eq!(g.size(), 0);
    /// ```
    ///
    fn capacity(&self) -> usize;

    /// With capacity constructor.
    ///
    /// Construct a graph of a given capacity (a.k.a. order).
    /// Depending on the underlying storage, this could avoid reallocations.
    ///
    /// # Examples
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// // Build a graph with a specific capacity.
    /// let g = Graph::with_capacity(3);
    ///
    /// // Capacity constraits is soft-enforced.
    /// assert_le!(g.capacity(), 3);
    ///
    /// // The order is still zero.
    /// assert_eq!(g.order(), 0);
    ///
    /// // The size is still zero.
    /// assert_eq!(g.size(), 0);
    /// ```
    ///
    fn with_capacity(capacity: usize) -> Self;

    /// Reserves additional capacity.
    ///
    /// Reserves capacity for at least `additional` vertex to be inserted in the graph.
    /// Depending on the underlying storage, this could avoid reallocations.
    ///
    /// # Panics
    ///
    /// Panics if the additional capacity overflows `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// // Build a null graph.
    /// let mut g = Graph::null();
    ///
    /// // Reserve additional capacity.
    /// g.reserve(3);
    ///
    /// // Capacity constraits is soft-enforced.
    /// assert_le!(g.capacity(), 3);
    ///
    /// // The order is still zero.
    /// assert_eq!(g.order(), 0);
    ///
    /// // The size is still zero.
    /// assert_eq!(g.size(), 0);
    /// ```
    ///
    fn reserve(&mut self, additional: usize);

    /// Shrinks the capacity with a lower limit.
    ///
    /// Shrinks the capacity of the graph with a lower limit.
    ///
    /// # Examples
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// // Build graph with given capacity.
    /// let mut g = Graph::with_capacity(100);
    ///
    /// // Capacity constraits is soft-enforced.
    /// assert_le!(g.capacity(), 100);
    ///
    /// // Shrink capacity to given minimum.
    /// g.shrink_to(50);
    ///
    /// assert_le!(g.capacity(), 50);
    /// ```
    ///
    fn shrink_to(&mut self, min_capacity: usize);

    /// Shrinks the capacity.
    ///
    /// Shrinks the capacity of the graph as much as possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// // Build graph with given capacity.
    /// let mut g = Graph::with_capacity(100);
    ///
    /// // Capacity constraits is soft-enforced.
    /// assert_le!(g.capacity(), 100);
    ///
    /// // Shrink capacity as much as possible.
    /// g.shrink_to_fit();
    ///
    /// assert_le!(g.capacity(), 0);
    /// ```
    ///
    fn shrink_to_fit(&mut self);
}

macro_rules! impl_capacity {
    ($graph:ident) => {
        impl<T, U> $crate::traits::Capacity for $graph<T, U>
        where
            T: $crate::types::VertexTrait,
            U: $crate::traits::WithAttributes<T>,
        {
            fn with_capacity(capacity: usize) -> Self {
                Self {
                    data: $crate::traits::Capacity::with_capacity(capacity),
                    attributes: Default::default(),
                }
            }

            delegate::delegate! {
                to self.data {
                    fn capacity(&self) -> usize;
                    fn reserve(&mut self, additional: usize);
                    fn shrink_to(&mut self, min_capacity: usize);
                    fn shrink_to_fit(&mut self);
                }
            }
        }
    };
}

pub(crate) use impl_capacity;
