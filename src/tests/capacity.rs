#[cfg(test)]
#[generic_tests::define]
mod tests {
    use crate::graphs::storages::AdjacencyListStorage;
    use crate::traits::Storage;
    use all_asserts::*;

    #[test]
    fn with_capacity<T>()
    where
        T: Storage<Vertex = i32>,
    {
        let g = T::with_capacity(3);
        // FIXME: capacity constraits is soft-enforced.
        assert_le!(g.capacity(), 3);

        // The order is still zero.
        assert_eq!(g.order(), 0);

        // The size is still zero.
        assert_eq!(g.size(), 0);
    }

    #[test]
    fn reserve<T>()
    where
        T: Storage<Vertex = i32>,
    {
        let mut g = T::new();

        // Reserve additional capacity.
        g.reserve(3);

        // The order is still zero.
        assert_eq!(g.order(), 0);

        // The size is still zero.
        assert_eq!(g.size(), 0);
    }

    #[instantiate_tests(<AdjacencyListStorage<i32>>)]
    mod adjacency_list_storage {}
}
