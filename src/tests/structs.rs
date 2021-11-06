#[cfg(test)]
#[generic_tests::define]
mod tests {
    use crate::structs::AdjacencyListGraph;
    use crate::traits::Graph;
    use all_asserts::*;

    const N: u32 = 1e3 as u32;

    #[test]
    fn eq<T>()
    where
        T: Graph<VID = u32, EID = (u32, u32)>,
    {
        let mut g = T::new();
        let mut h = T::new();

        // Null graphs are equals by definition.
        assert_eq!(g, h); // G = (), H = ()

        // Two graphs are equals if the have the same vertex set and the same edge set.
        g.add_vertex(&0).unwrap();
        assert_ne!(g, h); // G = (0), H = ()

        h.add_vertex(&0).unwrap();
        assert_eq!(g, h); // G = (0), H = (0)

        g.add_edge(&(0, 0)).unwrap();
        assert_ne!(g, h); // G = (0, (0, 0)), H = (0)

        h.add_vertex(&1).unwrap();
        assert_ne!(g, h); // G = (0, (0, 0)), H = (0, 1)
    }

    #[test]
    fn partial_cmp<T>()
    where
        T: Graph<VID = u32, EID = (u32, u32)>,
    {
        let mut g = T::new();
        let h = T::new();

        // Null graphs are equals by definition.
        assert_false!(g < h);
        assert_true!(g <= h);
        assert_false!(g > h);
        assert_true!(g >= h);

        // The null graph is subgraph of every graph.
        g.add_vertex(&0).unwrap();
        assert_false!(g < h);
        assert_false!(g <= h);
        assert_true!(g > h);
        assert_true!(g >= h);
    }

    #[test]
    fn new<T>()
    where
        T: Graph<VID = u32, EID = (u32, u32)>,
    {
        // Test empty new call.
        T::new();
    }

    #[test]
    fn from_order<T>()
    where
        T: Graph<VID = u32, EID = (u32, u32)>,
    {
        let mut g = T::from(0);

        // Test min graph order.
        assert_eq!(g.order(), 0);

        // Test next graph order.
        g = T::from(1);
        assert_eq!(g.order(), 1);

        // Test high graph order.
        g = T::from(N);
        assert_eq!(g.order(), N as usize);
    }

    #[test]
    #[should_panic]
    fn from_order_panic<T>()
    where
        T: Graph<VID = u32, EID = (u32, u32)>,
    {
        // Negative order should panic.
        T::from(-1 as i32);
    }

    #[test]
    fn order<T>()
    where
        T: Graph<VID = u32, EID = (u32, u32)>,
    {
        let mut g = T::new();

        // Test null graph order.
        assert_eq!(g.order(), 0);

        // Test increasing graph order.
        g.add_vertex(&0).unwrap();
        assert_eq!(g.order(), 1);

        // Test decreasing graph order.
        g.del_vertex(&0).unwrap();
        assert_eq!(g.order(), 0);

        // Test high graph order.
        g = T::from(N);
        assert_eq!(g.order(), N as usize);
    }

    #[test]
    fn size<T>()
    where
        T: Graph<VID = u32, EID = (u32, u32)>,
    {
        let mut g = T::new();

        // Test null graph has no size by definition.
        assert_eq!(g.size(), 0);

        // Test increasing size graph.
        g.add_vertex(&0).unwrap();
        g.add_vertex(&1).unwrap();
        g.add_edge(&(0, 1)).unwrap();
        assert_eq!(g.size(), 1);

        // Test decreasing size graph.
        g.del_edge(&(0, 1)).unwrap();
        assert_eq!(g.size(), 0);

        // Test sequence size graph.
        g = T::from(N);
        for i in 0..N {
            g.add_edge(&(0, i)).unwrap();
            assert_eq!(g.size(), (i + 1) as usize);
        }
    }

    #[test]
    fn has_vertex<T>()
    where
        T: Graph<VID = u32, EID = (u32, u32)>,
    {
        let mut g = T::new();

        // Test null graph has no vertex by definition.
        assert_false!(g.has_vertex(&0));

        // Test add first vertex.
        g.add_vertex(&0).unwrap();
        assert_true!(g.has_vertex(&0));

        // Test del first vertex.
        g.del_vertex(&0).unwrap();
        assert_false!(g.has_vertex(&0));

        // Test sequence of vertices.
        g = T::from(N);
        assert_true!((0..N).all(|i| g.has_vertex(&i)));
    }

    #[test]
    fn add_vertex<T>()
    where
        T: Graph<VID = u32, EID = (u32, u32)>,
    {
        let mut g = T::new();

        // Add min VID.
        assert_false!(g.add_vertex(&T::VID::MIN).is_err());
        assert_true!(g.has_vertex(&T::VID::MIN));

        // Test double addition.
        assert_true!(g.add_vertex(&T::VID::MIN).is_err());

        // Add contiguous VID.
        assert_false!(g.add_vertex(&1).is_err());
        assert_true!(g.has_vertex(&1));

        // Add non contiguous VID.
        assert_false!(g.add_vertex(&N).is_err());
        assert_true!(g.has_vertex(&N));

        // Add max VID.
        assert_false!(g.add_vertex(&T::VID::MAX).is_err());
        assert_true!(g.has_vertex(&T::VID::MAX));
    }

