#[cfg(test)]
#[generic_tests::define]
mod tests {
    use crate::graphs::{AdjacencyListGraph, GraphTrait};
    use crate::types::*;
    use crate::{E, V};
    use all_asserts::*;

    const N: u32 = 1e3 as u32;
    const E: [(u32, u32); 5] = [(4, 3), (0, 1), (2, 3), (5, 6), (7, 2)];

    // TODO: Replace with is_sorted method on iterators once stable.
    fn is_sorted<I>(data: I) -> bool
    where
        I: IntoIterator,
        I::Item: Ord,
    {
        let mut it = data.into_iter();
        match it.next() {
            None => true,
            Some(first) => it
                .scan(first, |state, next| {
                    let cmp = *state <= next;
                    *state = next;
                    Some(cmp)
                })
                .all(|b| b),
        }
    }

    #[test]
    fn eq<T>()
    where
        T: GraphTrait<Vertex = u32>,
    {
        let mut g = T::new();
        let mut h = T::new();

        // Null graphs are equals by definition.
        assert_eq!(g, h); // G = (), H = ()

        // Two graphs are equals if the have the same vertex set and the same edge set.
        g.add_vertex(&0);
        assert_ne!(g, h); // G = (0), H = ()

        h.add_vertex(&0);
        assert_eq!(g, h); // G = (0), H = (0)

        g.add_edge(&(0, 0));
        assert_ne!(g, h); // G = (0, (0, 0)), H = (0)

        h.add_vertex(&1);
        assert_ne!(g, h); // G = (0, (0, 0)), H = (0, 1)
    }

    #[test]
    fn partial_cmp<T>()
    where
        T: GraphTrait<Vertex = u32>,
    {
        let mut g = T::new();
        let h = T::new();

        // Null graphs are equals by definition.
        assert_false!(g < h);
        assert_true!(g <= h);
        assert_false!(g > h);
        assert_true!(g >= h);

        // The null graph is subgraph of every graph.
        g.add_vertex(&0);
        assert_false!(g < h);
        assert_false!(g <= h);
        assert_true!(g > h);
        assert_true!(g >= h);
    }

    #[test]
    fn new<T>()
    where
        T: GraphTrait<Vertex = u32>,
    {
        // Test empty new call.
        T::new();
    }

    #[test]
    fn default<T>()
    where
        T: GraphTrait,
    {
        // Test default call.
        let g = T::default();
        assert_eq!(g.order(), 0 as usize);
        assert_eq!(g.size(), 0 as usize);
    }

    #[test]
    fn from_order<T>()
    where
        T: GraphTrait<Vertex = u32>,
    {
        let mut g = T::from_order(0);

        // Test min graph order.
        assert_eq!(g.order(), 0);

        // Test next graph order.
        g = T::from_order(1);
        assert_eq!(g.order(), 1);

        // Test high graph order.
        g = T::from_order(N as usize);
        assert_eq!(g.order(), N as usize);
    }

    #[test]
    fn from_vertices<T>()
    where
        T: GraphTrait<Vertex = u32>,
    {
        let mut g = T::from_vertices([]);

        // Test min graph vertex set.
        assert_eq!(g.order(), 0);

        // Test next graph vertex set.
        g = T::from_vertices([0]);
        assert_eq!(g.order(), 1);

        // Test next graph unordered vertex set.
        g = T::from_vertices([0, 4, 2, 3, 1]);
        assert_eq!(g.order(), 5);

        // Test high graph vertex set.
        g = T::from_vertices(0..N);
        assert_eq!(g.order(), N as usize);
    }

    #[test]
    #[should_panic]
    fn from_vertices_panic<T>()
    where
        T: GraphTrait<Vertex = u32>,
    {
        // Test next graph duplicated vertex set.
        let g = T::from_vertices([0, 4, 2, 3, 1, 4, 3]);
        assert_eq!(g.order(), 5);
    }

    #[test]
    fn from_edges<T>()
    where
        T: GraphTrait<Vertex = u32>,
    {
        let mut g = T::from_edges([]);

        // Test min graph vertex set.
        assert_eq!(g.size(), 0);

        // Test next graph vertex set.
        g = T::from_edges([(0, 0)]);
        assert_eq!(g.size(), 1);

        // Test next graph unordered vertex set.
        g = T::from_edges(E);
        assert_eq!(g.size(), 5);

        // Test high graph vertex set.
        g = T::from_edges((0..N).zip(0..N));
        assert_eq!(g.size(), N as usize);
    }

    #[test]
    #[should_panic]
    fn from_edges_panic<T>()
    where
        T: GraphTrait<Vertex = u32>,
    {
        // Test next graph duplicated vertex set.
        let g = T::from_edges(E);
        assert_eq!(g.order(), 5);
    }

