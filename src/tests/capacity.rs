#[cfg(test)]
#[generic_tests::define]
mod tests {
    use crate::graphs::storages::{DirectedAdjacencyList, UndirectedAdjacencyList};
    use crate::traits::Storage;
    use all_asserts::*;

    #[test]
    fn with_capacity<G>()
    where
        G: Storage<Vertex = i32>,
    {
        let g = G::with_capacity(3);
        // FIXME: capacity constraits is soft-enforced.
        assert_le!(g.capacity(), 3);

        // The order is still zero.
        assert_eq!(g.order(), 0);

        // The size is still zero.
        assert_eq!(g.size(), 0);
    }

    #[test]
    fn reserve<G>()
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::null();

        // Reserve additional capacity.
        g.reserve(3);

        // The order is still zero.
        assert_eq!(g.order(), 0);

        // The size is still zero.
        assert_eq!(g.size(), 0);
    }

    #[instantiate_tests(<UndirectedAdjacencyList<i32>>)]
    mod undirected_adjacency_list {}

    #[instantiate_tests(<DirectedAdjacencyList<i32>>)]
    mod directed_adjacency_list {}
}