    #[test]
    fn del_vertex<T>()
    where
        T: Graph<VID = u32, EID = (u32, u32)>,
    {
        let mut g = T::new();

        // Del min VID.
        g.add_vertex(&T::VID::MIN).unwrap();
        assert_false!(g.del_vertex(&T::VID::MIN).is_err());
        assert_false!(g.has_vertex(&T::VID::MIN));

        // Test double deletion.
        assert_true!(g.del_vertex(&T::VID::MIN).is_err());

        // Del contiguous VID.
        g.add_vertex(&1).unwrap();
        assert_false!(g.del_vertex(&1).is_err());
        assert_false!(g.has_vertex(&1));

        // Del non contiguous VID.
        g.add_vertex(&N).unwrap();
        assert_false!(g.del_vertex(&N).is_err());
        assert_false!(g.has_vertex(&N));

        // Del max VID.
        g.add_vertex(&T::VID::MAX).unwrap();
        assert_false!(g.del_vertex(&T::VID::MAX).is_err());
        assert_false!(g.has_vertex(&T::VID::MAX));

        // Del vertex and associated edges.
        g.add_vertex(&N).unwrap();
        g.add_edge(&(N, N)).unwrap();
        assert_false!(g.del_vertex(&N).is_err());
        assert_true!(g.has_edge(&(N, N)).is_err());
    }

    #[test]
    fn has_edge<T>()
    where
        T: Graph<VID = u32, EID = (u32, u32)>,
    {
        let mut g = T::new();

        // Test null graph has no edge by definition.
        assert_true!(g.has_edge(&(0, 0)).is_err());

        // Test add first edge.
        g.add_vertex(&0).unwrap();
        g.add_edge(&(0, 0)).unwrap();
        assert_true!(g.has_edge(&(0, 0)).unwrap());

        // Test del first edge.
        g.del_edge(&(0, 0)).unwrap();
        assert_false!(g.has_edge(&(0, 0)).unwrap());

        // Test sequence of edges.
        g = T::from(N);
        for i in 0..N {
            g.add_edge(&(0, i)).unwrap();
        }
        assert_true!((0..N).all(|i| g.has_edge(&(0, i)).unwrap()));
    }

    #[test]
    fn add_edge<T>()
    where
        T: Graph<VID = u32, EID = (u32, u32)>,
    {
        let mut g = T::new();

        // Test missing vertex.
        let mut e = (0, 0);
        assert_true!(g.add_edge(&e).is_err());

        g.add_vertex(&T::VID::MIN).unwrap();
        g.add_vertex(&(T::VID::MIN + 1)).unwrap();
        g.add_vertex(&N).unwrap();
        g.add_vertex(&T::VID::MAX).unwrap();

        // Add min EID.
        e = (T::VID::MIN, T::VID::MIN);
        assert_false!(g.add_edge(&e).is_err());
        assert_true!(g.has_edge(&e).unwrap());

        // Test double addition.
        assert_true!(g.add_edge(&e).is_err());

        // Add contiguous EID.
        e = (T::VID::MIN, T::VID::MIN + 1);
        assert_false!(g.add_edge(&e).is_err());
        assert_true!(g.has_edge(&e).unwrap());

        // Add non contiguous EID.
        e = (N, N);
        assert_false!(g.add_edge(&e).is_err());
        assert_true!(g.has_edge(&e).unwrap());

        // Add max VID.
        e = (T::VID::MAX, T::VID::MAX);
        assert_false!(g.add_edge(&e).is_err());
        assert_true!(g.has_edge(&e).unwrap());
    }

    #[test]
    fn del_edge<T>()
    where
        T: Graph<VID = u32, EID = (u32, u32)>,
    {
        let mut g = T::new();

        // Test missing vertex.
        let mut e = (0, 0);
        assert_true!(g.del_edge(&e).is_err());

        g.add_vertex(&T::VID::MIN).unwrap();
        g.add_vertex(&(T::VID::MIN + 1)).unwrap();
        g.add_vertex(&N).unwrap();
        g.add_vertex(&T::VID::MAX).unwrap();

        // Del min EID.
        e = (T::VID::MIN, T::VID::MIN);
        g.add_edge(&e).unwrap();
        assert_false!(g.del_edge(&e).is_err());
        assert_false!(g.has_edge(&e).unwrap());

        // Test double deletion.
        assert_true!(g.del_edge(&e).is_err());

        // Del contiguous EID.
        e = (T::VID::MIN, T::VID::MIN + 1);
        g.add_edge(&e).unwrap();
        assert_false!(g.del_edge(&e).is_err());
        assert_false!(g.has_edge(&e).unwrap());

        // Del non contiguous EID.
        e = (N, N);
        g.add_edge(&e).unwrap();
        assert_false!(g.del_edge(&e).is_err());
        assert_false!(g.has_edge(&e).unwrap());

        // Del max VID.
        e = (T::VID::MAX, T::VID::MAX);
        g.add_edge(&e).unwrap();
        assert_false!(g.del_edge(&e).is_err());
        assert_false!(g.has_edge(&e).unwrap());
    }

    #[instantiate_tests(<AdjacencyListGraph<u32>>)]
    mod adjacency_list_graph {}
}