    #[test]
    fn vertices_iter<T>()
    where
        T: GraphTrait,
    {
        let mut g = T::from_order(0);
        assert_eq!(V!(g).count(), 0);

        g = T::from_order(N as usize);
        assert_eq!(V!(g).count(), N as usize);

        assert_true!(V!(g).eq(g.vertices_iter()));
        assert_true!(V!(g).all(|x| g.has_vertex(&x)));
        assert_true!(is_sorted(V!(g)));
    }

    #[test]
    fn edges_iter<T>()
    where
        T: GraphTrait<Vertex = u32>,
    {
        let mut g = T::from_order(0);
        assert_eq!(E!(g).count(), 0);

        g = T::from_order(N as usize);
        g.add_edge(&(1, 1));
        g.add_edge(&(0, 1));
        g.add_edge(&(0, 0));
        assert_eq!(E!(g).count(), 3);

        assert_true!(E!(g).eq(g.edges_iter()));
        assert_true!(E!(g).all(|x| g.has_edge(&x)));
        assert_true!(is_sorted(E!(g)));
    }

    #[test]
    fn as_data<T>()
    where
        T: GraphTrait<Vertex = u32>,
    {
        let g = T::from_edges(E);
        g.as_data();
    }

    #[test]
    fn as_edge_list<T>()
    where
        T: GraphTrait<Vertex = u32>,
    {
        let g = T::from_edges(E);
        assert_eq!(g.as_edge_list(), EdgeList::from(E));
    }

    #[test]
    fn as_adjacency_list<T>()
    where
        T: GraphTrait<Vertex = u32>,
    {
        let g = T::from_edges(E);
        let mut a = AdjacencyList::new();
        for (x, y) in E {
            a.entry(x).or_default().insert(y);
        }
        assert_eq!(g.as_adjacency_list(), a);
    }

    #[test]
    fn as_dense_adjacency_matrix<T>()
    where
        T: GraphTrait<Vertex = u32>,
    {
        let g = T::from_edges(E);
        let mut a = DenseAdjacencyMatrix::zeros(8, 8);
        for (x, y) in E {
            a[(x as usize, y as usize)] = 1;
        }
        assert_eq!(g.as_dense_adjacency_matrix(), a);
    }

    #[test]
    fn as_sparse_adjacency_matrix<T>()
    where
        T: GraphTrait<Vertex = u32>,
    {
        let g = T::from_edges(E);
        let mut a = DenseAdjacencyMatrix::zeros(8, 8);
        for (x, y) in E {
            a[(x as usize, y as usize)] = 1;
        }
        assert_eq!(g.as_sparse_adjacency_matrix(), SparseAdjacencyMatrix::from(&a));
    }

    #[test]
    fn order<T>()
    where
        T: GraphTrait<Vertex = u32>,
    {
        let mut g = T::new();

        // Test null graph order.
        assert_eq!(g.order(), 0);

        // Test increasing graph order.
        g.add_vertex(&0);
        assert_eq!(g.order(), 1);

        // Test decreasing graph order.
        g.del_vertex(&0);
        assert_eq!(g.order(), 0);

        // Test high graph order.
        g = T::from_order(N as usize);
        assert_eq!(g.order(), N as usize);
    }

    #[test]
    fn size<T>()
    where
        T: GraphTrait<Vertex = u32>,
    {
        let mut g = T::new();

        // Test null graph has no size by definition.
        assert_eq!(g.size(), 0);

        // Test increasing size graph.
        g.add_vertex(&0);
        g.add_vertex(&1);
        g.add_edge(&(0, 1));
        assert_eq!(g.size(), 1);

        // Test decreasing size graph.
        g.del_edge(&(0, 1));
        assert_eq!(g.size(), 0);

        // Test sequence size graph.
        g = T::from_order(N as usize);
        for i in 0..N {
            g.add_edge(&(0, i));
            assert_eq!(g.size(), (i + 1) as usize);
        }
    }

    #[test]
    fn has_vertex<T>()
    where
        T: GraphTrait<Vertex = u32>,
    {
        let mut g = T::new();

        // Test null graph has no vertex by definition.
        assert_false!(g.has_vertex(&0));

        // Test add first vertex.
        g.add_vertex(&0);
        assert_true!(g.has_vertex(&0));

        // Test del first vertex.
        g.del_vertex(&0);
        assert_false!(g.has_vertex(&0));

        // Test sequence of vertices.
        g = T::from_order(N as usize);
        assert_true!((0..N).all(|i| g.has_vertex(&i)));
    }

    #[test]
    fn add_vertex<T>()
    where
        T: GraphTrait<Vertex = u32>,
    {
        let mut g = T::new();

        // Add min Vertex.
        g.add_vertex(&T::Vertex::MIN);
        assert_true!(g.has_vertex(&T::Vertex::MIN));

        // Test double addition.
        assert_true!(g.try_add_vertex(&T::Vertex::MIN).is_err());

        // Add contiguous Vertex.
        g.add_vertex(&1);
        assert_true!(g.has_vertex(&1));

        // Add non contiguous Vertex.
        g.add_vertex(&N);
        assert_true!(g.has_vertex(&N));

        // Add max Vertex.
        g.add_vertex(&T::Vertex::MAX);
        assert_true!(g.has_vertex(&T::Vertex::MAX));
    }

