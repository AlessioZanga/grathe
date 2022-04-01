#[cfg(test)]
mod tests {
    macro_rules! generic_tests {
        ($G: ident) => {
            paste::item! {
                use all_asserts::*;
                use grathe::traits::{Capacity, Storage};

                #[test]
                fn capacity()
                {
                    // Test `G::capacity() -> usize`.

                    // Test for default.
                    let g = $G::<i32>::default();
                    assert_le!(g.capacity(), 0);
                    assert_eq!(g.order(), 0);
                    assert_eq!(g.size(), 0);
                }

                #[test]
                fn with_capacity()
                {
                    // Test `G::with_capacity(usize) -> G`.

                    // Test for ...
                    let data = [
                        0,          // ... zero,
                        1,          // ... one,
                        31,         // ... random value.
                        usize::MAX, // ... upper bound,
                    ];

                    // Test for each scenario.
                    for i in data {
                        let g = $G::<i32>::with_capacity(i);
                        assert_le!(g.capacity(), i);
                        assert_eq!(g.order(), 0);
                        assert_eq!(g.size(), 0);
                    }
                }

                #[test]
                fn reserve()
                {
                    // Test `G::reserve(usize) -> ()`.

                    // Test for ...
                    let data = [
                        0,          // ... zero,
                        1,          // ... one,
                        31,         // ... random value.
                        usize::MAX, // ... upper bound,
                    ];

                    // Test for each scenario.
                    for i in data {
                        let mut g = $G::<i32>::default();
                        g.reserve(i);
                        assert_le!(g.capacity(), i);
                        assert_eq!(g.order(), 0);
                        assert_eq!(g.size(), 0);
                    }
                }

                #[test]
                fn shrink_to()
                {
                    // Test `G::shrink_to(usize) -> ()`.

                    // Test for ...
                    let data = [
                        0,          // ... zero,
                        1,          // ... one,
                        31,         // ... random value.
                        usize::MAX, // ... upper bound,
                    ];

                    // Test for each scenario.
                    for i in data {
                        let mut g = $G::<i32>::default();
                        g.shrink_to(i);
                        assert_le!(g.capacity(), i);
                        assert_eq!(g.order(), 0);
                        assert_eq!(g.size(), 0);
                    }
                }

                #[test]
                fn shrink_to_fit()
                {
                    // Test `G::shrink_to_fit() -> ()`.

                    // Test for default.
                    let mut g = $G::<i32>::default();
                    g.shrink_to_fit();
                    assert_eq!(g, Default::default());
                    assert_le!(g.capacity(), 0);
                    assert_eq!(g.order(), 0);
                    assert_eq!(g.size(), 0);
                }
            }
        };
    }

    mod undirected_adjacency_list {
        use grathe::graphs::storages::UndirectedAdjacencyList;
        generic_tests!(UndirectedAdjacencyList);
    }

    mod directed_adjacency_list {
        use grathe::graphs::storages::DirectedAdjacencyList;
        generic_tests!(DirectedAdjacencyList);
    }
}
