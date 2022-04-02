#[cfg(test)]
mod tests {
    mod undirected {
        macro_rules! generic_tests {
            ($G: ident) => {
                use all_asserts::*;
                use grathe::traits::{Connectivity, From, Storage};

                #[test]
                #[ignore]
                // FIXME:
                fn has_path() {
                    // Test self edge graph.
                    let g = $G::<i32>::from_edges([(0, 0)]);
                    assert_true!(g.has_path(&0, &0));

                    // Test single edge graph.
                    let g = $G::<i32>::from_edges([(0, 1)]);

                    assert_false!(g.has_path(&0, &0));
                    assert_true!(g.has_path(&0, &1));

                    // Test single path graph.
                    let g = $G::<i32>::from_edges([(0, 1), (1, 2)]);

                    assert_true!(g.has_path(&0, &1));
                    assert_true!(g.has_path(&1, &2));
                    assert_true!(g.has_path(&1, &0));
                    assert_true!(g.has_path(&2, &0));
                    assert_true!(g.has_path(&2, &1));

                    // Test multi path edge graph.
                    let g = $G::<i32>::from_edges([(0, 1), (1, 2), (1, 3), (3, 4)]);

                    assert_true!(g.has_path(&0, &4));
                    assert_true!(g.has_path(&1, &4));
                    assert_true!(g.has_path(&2, &4));

                    // Test disconnected graph.
                    let g = $G::<i32>::from_edges([(0, 1), (1, 2), (3, 4)]);

                    assert_false!(g.has_path(&0, &4));
                    assert_false!(g.has_path(&1, &4));
                    assert_false!(g.has_path(&2, &4));
                    assert_true!(g.has_path(&3, &4));

                    // Test cyclic graph.
                    let g = $G::<i32>::from_edges([(0, 1), (1, 2), (3, 4), (2, 0)]);
                    assert_true!(g.has_path(&0, &0));
                }

                #[test]
                #[should_panic]
                // FIXME:
                fn has_path_should_panic() {
                    // Test null path.
                    let g = $G::<i32>::null();
                    assert_false!(g.has_path(&0, &0));
                }

                #[test]
                #[ignore]
                // FIXME:
                fn is_connected() {
                    // Test null graph.
                    let g = $G::<i32>::null();
                    assert_true!(g.is_connected());

                    // Test single edge graph.
                    let g = $G::<i32>::from_edges([(0, 1)]);
                    assert_true!(g.is_connected());

                    // Test multiple edges graph.
                    let g = $G::<i32>::from_edges([(0, 1), (1, 2)]);
                    assert_true!(g.is_connected());

                    // Test disconnected graph.
                    let mut g = $G::<i32>::from_edges([(0, 1), (1, 2)]);
                    g.add_vertex(3);
                    assert_false!(g.is_connected());

                    // Test connected cyclic graph.
                    let g = $G::<i32>::from_edges([(0, 1), (1, 2), (2, 0)]);
                    assert_true!(g.is_connected());

                    // Test disconnected cyclic graph.
                    let mut g = $G::<i32>::from_edges([(0, 1), (1, 2), (2, 0)]);
                    g.add_vertex(3);
                    assert_false!(g.is_connected());
                }

                #[test]
                #[ignore]
                // FIXME:
                fn is_acyclic() {
                    let g = $G::<i32>::null();
                    assert_true!(g.is_acyclic());

                    let g = $G::<i32>::from_vertices([0]);
                    assert_true!(g.is_acyclic());

                    let g = $G::<i32>::from_edges([(0, 1)]);
                    assert_true!(g.is_acyclic());

                    let g = $G::<i32>::from_edges([(0, 1), (1, 2)]);
                    assert_true!(g.is_acyclic());

                    let g = $G::<i32>::from_edges([(0, 1), (1, 2), (2, 0)]);
                    assert_false!(g.is_acyclic());
                }
            };
        }

        mod undirected_adjacency_list {
            use grathe::graphs::storages::UndirectedAdjacencyList;
            generic_tests!(UndirectedAdjacencyList);
        }
    }