    #[test]
    fn del_vertex<T>()
    where
        T: GraphTrait<Vertex = u32>,
    {
        let mut g = T::new();

        // Del min Vertex.
        g.add_vertex(&T::Vertex::MIN);
        g.del_vertex(&T::Vertex::MIN);
        assert_false!(g.has_vertex(&T::Vertex::MIN));

        // Test double deletion.
        assert_true!(g.try_del_vertex(&T::Vertex::MIN).is_err());

        // Del contiguous Vertex.
        g.add_vertex(&1);
        g.del_vertex(&1);
        assert_false!(g.has_vertex(&1));

        // Del non contiguous Vertex.
        g.add_vertex(&N);
        g.del_vertex(&N);
        assert_false!(g.has_vertex(&N));

        // Del max Vertex.
        g.add_vertex(&T::Vertex::MAX);
        g.del_vertex(&T::Vertex::MAX);
        assert_false!(g.has_vertex(&T::Vertex::MAX));

        // Del vertex and associated edges.
        g.add_vertex(&N);
        g.add_edge(&(N, N));
        g.del_vertex(&N);
        assert_true!(g.try_has_edge(&(N, N)).is_err());
    }

    #[test]
    fn has_edge<T>()
    where
        T: GraphTrait<Vertex = u32>,
    {
        let mut g = T::new();

        // Test null graph has no edge by definition.
        assert_true!(g.try_has_edge(&(0, 0)).is_err());

        // Test add first edge.
        g.add_vertex(&0);
        g.add_edge(&(0, 0));
        assert_true!(g.has_edge(&(0, 0)));

        // Test del first edge.
        g.del_edge(&(0, 0));
        assert_false!(g.has_edge(&(0, 0)));

        // Test sequence of edges.
        g = T::from_order(N as usize);
        for i in 0..N {
            g.add_edge(&(0, i));
        }
        assert_true!((0..N).all(|i| g.has_edge(&(0, i))));
    }

    #[test]
    fn add_edge<T>()
    where
        T: GraphTrait<Vertex = u32>,
    {
        let mut g = T::new();

        // Test missing vertex.
        let mut e = (0, 0);
        assert_true!(g.try_add_edge(&e).is_err());

        g.add_vertex(&T::Vertex::MIN);
        g.add_vertex(&(T::Vertex::MIN + 1));
        g.add_vertex(&N);
        g.add_vertex(&T::Vertex::MAX);

        // Add min Edge.
        e = (T::Vertex::MIN, T::Vertex::MIN);
        g.add_edge(&e);
        assert_true!(g.has_edge(&e));

        // Test double addition.
        assert_true!(g.try_add_edge(&e).is_err());

        // Add contiguous Edge.
        e = (T::Vertex::MIN, T::Vertex::MIN + 1);
        g.add_edge(&e);
        assert_true!(g.has_edge(&e));

        // Add non contiguous Edge.
        e = (N, N);
        g.add_edge(&e);
        assert_true!(g.has_edge(&e));

        // Add max Vertex.
        e = (T::Vertex::MAX, T::Vertex::MAX);
        g.add_edge(&e);
        assert_true!(g.has_edge(&e));
    }

    #[test]
    fn del_edge<T>()
    where
        T: GraphTrait<Vertex = u32>,
    {
        let mut g = T::new();

        // Test missing vertex.
        let mut e = (0, 0);
        assert_true!(g.try_del_edge(&e).is_err());

        g.add_vertex(&T::Vertex::MIN);
        g.add_vertex(&(T::Vertex::MIN + 1));
        g.add_vertex(&N);
        g.add_vertex(&T::Vertex::MAX);

        // Del min Edge.
        e = (T::Vertex::MIN, T::Vertex::MIN);
        g.add_edge(&e);
        g.del_edge(&e);
        assert_false!(g.has_edge(&e));

        // Test double deletion.
        assert_true!(g.try_del_edge(&e).is_err());

        // Del contiguous Edge.
        e = (T::Vertex::MIN, T::Vertex::MIN + 1);
        g.add_edge(&e);
        g.del_edge(&e);
        assert_false!(g.has_edge(&e));

        // Del non contiguous Edge.
        e = (N, N);
        g.add_edge(&e);
        g.del_edge(&e);
        assert_false!(g.has_edge(&e));

        // Del max Vertex.
        e = (T::Vertex::MAX, T::Vertex::MAX);
        g.add_edge(&e);
        g.del_edge(&e);
        assert_false!(g.has_edge(&e));
    }

    #[instantiate_tests(<AdjacencyListGraph<u32>>)]
    mod adjacency_list_graph {}
}
