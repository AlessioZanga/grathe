#[cfg(test)]
mod subgraph {
    macro_rules! generic_tests {
        ($G: ident) => {
            use all_asserts::*;
            use grathe::{
                traits::{Storage, Subgraph},
                E, V,
            };

            #[test]
            fn subgraph_is_subgraph_is_supergraph() {
                // Test `G::subgraph(I: IntoIter<Item = i32>) -> Self`.

                // Test for ...
                let data = [
                    // ... zero vertices and zero edges,
                    (vec![], vec![], vec![], (0, 0, vec![], vec![])),
                    // ... one vertex and zero edges,
                    (vec![0], vec![], vec![0], (1, 0, vec![&0], vec![])),
                    // ... one vertex and one edge,
                    (vec![0], vec![(0, 0)], vec![0], (1, 1, vec![&0], vec![(&0, &0)])),
                    // ... multiple vertices and zero edges,
                    (
                        vec![0, 1, 2, 3],
                        vec![],
                        vec![0, 1, 2],
                        (3, 0, vec![&0, &1, &2], vec![]),
                    ),
                    // ... multiple vertices and one edge,
                    (
                        vec![0, 1, 2, 3],
                        vec![(0, 1)],
                        vec![0, 1, 2],
                        (3, 1, vec![&0, &1, &2], vec![(&0, &1)]),
                    ),
                    // ... multiple vertices and multiple edges,
                    (
                        vec![0, 1, 2, 3],
                        vec![(0, 1), (1, 2), (2, 3)],
                        vec![0, 1, 2],
                        (3, 2, vec![&0, &1, &2], vec![(&0, &1), (&1, &2)]),
                    ),
                    // ... random vertices and edges,
                    (
                        vec![71, 1, 58, 3, 75],
                        vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                        vec![1, 58, 75],
                        (3, 1, vec![&1, &58, &75], vec![(&1, &58)]),
                    ),
                ];

                // Test for each scenario.
                for (i, j, l, k) in data {
                    let g = $G::<i32>::new(i, j);
                    let h = g.subgraph(l);
                    let (o, s, v, e) = k;
                    assert_eq!(h.order(), o);
                    assert_eq!(h.size(), s);
                    assert_eq!(Vec::from_iter(V!(h)), v);
                    assert_eq!(Vec::from_iter(E!(h)), e);
                    assert!(h.is_subgraph(&g));
                    assert!(g.is_supergraph(&h));
                    assert_le!(h, g);
                    assert_ge!(g, h);
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
