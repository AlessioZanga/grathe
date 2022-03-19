#[cfg(test)]
#[generic_tests::define]
mod tests {
    use crate::graphs::storages::{DirectedAdjacencyList, UndirectedAdjacencyList};
    use crate::traits::Storage;
    use all_asserts::*;

    #[test]
    fn capacity<G>()
    where
        G: Storage,
    {
        // Test `G::capacity()`.

        // Test for default.
        let g = G::default();
        assert_le!(g.capacity(), 0);
        assert_eq!(g.order(), 0);
        assert_eq!(g.size(), 0);
    }

    #[test]
    fn with_capacity<G>()
    where
        G: Storage,
    {
        // Test `G::with_capacity(usize)`.

        // Test for ...
        let data = [
            0,          // ... zero,
            1,          // ... one,
            31,         // ... random value.
            usize::MAX, // ... upper bound,
        ];

        // Test for each scenario.
        for i in data {
            let g = G::with_capacity(i);
            assert_le!(g.capacity(), i);
            assert_eq!(g.order(), 0);
            assert_eq!(g.size(), 0);
        }
    }

    #[test]
    fn reserve<G>()
    where
        G: Storage,
    {
        // Test `G::reserve(usize)`.

        // Test for ...
        let data = [
            0,          // ... zero,
            1,          // ... one,
            31,         // ... random value.
            usize::MAX, // ... upper bound,
        ];

        // Test for each scenario.
        for i in data {
            let mut g = G::default();
            g.reserve(i);
            assert_le!(g.capacity(), i);
            assert_eq!(g.order(), 0);
            assert_eq!(g.size(), 0);
        }
    }

    #[test]
    fn shrink_to<G>()
    where
        G: Storage,
    {
        // Test `G::shrink_to(usize)`.

        // Test for ...
        let data = [
            0,          // ... zero,
            1,          // ... one,
            31,         // ... random value.
            usize::MAX, // ... upper bound,
        ];

        // Test for each scenario.
        for i in data {
            let mut g = G::default();
            g.shrink_to(i);
            assert_le!(g.capacity(), i);
            assert_eq!(g.order(), 0);
            assert_eq!(g.size(), 0);
        }
    }

    #[test]
    fn shrink_to_fit<G>()
    where
        G: Storage,
    {
        // Test `G::shrink_to_fit()`.

        // Test for default.
        let mut g = G::default();
        g.shrink_to_fit();
        assert_eq!(g, Default::default());
        assert_le!(g.capacity(), 0);
        assert_eq!(g.order(), 0);
        assert_eq!(g.size(), 0);
    }

    #[instantiate_tests(<UndirectedAdjacencyList<i32>>)]
    mod undirected_adjacency_list {}

    #[instantiate_tests(<DirectedAdjacencyList<i32>>)]
    mod directed_adjacency_list {}
}
