#[cfg(test)]
mod undirected {
    macro_rules! generic_tests {
        ($G: ident) => {
            #[test]
            fn breadth_first_search_tree() {
                // Build a null graph.
                let g = $G::<i32>::null();
                // Build a search object.
                let mut search = BFS::from(&g);
                // To search on a null graph
                // without a source vertex
                // yields no result.
                assert_eq!(search.next(), None);

                // Build a null graph.
                let mut g = $G::<i32>::null();
                assert!(g.add_vertex(0));

                // Execute BFS for the trivial graph.
                let mut search = BFS::from((&g, &0));
                // Collect the vertex in pre-order.
                let order: Vec<_> = search.by_ref().collect();
                // Check visit order.
                assert_eq!(order, [&0]);

                // The BFS on a trivial graph contains only the root ...
                assert_eq!(search.distance.len(), 1);
                // ... and no predecessors by definition.
                assert_eq!(search.predecessor.len(), 0);
                // The root has distance zero ...
                assert_eq!(search.distance.get(&0), Some(&0));
                // ... and no predecessors by definition.
                assert_eq!(search.predecessor.get(&0), None);

                // Add an edge.
                assert!(g.add_vertex(1));
                assert!(g.add_edge(&0, &1));

                // Execute BFS for the non-trivial graph.
                let mut search = BFS::from((&g, &0));
                // Collect the vertex in pre-order.
                let order: Vec<_> = search.by_ref().collect();
                // Check visit order.
                assert_eq!(order, [&0, &1]);

                // Check BFS coherence.
                assert_eq!(search.distance.len(), 2);
                assert_eq!(search.predecessor.len(), 1);

                // Check distances.
                assert_eq!(search.distance.get(&0), Some(&0));
                assert_eq!(search.distance.get(&1), Some(&1));

                // Check predecessors.
                assert_eq!(search.predecessor.get(&0), None);
                assert_eq!(search.predecessor.get(&1), Some(&&0));

                // Add a disconnected vertex.
                assert!(g.add_vertex(2));

                // Execute BFS for the non-connected graph.
                let mut search = BFS::from((&g, &0));
                // Collect the vertex in pre-order.
                let order: Vec<_> = search.by_ref().collect();
                // Check visit order.
                assert_eq!(order, [&0, &1]);

                // Check BFS coherence.
                assert_eq!(search.distance.len(), 2);
                assert_eq!(search.predecessor.len(), 1);

                // Check distances.
                assert_eq!(search.distance.get(&0), Some(&0));
                assert_eq!(search.distance.get(&1), Some(&1));
                assert_eq!(search.distance.get(&2), None);

                // Check predecessors.
                assert_eq!(search.predecessor.get(&0), None);
                assert_eq!(search.predecessor.get(&1), Some(&&0));
                assert_eq!(search.predecessor.get(&2), None);

                // Build non-trivial graph.
                let g = $G::<i32>::from_edges([
                    (0, 1),
                    (1, 2),
                    (0, 3),
                    (3, 4),
                    (3, 5),
                    (4, 5),
                    (4, 6),
                    (5, 6),
                    (5, 7),
                    (6, 7),
                    (7, 7),
                ]);

                // Which essentially is:
                //
                //  1 - 0*  4 - 6
                //  |   | / | / | / \   <-- Self loop
                //  2   3 - 5 - 7 - /       over `7`.
                //
                // where `0` is the source vertex.

                // Execute DFS for the non-trivial graph.
                let mut search = BFS::from((&g, &0));
                // Collect the vertex in pre-order.
                let order: Vec<_> = search.by_ref().collect();
                // Check visit order.
                assert_eq!(order, [&0, &1, &3, &2, &4, &5, &6, &7]);

                // Check BFS coherence.
                assert_eq!(search.distance.len(), 8);
                assert_eq!(search.predecessor.len(), 7);

                // Check distances.
                assert_eq!(search.distance.get(&0), Some(&0));
                assert_eq!(search.distance.get(&1), Some(&1));
                assert_eq!(search.distance.get(&2), Some(&2));
                assert_eq!(search.distance.get(&3), Some(&1));
                assert_eq!(search.distance.get(&4), Some(&2));
                assert_eq!(search.distance.get(&5), Some(&2));
                assert_eq!(search.distance.get(&6), Some(&3));
                assert_eq!(search.distance.get(&7), Some(&3));

                // Check predecessors.
                assert_eq!(search.predecessor.get(&0), None);
                assert_eq!(search.predecessor.get(&1), Some(&&0));
                assert_eq!(search.predecessor.get(&2), Some(&&1));
                assert_eq!(search.predecessor.get(&3), Some(&&0));
                assert_eq!(search.predecessor.get(&4), Some(&&3));
                assert_eq!(search.predecessor.get(&5), Some(&&3));
                assert_eq!(search.predecessor.get(&6), Some(&&4));
                assert_eq!(search.predecessor.get(&7), Some(&&5));
            }

            #[test]
            #[should_panic]
            fn breadth_first_search_tree_should_panic() {
                // Build a null graph.
                let g = $G::<i32>::null();
                BFS::from((&g, &0)).next();
            }

            #[test]
            fn depth_first_search_tree() {
                // Build a null graph.
                let g = $G::<i32>::null();
                // Build a search object.
                let mut search = DFS::from(&g);
                // To search on a null graph
                // without a source vertex
                // yields no result.
                assert_eq!(search.next(), None);

                // Build a null graph.
                let mut g = $G::<i32>::null();
                assert!(g.add_vertex(0));

                // Execute DFS for the trivial graph.
                let mut search = DFS::from((&g, &0));
                // Collect the vertex in pre-order.
                let order: Vec<_> = search.by_ref().collect();
                // Check visit order.
                assert_eq!(order, [&0]);

                // The DFS on a trivial graph contains only the root ...
                assert_eq!(search.discovery_time.len(), 1);
                assert_eq!(search.finish_time.len(), 1);
                // ... and no predecessors by definition.
                assert_eq!(search.predecessor.len(), 0);
                // The root has discovery time zero ...
                assert_eq!(search.discovery_time.get(&0), Some(&0));
                // ... and finish time one ...
                assert_eq!(search.finish_time.get(&0), Some(&1));
                // ... and no predecessors by definition.
                assert_eq!(search.predecessor.get(&0), None);

                // Add an edge.
                assert!(g.add_vertex(1));
                assert!(g.add_edge(&0, &1));

                // Execute DFS for the non-trivial graph.
                let mut search = DFS::from((&g, &0));
                // Collect the vertex in pre-order.
                let order: Vec<_> = search.by_ref().collect();
                // Check visit order.
                assert_eq!(order, [&0, &1]);

                // Check DFS coherence.
                assert_eq!(search.discovery_time.len(), 2);
                assert_eq!(search.finish_time.len(), 2);
                assert_eq!(search.predecessor.len(), 1);

                // Check distances.
                assert_eq!(search.discovery_time.get(&0), Some(&0));
                assert_eq!(search.discovery_time.get(&1), Some(&1));
                assert_eq!(search.finish_time.get(&1), Some(&2));
                assert_eq!(search.finish_time.get(&0), Some(&3));

                // Check predecessors.
                assert_eq!(search.predecessor.get(&0), None);
                assert_eq!(search.predecessor.get(&1), Some(&&0));

                // Add a disconnected vertex.
                assert!(g.add_vertex(2));

                // Execute DFS for the disconnected graph.
                let mut search = DFS::from((&g, &0));
                // Collect the vertex in pre-order.
                let order: Vec<_> = search.by_ref().collect();
                // Check visit order.
                assert_eq!(order, [&0, &1]);

                // Check DFS coherence.
                assert_eq!(search.discovery_time.len(), 2);
                assert_eq!(search.finish_time.len(), 2);
                assert_eq!(search.predecessor.len(), 1);

                // Check distances.
                assert_eq!(search.discovery_time.get(&0), Some(&0));
                assert_eq!(search.discovery_time.get(&1), Some(&1));
                assert_eq!(search.finish_time.get(&1), Some(&2));
                assert_eq!(search.finish_time.get(&0), Some(&3));

                assert_eq!(search.discovery_time.get(&2), None);
                assert_eq!(search.finish_time.get(&2), None);

                // Check predecessors.
                assert_eq!(search.predecessor.get(&0), None);
                assert_eq!(search.predecessor.get(&1), Some(&&0));
                assert_eq!(search.predecessor.get(&2), None);

                // Build non-trivial graph.
                let g = $G::<i32>::from_edges([
                    (0, 1),
                    (1, 2),
                    (0, 3),
                    (3, 4),
                    (3, 5),
                    (4, 5),
                    (4, 6),
                    (5, 6),
                    (5, 7),
                    (6, 7),
                    (7, 7),
                ]);

                // Which essentially is:
                //
                //  1 - 0*  4 - 6
                //  |   | / | / | / \   <-- Self loop
                //  2   3 - 5 - 7 - /       over `7`.
                //
                // where `0` is the source vertex.

                // Execute DFS for the non-trivial graph.
                let mut search = DFS::from((&g, &0));
                // Collect the vertex in pre-order.
                let order: Vec<_> = search.by_ref().collect();
                // Check visit order.
                assert_eq!(order, [&0, &1, &2, &3, &4, &5, &6, &7]);

                // Check DFS coherence.
                assert_eq!(search.discovery_time.len(), 8);
                assert_eq!(search.finish_time.len(), 8);
                assert_eq!(search.predecessor.len(), 7);

                // Check distances.
                assert_eq!(search.discovery_time.get(&0), Some(&0));
                assert_eq!(search.discovery_time.get(&1), Some(&1));
                assert_eq!(search.discovery_time.get(&2), Some(&2));
                assert_eq!(search.finish_time.get(&2), Some(&3));
                assert_eq!(search.finish_time.get(&1), Some(&4));
                assert_eq!(search.discovery_time.get(&3), Some(&5));
                assert_eq!(search.discovery_time.get(&4), Some(&6));
                assert_eq!(search.discovery_time.get(&5), Some(&7));
                assert_eq!(search.discovery_time.get(&6), Some(&8));
                assert_eq!(search.discovery_time.get(&7), Some(&9));
                assert_eq!(search.finish_time.get(&7), Some(&10));
                assert_eq!(search.finish_time.get(&6), Some(&11));
                assert_eq!(search.finish_time.get(&5), Some(&12));
                assert_eq!(search.finish_time.get(&4), Some(&13));
                assert_eq!(search.finish_time.get(&3), Some(&14));
                assert_eq!(search.finish_time.get(&0), Some(&15));

                // Check predecessors.
                assert_eq!(search.predecessor.get(&0), None);
                assert_eq!(search.predecessor.get(&1), Some(&&0));
                assert_eq!(search.predecessor.get(&2), Some(&&1));
                assert_eq!(search.predecessor.get(&3), Some(&&0));
                assert_eq!(search.predecessor.get(&4), Some(&&3));
                assert_eq!(search.predecessor.get(&5), Some(&&4));
                assert_eq!(search.predecessor.get(&6), Some(&&5));
                assert_eq!(search.predecessor.get(&7), Some(&&6));
            }

            #[test]
            #[should_panic]
            fn depth_first_search_tree_should_panic() {
                // Build a null graph.
                let g = $G::<i32>::null();
                DFS::from((&g, &0)).next();
            }

            #[test]
            fn lexicographic_breadth_first_search_tree() {
                // Alias of VecDequeue.
                type Q<G> = std::collections::VecDeque<G>;

                // Build a null graph.
                let g = $G::<i32>::null();
                // Build a search object.
                let mut search = LexBFS::from(&g);
                // To search on a null graph
                // without a source vertex
                // yields no result.
                assert_eq!(search.next(), None);

                // Test from Figure 1 of reference paper,
                // with vertices (x, y, w, z, u, v, a, d, c, b)
                // mapped to the (0, 1, 2, 3, 4, 5, 6, 7, 8, 9)
                // sequence for an easy equality check.
                let g = $G::<i32>::from_edges([
                    (0, 1),
                    (0, 2),
                    (0, 4),
                    (0, 5),
                    (3, 1),
                    (3, 2),
                    (3, 4),
                    (3, 5),
                    (6, 1),
                    (6, 2),
                    (6, 4),
                    (6, 5),
                    (7, 1),
                    (7, 2),
                    (7, 4),
                    (7, 5),
                    (8, 1),
                    (8, 2),
                    (8, 4),
                    (8, 5),
                    (9, 1),
                    (9, 2),
                    (9, 4),
                    (9, 5),
                    (0, 3),
                    (1, 2),
                    (3, 6),
                    (4, 5),
                    (7, 8),
                    (7, 9),
                ]);
                let mut search = LexBFS::from(&g);

                // Test from Table 1 of reference paper.
                assert_eq!(
                    search.partitions,
                    Q::from_iter([
                        // [x, y, w, z, u, v, a, d, c, b]
                        Q::from_iter(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
                    ])
                );

                assert_eq!(search.next(), Some(&0)); // [0, x]
                assert_eq!(search.predecessor.get(&0), None);
                assert_eq!(
                    search.partitions,
                    Q::from_iter([
                        // [y, w, z, u, v]
                        Q::from_iter(&[1, 2, 3, 4, 5]),
                        // [a, d, c, b]
                        Q::from_iter(&[6, 7, 8, 9])
                    ])
                );

                assert_eq!(search.next(), Some(&1)); // [1, y]
                assert_eq!(search.predecessor[&1], &0);
                assert_eq!(
                    search.partitions,
                    Q::from_iter([
                        // [w, z]
                        Q::from_iter(&[2, 3]),
                        // [u, v]
                        Q::from_iter(&[4, 5]),
                        // [a, d, c, b]
                        Q::from_iter(&[6, 7, 8, 9])
                    ])
                );

                assert_eq!(search.next(), Some(&2)); // [2, w]
                assert_eq!(search.predecessor[&2], &0);
                assert_eq!(
                    search.partitions,
                    Q::from_iter([
                        // [z]
                        Q::from_iter(&[3]),
                        // [u, v]
                        Q::from_iter(&[4, 5]),
                        // [a, d, c, b]
                        Q::from_iter(&[6, 7, 8, 9])
                    ])
                );

                assert_eq!(search.next(), Some(&3)); // [3, z]
                assert_eq!(search.predecessor[&3], &0);
                assert_eq!(
                    search.partitions,
                    Q::from_iter([
                        // [u, v]
                        Q::from_iter(&[4, 5]),
                        // [a]
                        Q::from_iter(&[6]),
                        // [d, c, b]
                        Q::from_iter(&[7, 8, 9])
                    ])
                );

                assert_eq!(search.next(), Some(&4)); // [4, u]
                assert_eq!(search.predecessor[&4], &0);
                assert_eq!(
                    search.partitions,
                    Q::from_iter([
                        // [v]
                        Q::from_iter(&[5]),
                        // [a]
                        Q::from_iter(&[6]),
                        // [d, c, b]
                        Q::from_iter(&[7, 8, 9])
                    ])
                );

                assert_eq!(search.next(), Some(&5)); // [5, v]
                assert_eq!(search.predecessor[&5], &0);
                assert_eq!(
                    search.partitions,
                    Q::from_iter([
                        // [a]
                        Q::from_iter(&[6]),
                        // [d, c, b]
                        Q::from_iter(&[7, 8, 9])
                    ])
                );

                assert_eq!(search.next(), Some(&6)); // [6, a]
                assert_eq!(search.predecessor[&6], &1);
                assert_eq!(
                    search.partitions,
                    Q::from_iter([
                        // [d, c, b]
                        Q::from_iter(&[7, 8, 9])
                    ])
                );

                assert_eq!(search.next(), Some(&7)); // [7, d]
                assert_eq!(search.predecessor[&7], &1);
                assert_eq!(
                    search.partitions,
                    Q::from_iter([
                        // [c, b]
                        Q::from_iter(&[8, 9])
                    ])
                );

                assert_eq!(search.next(), Some(&8)); // [8, c]
                assert_eq!(search.predecessor[&8], &1);
                assert_eq!(
                    search.partitions,
                    Q::from_iter([
                        // [b]
                        Q::from_iter(&[9])
                    ])
                );

                assert_eq!(search.next(), Some(&9)); // [9, b]
                assert_eq!(search.predecessor[&9], &1);
                assert_eq!(search.partitions, Q::from_iter([]));

                assert_eq!(search.next(), None);
                assert_eq!(search.partitions, Q::from_iter([]));

                // Test from course slides
                // with (f, g, c, d, b, a, e)
                // as   (0, 1, 2, 3, 4, 5, 6).
                let g = $G::<i32>::from_edges([
                    (0, 1),
                    (0, 2),
                    (1, 2),
                    (1, 3),
                    (2, 3),
                    (2, 4),
                    (2, 5),
                    (3, 4),
                    (3, 5),
                    (3, 6),
                    (4, 5),
                ]);
                let mut search = LexBFS::from(&g);

                assert_eq!(
                    search.partitions,
                    Q::from_iter([
                        // [f, g, c, d, b, a, e]
                        Q::from_iter(&[0, 1, 2, 3, 4, 5, 6])
                    ])
                );

                assert_eq!(search.next(), Some(&0)); // [0, f]
                assert_eq!(search.predecessor.get(&0), None);
                assert_eq!(
                    search.partitions,
                    Q::from_iter([
                        // [g, c]
                        Q::from_iter(&[1, 2]),
                        // [d, b, a, e]
                        Q::from_iter(&[3, 4, 5, 6])
                    ])
                );

                assert_eq!(search.next(), Some(&1)); // [1, g]
                assert_eq!(search.predecessor[&1], &0);
                assert_eq!(
                    search.partitions,
                    Q::from_iter([
                        // [c]
                        Q::from_iter(&[2]),
                        // [d]
                        Q::from_iter(&[3]),
                        // [b, a, e]
                        Q::from_iter(&[4, 5, 6])
                    ])
                );

                assert_eq!(search.next(), Some(&2)); // [2, c]
                assert_eq!(search.predecessor[&2], &0);
                assert_eq!(
                    search.partitions,
                    Q::from_iter([
                        // [d]
                        Q::from_iter(&[3]),
                        // [b, a]
                        Q::from_iter(&[4, 5]),
                        // [e]
                        Q::from_iter(&[6]),
                    ])
                );

                assert_eq!(search.next(), Some(&3)); // [3, d]
                assert_eq!(search.predecessor[&3], &1);
                assert_eq!(
                    search.partitions,
                    Q::from_iter([
                        // [b, a]
                        Q::from_iter(&[4, 5]),
                        // [e]
                        Q::from_iter(&[6]),
                    ])
                );

                assert_eq!(search.next(), Some(&4)); // [4, b]
                assert_eq!(search.predecessor[&4], &2);
                assert_eq!(
                    search.partitions,
                    Q::from_iter([
                        // [a]
                        Q::from_iter(&[5]),
                        // [e]
                        Q::from_iter(&[6])
                    ])
                );

                assert_eq!(search.next(), Some(&5)); // [5, a]
                assert_eq!(search.predecessor[&5], &2);
                assert_eq!(
                    search.partitions,
                    Q::from_iter([
                        // [e]
                        Q::from_iter(&[6])
                    ])
                );

                assert_eq!(search.next(), Some(&6)); // [6, e]
                assert_eq!(search.predecessor[&6], &3);
                assert_eq!(search.partitions, Q::from_iter([]));

                assert_eq!(search.next(), None);
                assert_eq!(search.partitions, Q::from_iter([]));
            }

            #[test]
            #[should_panic]
            fn lexicographic_breadth_first_search_tree_should_panic() {
                // Build a null graph.
                let g = $G::<i32>::null();
                LexBFS::from((&g, &0)).next();
            }

            #[test]
            fn lexicographic_depth_first_search() {
                // Build a null graph.
                let g = $G::<i32>::null();
                // Build a search object.
                let mut search = LexDFS::from(&g);
                // To search on a null graph
                // without a source vertex
                // yields no result.
                assert_eq!(search.next(), None);

                let g = $G::<i32>::from_edges([(0, 1), (0, 2), (0, 4), (1, 2), (1, 3), (2, 4), (2, 3)]);
                let mut search = LexDFS::from(&g);

                assert_eq!(search.next(), Some(&0));
                assert_eq!(search.predecessor.get(&0), None);

                assert_eq!(search.next(), Some(&1));
                assert_eq!(search.predecessor[&1], &0);

                assert_eq!(search.next(), Some(&2));
                assert_eq!(search.predecessor[&2], &1);

                assert_eq!(search.next(), Some(&3));
                assert_eq!(search.predecessor[&3], &2);

                assert_eq!(search.next(), Some(&4));
                assert_eq!(search.predecessor[&4], &2);

                assert_eq!(search.next(), None);
            }

            #[test]
            #[should_panic]
            fn lexicographic_depth_first_search_tree_should_panic() {
                // Build a null graph.
                let g = $G::<i32>::null();
                LexDFS::from((&g, &0)).next();
            }
        };
    }

    mod adjacency_list {
        use grathe::algorithms::{LexBFS, LexDFS, BFS, DFS};
        use grathe::graphs::storages::UndirectedAdjacencyList;
        use grathe::traits::{From, Storage};

        generic_tests!(UndirectedAdjacencyList);
    }
}

#[cfg(test)]
mod directed {
    macro_rules! generic_tests {
        ($G: ident) => {
            #[test]
            fn breadth_first_search_tree() {
                // Build a null graph.
                let g = $G::<i32>::null();
                // Build a search object.
                let mut search = BFS::from(&g);
                // To search on a null graph
                // without a source vertex
                // yields no result.
                assert_eq!(search.next(), None);

                // Build a null graph.
                let mut g = $G::<i32>::null();
                assert!(g.add_vertex(0));
                // Execute BFS for the trivial graph.
                let search = BFS::from((&g, &0));

                // The BFS on a trivial graph contains only the root ...
                assert_eq!(search.distance.len(), 1);
                // ... and no predecessors by definition.
                assert_eq!(search.predecessor.len(), 0);
                // The root has distance zero ...
                assert_eq!(search.distance.get(&0), Some(&0));
                // ... and no predecessors by definition.
                assert_eq!(search.predecessor.get(&0), None);

                // Add an edge.
                assert!(g.add_vertex(1));
                g.add_edge(&0, &1);
                // Execute BFS for the non-trivial graph.
                let mut search = BFS::from((&g, &0));
                // Collect the vertex in pre-order.
                let order: Vec<_> = search.by_ref().collect();
                // Check visit order.
                assert_eq!(order, [&0, &1]);

                // Check BFS coherence.
                assert_eq!(search.distance.len(), 2);
                assert_eq!(search.predecessor.len(), 1);

                // Check distances.
                assert_eq!(search.distance.get(&0), Some(&0));
                assert_eq!(search.distance.get(&1), Some(&1));

                // Check predecessors.
                assert_eq!(search.predecessor.get(&0), None);
                assert_eq!(search.predecessor.get(&1), Some(&&0));

                // Add a disconnected vertex.
                assert!(g.add_vertex(2));
                // Execute BFS for the non-connected graph.
                let mut search = BFS::from((&g, &0));
                // Collect the vertex in pre-order.
                let order: Vec<_> = search.by_ref().collect();
                // Check visit order.
                assert_eq!(order, [&0, &1]);

                // Check BFS coherence.
                assert_eq!(search.distance.len(), 2);
                assert_eq!(search.predecessor.len(), 1);

                // Check distances.
                assert_eq!(search.distance.get(&0), Some(&0));
                assert_eq!(search.distance.get(&1), Some(&1));
                assert_eq!(search.distance.get(&2), None);

                // Check predecessors.
                assert_eq!(search.predecessor.get(&0), None);
                assert_eq!(search.predecessor.get(&1), Some(&&0));
                assert_eq!(search.predecessor.get(&2), None);

                // Build non-trivial graph.
                let g = $G::<i32>::from_edges([
                    (0, 1),
                    (1, 2),
                    (0, 3),
                    (3, 4),
                    (3, 5),
                    (4, 5),
                    (4, 6),
                    (5, 6),
                    (5, 7),
                    (6, 7),
                    (7, 7),
                ]);
                // Which essentially is:
                //
                //  1 <- 0* > 4 -> 6
                //  |    | /  | /  | /  \   <-- Self loop
                //  v    v    v    v
                //  2    3 -> 5 -> 7 <- /       over `7`.
                //
                // where `0` is the source vertex.
                // Execute DFS for the non-trivial graph.
                let mut search = BFS::from((&g, &0));
                // Collect the vertex in pre-order.
                let order: Vec<_> = search.by_ref().collect();
                // Check visit order.
                assert_eq!(order, [&0, &1, &3, &2, &4, &5, &6, &7]);

                // Check BFS coherence.
                assert_eq!(search.distance.len(), 8);
                assert_eq!(search.predecessor.len(), 7);

                // Check distances.
                assert_eq!(search.distance.get(&0), Some(&0));
                assert_eq!(search.distance.get(&1), Some(&1));
                assert_eq!(search.distance.get(&2), Some(&2));
                assert_eq!(search.distance.get(&3), Some(&1));
                assert_eq!(search.distance.get(&4), Some(&2));
                assert_eq!(search.distance.get(&5), Some(&2));
                assert_eq!(search.distance.get(&6), Some(&3));
                assert_eq!(search.distance.get(&7), Some(&3));

                // Check predecessors.
                assert_eq!(search.predecessor.get(&0), None);
                assert_eq!(search.predecessor.get(&1), Some(&&0));
                assert_eq!(search.predecessor.get(&2), Some(&&1));
                assert_eq!(search.predecessor.get(&3), Some(&&0));
                assert_eq!(search.predecessor.get(&4), Some(&&3));
                assert_eq!(search.predecessor.get(&5), Some(&&3));
                assert_eq!(search.predecessor.get(&6), Some(&&4));
                assert_eq!(search.predecessor.get(&7), Some(&&5));
            }

            #[test]
            #[should_panic]
            fn breadth_first_search_tree_should_panic() {
                // Build a null graph.
                let g = $G::<i32>::null();
                BFS::from((&g, &0)).next();
            }

            #[test]
            fn breadth_first_search_forest() {
                // Build a null graph.
                let g = $G::<i32>::null();
                // Build a search object.
                let mut search = BFS::new(&g, None, grathe::algorithms::Traversal::Forest);
                // To search on a null graph
                // without a source vertex
                // yields no result.
                assert_eq!(search.next(), None);

                // Build a disconnected graph.
                let g = $G::<i32>::from_edges([
                    (0, 1),
                    (0, 2),
                    (0, 3),
                    (1, 4),
                    (2, 1),
                    (3, 5),
                    (5, 4),
                    (6, 7),
                    (7, 8),
                    (6, 9),
                    (9, 8),
                ]);
                // Build a search object.
                let mut search = BFS::new(&g, None, grathe::algorithms::Traversal::Forest);
                // Collect the vertex in pre-order.
                let order: Vec<_> = search.by_ref().collect();
                // Check visit order.
                assert_eq!(order, [&0, &1, &2, &3, &4, &5, &6, &7, &9, &8]);

                // Check BFS coherence.
                assert_eq!(search.distance.len(), 10);
                assert_eq!(search.predecessor.len(), 8);

                // Check distances.
                assert_eq!(search.distance.get(&0), Some(&0));
                assert_eq!(search.distance.get(&1), Some(&1));
                assert_eq!(search.distance.get(&2), Some(&1));
                assert_eq!(search.distance.get(&3), Some(&1));
                assert_eq!(search.distance.get(&4), Some(&2));
                assert_eq!(search.distance.get(&5), Some(&2));
                assert_eq!(search.distance.get(&6), Some(&usize::MAX));
                assert_eq!(search.distance.get(&7), Some(&usize::MAX));
                assert_eq!(search.distance.get(&8), Some(&usize::MAX));
                assert_eq!(search.distance.get(&9), Some(&usize::MAX));

                // Check predecessors.
                assert_eq!(search.predecessor.get(&0), None);
                assert_eq!(search.predecessor.get(&1), Some(&&0));
                assert_eq!(search.predecessor.get(&2), Some(&&0));
                assert_eq!(search.predecessor.get(&3), Some(&&0));
                assert_eq!(search.predecessor.get(&4), Some(&&1));
                assert_eq!(search.predecessor.get(&5), Some(&&3));
                assert_eq!(search.predecessor.get(&6), None);
                assert_eq!(search.predecessor.get(&7), Some(&&6));
                assert_eq!(search.predecessor.get(&8), Some(&&7));
                assert_eq!(search.predecessor.get(&9), Some(&&6));
            }

            #[test]
            fn depth_first_search_tree() {
                // Build a null graph.
                let g = $G::<i32>::null();
                // Build a search object.
                let mut search = DFS::from(&g);
                // To search on a null graph
                // without a source vertex
                // yields no result.
                assert_eq!(search.next(), None);

                // Build a null graph.
                let mut g = $G::<i32>::null();
                assert!(g.add_vertex(0));
                // Execute DFS for the trivial graph.
                let mut search = DFS::from((&g, &0));
                // Collect the vertex in pre-order.
                let order: Vec<_> = search.by_ref().collect();
                // Check visit order.
                assert_eq!(order, [&0]);

                // The DFS on a trivial graph contains only the root ...
                assert_eq!(search.discovery_time.len(), 1);
                assert_eq!(search.finish_time.len(), 1);
                // ... and no predecessors by definition.
                assert_eq!(search.predecessor.len(), 0);
                // The root has discovery time zero ...
                assert_eq!(search.discovery_time.get(&0), Some(&0));
                // ... and finish time one ...
                assert_eq!(search.finish_time.get(&0), Some(&1));
                // ... and no predecessors by definition.
                assert_eq!(search.predecessor.get(&0), None);

                // Add an edge.
                assert!(g.add_vertex(1));
                assert!(g.add_edge(&0, &1));
                // Execute DFS for the non-trivial graph.
                let mut search = DFS::from((&g, &0));
                // Collect the vertex in pre-order.
                let order: Vec<_> = search.by_ref().collect();
                // Check visit order.
                assert_eq!(order, [&0, &1]);

                // Check DFS coherence.
                assert_eq!(search.discovery_time.len(), 2);
                assert_eq!(search.finish_time.len(), 2);
                assert_eq!(search.predecessor.len(), 1);

                // Check distances.
                assert_eq!(search.discovery_time.get(&0), Some(&0));
                assert_eq!(search.discovery_time.get(&1), Some(&1));
                assert_eq!(search.finish_time.get(&1), Some(&2));
                assert_eq!(search.finish_time.get(&0), Some(&3));

                // Check predecessors.
                assert_eq!(search.predecessor.get(&0), None);
                assert_eq!(search.predecessor.get(&1), Some(&&0));

                // Add a disconnected vertex.
                assert!(g.add_vertex(2));
                // Execute DFS for the non-connected graph.
                let mut search = DFS::from((&g, &0));
                // Collect the vertex in pre-order.
                let order: Vec<_> = search.by_ref().collect();
                // Check visit order.
                assert_eq!(order, [&0, &1]);

                // Check DFS coherence.
                assert_eq!(search.discovery_time.len(), 2);
                assert_eq!(search.finish_time.len(), 2);
                assert_eq!(search.predecessor.len(), 1);

                // Check distances.
                assert_eq!(search.discovery_time.get(&0), Some(&0));
                assert_eq!(search.discovery_time.get(&1), Some(&1));
                assert_eq!(search.finish_time.get(&1), Some(&2));
                assert_eq!(search.finish_time.get(&0), Some(&3));
                assert_eq!(search.discovery_time.get(&2), None);
                assert_eq!(search.finish_time.get(&2), None);

                // Check predecessors.
                assert_eq!(search.predecessor.get(&0), None);
                assert_eq!(search.predecessor.get(&1), Some(&&0));
                assert_eq!(search.predecessor.get(&2), None);

                // Build non-trivial graph.
                let g = $G::<i32>::from_edges([
                    (0, 1),
                    (1, 2),
                    (0, 3),
                    (3, 4),
                    (3, 5),
                    (4, 5),
                    (4, 6),
                    (5, 6),
                    (5, 7),
                    (6, 7),
                    (7, 7),
                ]);
                // Which essentially is:
                //
                //  1 - 0*  4 - 6
                //  |   | / | / | / \   <-- Self loop
                //  2   3 - 5 - 7 - /       over `7`.
                //
                // where `0` is the source vertex.
                // Execute DFS for the non-trivial graph.
                let mut search = DFS::from((&g, &0));
                // Collect the vertex in pre-order.
                let order: Vec<_> = search.by_ref().collect();
                // Check visit order.
                assert_eq!(order, [&0, &1, &2, &3, &4, &5, &6, &7]);

                // Check DFS coherence.
                assert_eq!(search.discovery_time.len(), 8);
                assert_eq!(search.finish_time.len(), 8);
                assert_eq!(search.predecessor.len(), 7);

                // Check distances.
                assert_eq!(search.discovery_time.get(&0), Some(&0));
                assert_eq!(search.discovery_time.get(&1), Some(&1));
                assert_eq!(search.discovery_time.get(&2), Some(&2));
                assert_eq!(search.finish_time.get(&2), Some(&3));
                assert_eq!(search.finish_time.get(&1), Some(&4));
                assert_eq!(search.discovery_time.get(&3), Some(&5));
                assert_eq!(search.discovery_time.get(&4), Some(&6));
                assert_eq!(search.discovery_time.get(&5), Some(&7));
                assert_eq!(search.discovery_time.get(&6), Some(&8));
                assert_eq!(search.discovery_time.get(&7), Some(&9));
                assert_eq!(search.finish_time.get(&7), Some(&10));
                assert_eq!(search.finish_time.get(&6), Some(&11));
                assert_eq!(search.finish_time.get(&5), Some(&12));
                assert_eq!(search.finish_time.get(&4), Some(&13));
                assert_eq!(search.finish_time.get(&3), Some(&14));
                assert_eq!(search.finish_time.get(&0), Some(&15));

                // Check predecessors.
                assert_eq!(search.predecessor.get(&0), None);
                assert_eq!(search.predecessor.get(&1), Some(&&0));
                assert_eq!(search.predecessor.get(&2), Some(&&1));
                assert_eq!(search.predecessor.get(&3), Some(&&0));
                assert_eq!(search.predecessor.get(&4), Some(&&3));
                assert_eq!(search.predecessor.get(&5), Some(&&4));
                assert_eq!(search.predecessor.get(&6), Some(&&5));
                assert_eq!(search.predecessor.get(&7), Some(&&6));
            }

            #[test]
            #[should_panic]
            fn depth_first_search_tree_should_panic() {
                // Build a null graph.
                let g = $G::<i32>::null();
                DFS::from((&g, &0)).next();
            }

            #[test]
            fn depth_first_search_forest() {
                // Build a null graph.
                let g = $G::<i32>::null();
                // Build a search object.
                let mut search = DFS::new(&g, None, grathe::algorithms::Traversal::Forest);
                // To search on a null graph
                // without a source vertex
                // yields no result.
                assert_eq!(search.next(), None);

                // Build a disconnected graph.
                let g = $G::<i32>::from_edges([(0, 1), (1, 2), (3, 4)]);
                // Build a search object.
                let mut search = DFS::new(&g, None, grathe::algorithms::Traversal::Forest);
                // Collect the vertex in pre-order.
                let order: Vec<_> = search.by_ref().collect();
                // Check visit order.
                assert_eq!(order, [&0, &1, &2, &3, &4]);

                // Check DFS coherence.
                assert_eq!(search.discovery_time.len(), 5);
                assert_eq!(search.finish_time.len(), 5);
                assert_eq!(search.predecessor.len(), 3);

                // Check distances.
                assert_eq!(search.discovery_time.get(&0), Some(&0));
                assert_eq!(search.discovery_time.get(&1), Some(&1));
                assert_eq!(search.discovery_time.get(&2), Some(&2));
                assert_eq!(search.finish_time.get(&2), Some(&3));
                assert_eq!(search.finish_time.get(&1), Some(&4));
                assert_eq!(search.finish_time.get(&0), Some(&5));
                assert_eq!(search.discovery_time.get(&3), Some(&6));
                assert_eq!(search.discovery_time.get(&4), Some(&7));
                assert_eq!(search.finish_time.get(&4), Some(&8));
                assert_eq!(search.finish_time.get(&3), Some(&9));

                // Check predecessors.
                assert_eq!(search.predecessor.get(&0), None);
                assert_eq!(search.predecessor.get(&1), Some(&&0));
                assert_eq!(search.predecessor.get(&2), Some(&&1));
                assert_eq!(search.predecessor.get(&3), None);
                assert_eq!(search.predecessor.get(&4), Some(&&3));
            }

            #[test]
            fn topological_sort() {
                // Build a null graph.
                let g = $G::<i32>::null();
                let mut search = TopologicalSort::from(&g);

                assert_eq!(search.next().transpose().unwrap(), None);

                let g = $G::<i32>::from_edges([
                    (5, 11),
                    (7, 11),
                    (7, 8),
                    (3, 8),
                    (3, 10),
                    (11, 2),
                    (11, 9),
                    (11, 10),
                    (8, 9),
                ]);
                let mut search = TopologicalSort::from(&g);

                assert_eq!(search.next().transpose().unwrap(), Some(&3));
                assert_eq!(search.next().transpose().unwrap(), Some(&5));
                assert_eq!(search.next().transpose().unwrap(), Some(&7));
                assert_eq!(search.next().transpose().unwrap(), Some(&8));
                assert_eq!(search.next().transpose().unwrap(), Some(&11));
                assert_eq!(search.next().transpose().unwrap(), Some(&2));
                assert_eq!(search.next().transpose().unwrap(), Some(&9));
                assert_eq!(search.next().transpose().unwrap(), Some(&10));
                assert_eq!(search.next().transpose().unwrap(), None);
            }

            #[test]
            #[should_panic]
            fn topological_sort_should_panic() {
                let g = $G::<i32>::from_edges([(0, 1), (1, 2), (2, 1)]);
                let mut search = TopologicalSort::from(&g);

                assert_eq!(search.next().transpose().unwrap(), Some(&0));
                assert_eq!(search.next().transpose().unwrap(), None);
            }
        };
    }

    mod adjacency_list {
        use grathe::algorithms::{TopologicalSort, BFS, DFS};
        use grathe::graphs::storages::DirectedAdjacencyList;
        use grathe::traits::{From, Storage};

        generic_tests!(DirectedAdjacencyList);
    }
}