    mod directed {
        macro_rules! generic_tests {
            ($G: ident) => {
                use all_asserts::*;
                use grathe::traits::{Connectivity, From, Storage};

                #[test]
                #[ignore]
                // FIXME:
                fn has_path() {
                    // Test self edge graph.
                    let g = $G::<i32>::from_edges([(0, 0)]);
                    assert_true!(g.has_path(&0, &0));

                    // Test single edge graph.
                    let g = $G::<i32>::from_edges([(0, 1)]);

                    assert_false!(g.has_path(&0, &0));
                    assert_true!(g.has_path(&0, &1));

                    // Test single path graph.
                    let g = $G::<i32>::from_edges([(0, 1), (1, 2)]);

                    assert_true!(g.has_path(&0, &1));
                    assert_true!(g.has_path(&1, &2));
                    assert_false!(g.has_path(&1, &0));
                    assert_false!(g.has_path(&2, &0));
                    assert_false!(g.has_path(&2, &1));

                    // Test multi path graph.
                    let g = $G::<i32>::from_edges([(0, 1), (1, 2), (1, 3), (3, 4)]);

                    assert_true!(g.has_path(&0, &4));
                    assert_true!(g.has_path(&1, &4));
                    assert_false!(g.has_path(&2, &4));

                    // Test disconnected graph.
                    let g = $G::<i32>::from_edges([(0, 1), (1, 2), (3, 4)]);

                    assert_false!(g.has_path(&0, &4));
                    assert_false!(g.has_path(&1, &4));
                    assert_false!(g.has_path(&2, &4));
                    assert_true!(g.has_path(&3, &4));

                    // Test cyclic graph.
                    let g = $G::<i32>::from_edges([(0, 1), (1, 2), (3, 4), (2, 0)]);
                    assert_true!(g.has_path(&0, &0));
                }

                #[test]
                #[should_panic]
                // FIXME:
                fn has_path_should_panic() {
                    // Test null graph.
                    let g = $G::<i32>::null();
                    assert_false!(g.has_path(&0, &0));
                }

                #[test]
                #[ignore]
                // FIXME:
                fn is_connected() {
                    // Test null graph.
                    let g = $G::<i32>::null();
                    assert_true!(g.is_connected());

                    // Test single edge graph.
                    let g = $G::<i32>::from_edges([(0, 1)]);
                    assert_true!(g.is_connected());

                    // Test multiple edges graph.
                    let g = $G::<i32>::from_edges([(0, 1), (1, 2)]);
                    assert_true!(g.is_connected());

                    // Test disconnected graph.
                    let mut g = $G::<i32>::from_edges([(0, 1), (1, 2)]);
                    g.add_vertex(3);
                    assert_false!(g.is_connected());

                    // Test connected cyclic graph.
                    let g = $G::<i32>::from_edges([(0, 1), (1, 2), (2, 0)]);
                    assert_true!(g.is_connected());

                    // Test disconnected cyclic graph.
                    let mut g = $G::<i32>::from_edges([(0, 1), (1, 2), (2, 0)]);
                    g.add_vertex(3);
                    assert_false!(g.is_connected());
                }

                #[test]
                #[ignore]
                // FIXME:
                fn is_acyclic() {
                    let g = $G::<i32>::null();
                    assert_true!(g.is_acyclic());

                    let g = $G::<i32>::from_vertices([0]);
                    assert_true!(g.is_acyclic());

                    let g = $G::<i32>::from_edges([(0, 1)]);
                    assert_true!(g.is_acyclic());

                    let g = $G::<i32>::from_edges([(0, 1), (1, 2)]);
                    assert_true!(g.is_acyclic());

                    let g = $G::<i32>::from_edges([(0, 1), (1, 2), (2, 0)]);
                    assert_false!(g.is_acyclic());
                }
            };
        }

        mod directed_adjacency_list {
            use grathe::graphs::storages::DirectedAdjacencyList;
            generic_tests!(DirectedAdjacencyList);
        }
    }
}
