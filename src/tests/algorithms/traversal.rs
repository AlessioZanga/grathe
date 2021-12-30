#[cfg(test)]
#[generic_tests::define]
mod directed_tests {
    use crate::algorithms::{BFS, DFS};
    use crate::directions::DirectedTrait;
    use crate::errors::*;
    use crate::graphs::DirectedAdjacencyListGraph;

    #[test]
    fn breadth_first_search_tree<T>() -> Result<(), Error<i32>>
    where
        T: DirectedTrait<Vertex = i32>,
    {
        // Build a null graph.
        let mut g = T::new();
        let i = g.add_vertex(&0)?;

        // Execute BFS for the trivial graph.
        let search = BFS::from_directed(&g, &i);

        // The BFS on a trivial graph contains only the root ...
        assert_eq!(search.distance.len(), 1);
        // ... and no predecessors by definition.
        assert_eq!(search.predecessor.len(), 0);
        // The root has distance zero ...
        assert_eq!(search.distance.get(&i), Some(&0));
        // ... and no predecessors by definition.
        assert_eq!(search.predecessor.get(&i), None);

        // Add an edge.
        let j = g.add_vertex(&1)?;
        g.add_edge(&i, &j)?;

        // Execute BFS for the non-trivial graph.
        let search = BFS::from_directed(&g, &i);

        // Check BFS coherence.
        assert_eq!(search.distance.len(), 2);
        assert_eq!(search.predecessor.len(), 1);

        // Check distances.
        assert_eq!(search.distance.get(&i), Some(&0));
        assert_eq!(search.distance.get(&j), Some(&1));

        // Check predecessors.
        assert_eq!(search.predecessor.get(&i), None);
        assert_eq!(search.predecessor.get(&j), Some(&&i));

        // Add a disconnected vertex.
        let k = g.add_vertex(&2)?;

        // Execute BFS for the non-connected graph.
        let search = BFS::from_directed(&g, &i);

        // Check BFS coherence.
        assert_eq!(search.distance.len(), 2);
        assert_eq!(search.predecessor.len(), 1);

        // Check distances.
        assert_eq!(search.distance.get(&i), Some(&0));
        assert_eq!(search.distance.get(&j), Some(&1));
        assert_eq!(search.distance.get(&k), None);

        // Check predecessors.
        assert_eq!(search.predecessor.get(&i), None);
        assert_eq!(search.predecessor.get(&j), Some(&&i));
        assert_eq!(search.predecessor.get(&k), None);

        // Build non-trivial graph.
        let g = T::from_edges(&[
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
        let search = BFS::from_directed(&g, &0);

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

        Ok(())
    }

    #[test]
    fn depth_first_search_tree<T>() -> Result<(), Error<i32>>
    where
        T: DirectedTrait<Vertex = i32>,
    {
        // Build a null graph.
        let mut g = T::new();
        let i = g.add_vertex(&0)?;

        // Execute DFS for the trivial graph.
        let search = DFS::from_directed(&g, &i);

        // The DFS on a trivial graph contains only the root ...
        assert_eq!(search.discovery_time.len(), 1);
        assert_eq!(search.finish_time.len(), 1);
        // ... and no predecessors by definition.
        assert_eq!(search.predecessor.len(), 0);
        // The root has discovery time zero ...
        assert_eq!(search.discovery_time.get(&i), Some(&0));
        // ... and finish time one ...
        assert_eq!(search.finish_time.get(&i), Some(&1));
        // ... and no predecessors by definition.
        assert_eq!(search.predecessor.get(&i), None);

        // Add an edge.
        let j = g.add_vertex(&1)?;
        g.add_edge(&i, &j)?;

        // Execute DFS for the non-trivial graph.
        let search = DFS::from_directed(&g, &i);

        // Check DFS coherence.
        assert_eq!(search.discovery_time.len(), 2);
        assert_eq!(search.finish_time.len(), 2);
        assert_eq!(search.predecessor.len(), 1);

        // Check distances.
        assert_eq!(search.discovery_time.get(&i), Some(&0));
        assert_eq!(search.discovery_time.get(&j), Some(&1));
        assert_eq!(search.finish_time.get(&j), Some(&2));
        assert_eq!(search.finish_time.get(&i), Some(&3));

        // Check predecessors.
        assert_eq!(search.predecessor.get(&i), None);
        assert_eq!(search.predecessor.get(&j), Some(&&i));

        // Add a disconnected vertex.
        let k = g.add_vertex(&2)?;

        // Execute DFS for the non-connected graph.
        let search = DFS::from_directed(&g, &i);

        // Check DFS coherence.
        assert_eq!(search.discovery_time.len(), 2);
        assert_eq!(search.finish_time.len(), 2);
        assert_eq!(search.predecessor.len(), 1);

        // Check distances.
        assert_eq!(search.discovery_time.get(&i), Some(&0));
        assert_eq!(search.discovery_time.get(&j), Some(&1));
        assert_eq!(search.finish_time.get(&j), Some(&2));
        assert_eq!(search.finish_time.get(&i), Some(&3));

        assert_eq!(search.discovery_time.get(&k), None);
        assert_eq!(search.finish_time.get(&k), None);

        // Check predecessors.
        assert_eq!(search.predecessor.get(&i), None);
        assert_eq!(search.predecessor.get(&j), Some(&&i));
        assert_eq!(search.predecessor.get(&k), None);

        // Build non-trivial graph.
        let g = T::from_edges(&[
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
        let search = DFS::from_directed(&g, &0);

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

        Ok(())
    }

    #[instantiate_tests(<DirectedAdjacencyListGraph<i32>>)]
    mod directed_adjacency_list_graph {}
}

#[cfg(test)]
#[generic_tests::define]
mod undirected_tests {
    use crate::algorithms::{BFS, DFS};
    use crate::directions::UndirectedTrait;
    use crate::errors::*;
    use crate::graphs::UndirectedAdjacencyListGraph;

    #[test]
    fn breadth_first_search_tree<T>() -> Result<(), Error<i32>>
    where
        T: UndirectedTrait<Vertex = i32>,
    {
        // Build a null graph.
        let mut g = T::new();
        let i = g.add_vertex(&0)?;

        // Execute BFS for the trivial graph.
        let search = BFS::from_undirected(&g, &i);

        // The BFS on a trivial graph contains only the root ...
        assert_eq!(search.distance.len(), 1);
        // ... and no predecessors by definition.
        assert_eq!(search.predecessor.len(), 0);
        // The root has distance zero ...
        assert_eq!(search.distance.get(&i), Some(&0));
        // ... and no predecessors by definition.
        assert_eq!(search.predecessor.get(&i), None);

        // Add an edge.
        let j = g.add_vertex(&1)?;
        g.add_edge(&i, &j)?;

        // Execute BFS for the non-trivial graph.
        let search = BFS::from_undirected(&g, &i);

        // Check BFS coherence.
        assert_eq!(search.distance.len(), 2);
        assert_eq!(search.predecessor.len(), 1);

        // Check distances.
        assert_eq!(search.distance.get(&i), Some(&0));
        assert_eq!(search.distance.get(&j), Some(&1));

        // Check predecessors.
        assert_eq!(search.predecessor.get(&i), None);
        assert_eq!(search.predecessor.get(&j), Some(&&i));

        // Add a disconnected vertex.
        let k = g.add_vertex(&2)?;

        // Execute BFS for the non-connected graph.
        let search = BFS::from_undirected(&g, &i);

        // Check BFS coherence.
        assert_eq!(search.distance.len(), 2);
        assert_eq!(search.predecessor.len(), 1);

        // Check distances.
        assert_eq!(search.distance.get(&i), Some(&0));
        assert_eq!(search.distance.get(&j), Some(&1));
        assert_eq!(search.distance.get(&k), None);

        // Check predecessors.
        assert_eq!(search.predecessor.get(&i), None);
        assert_eq!(search.predecessor.get(&j), Some(&&i));
        assert_eq!(search.predecessor.get(&k), None);

        // Build non-trivial graph.
        let g = T::from_edges(&[
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
        let search = BFS::from_undirected(&g, &0);

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

        Ok(())
    }

    #[test]
    fn depth_first_search_tree<T>() -> Result<(), Error<i32>>
    where
        T: UndirectedTrait<Vertex = i32>,
    {
        // Build a null graph.
        let mut g = T::new();
        let i = g.add_vertex(&0)?;

        // Execute DFS for the trivial graph.
        let search = DFS::from_undirected(&g, &i);

        // The DFS on a trivial graph contains only the root ...
        assert_eq!(search.discovery_time.len(), 1);
        assert_eq!(search.finish_time.len(), 1);
        // ... and no predecessors by definition.
        assert_eq!(search.predecessor.len(), 0);
        // The root has discovery time zero ...
        assert_eq!(search.discovery_time.get(&i), Some(&0));
        // ... and finish time one ...
        assert_eq!(search.finish_time.get(&i), Some(&1));
        // ... and no predecessors by definition.
        assert_eq!(search.predecessor.get(&i), None);

        // Add an edge.
        let j = g.add_vertex(&1)?;
        g.add_edge(&i, &j)?;

        // Execute DFS for the non-trivial graph.
        let search = DFS::from_undirected(&g, &i);

        // Check DFS coherence.
        assert_eq!(search.discovery_time.len(), 2);
        assert_eq!(search.finish_time.len(), 2);
        assert_eq!(search.predecessor.len(), 1);

        // Check distances.
        assert_eq!(search.discovery_time.get(&i), Some(&0));
        assert_eq!(search.discovery_time.get(&j), Some(&1));
        assert_eq!(search.finish_time.get(&j), Some(&2));
        assert_eq!(search.finish_time.get(&i), Some(&3));

        // Check predecessors.
        assert_eq!(search.predecessor.get(&i), None);
        assert_eq!(search.predecessor.get(&j), Some(&&i));

        // Add a disconnected vertex.
        let k = g.add_vertex(&2)?;

        // Execute DFS for the non-connected graph.
        let search = DFS::from_undirected(&g, &i);

        // Check DFS coherence.
        assert_eq!(search.discovery_time.len(), 2);
        assert_eq!(search.finish_time.len(), 2);
        assert_eq!(search.predecessor.len(), 1);

        // Check distances.
        assert_eq!(search.discovery_time.get(&i), Some(&0));
        assert_eq!(search.discovery_time.get(&j), Some(&1));
        assert_eq!(search.finish_time.get(&j), Some(&2));
        assert_eq!(search.finish_time.get(&i), Some(&3));

        assert_eq!(search.discovery_time.get(&k), None);
        assert_eq!(search.finish_time.get(&k), None);

        // Check predecessors.
        assert_eq!(search.predecessor.get(&i), None);
        assert_eq!(search.predecessor.get(&j), Some(&&i));
        assert_eq!(search.predecessor.get(&k), None);

        // Build non-trivial graph.
        let g = T::from_edges(&[
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
        let search = DFS::from_undirected(&g, &0);

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

        Ok(())
    }

    #[instantiate_tests(<UndirectedAdjacencyListGraph<i32>>)]
    mod undirected_adjacency_list_graph {}
}
