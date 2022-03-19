#[cfg(test)]
#[generic_tests::define]
mod undirected {
    use crate::graphs::storages::UndirectedAdjacencyList;
    use crate::traits::Storage;
    use crate::types::Error;
    use crate::{Adj, E, V};
    use all_asserts::*;

    const N: i32 = 1e3 as i32;
    const E: [(i32, i32); 5] = [(4, 3), (0, 1), (2, 3), (5, 6), (7, 2)];

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
    // FIXME:
    fn eq<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::null();
        let mut h = G::null();

        // Null graphs are equals by definition.
        assert_eq!(g, h); // G = (), H = ()

        // Two graphs are equals if the have the same vertex set and the same edge set.
        let i = g.add_vertex(0)?;
        assert_ne!(g, h); // G = (0), H = ()

        h.add_vertex(0)?;
        assert_eq!(g, h); // G = (0), H = (0)

        g.add_edge(&i, &i)?;
        assert_ne!(g, h); // G = (0, (0, 0)), H = (0)

        h.add_vertex(1)?;
        assert_ne!(g, h); // G = (0, (0, 0)), H = (0, 1)

        Ok(())
    }

    #[test]
    // FIXME:
    fn partial_cmp<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::null();
        let mut h = G::null();

        // Null graphs are equals by definition.
        assert_true!(g == h);
        assert_false!(g < h);
        assert_true!(g <= h);
        assert_false!(g > h);
        assert_true!(g >= h);

        // The null graph is subgraph of every graph.
        g.add_vertex(0)?;
        assert_false!(g == h);
        assert_false!(g < h);
        assert_false!(g <= h);
        assert_true!(g > h);
        assert_true!(g >= h);

        // Checks for equal graphs.
        h.add_vertex(0)?;
        assert_true!(g == h);
        assert_false!(g < h);
        assert_true!(g <= h);
        assert_false!(g > h);
        assert_true!(g >= h);

        // Checks for sub- graphs.
        g.add_vertex(1)?;
        g.add_edge(&0, &1)?;
        assert_false!(g == h);
        assert_false!(g < h);
        assert_false!(g <= h);
        assert_true!(g > h);
        assert_true!(g >= h);

        // Checks for sub- graphs.
        h.add_vertex(1)?;
        assert_false!(g == h);
        assert_false!(g < h);
        assert_false!(g <= h);
        assert_true!(g > h);
        assert_true!(g >= h);

        // Checks for non comparable graphs.
        h.add_vertex(2)?;
        assert_false!(g == h);
        assert_false!(g < h);
        assert_false!(g <= h);
        assert_false!(g > h);
        assert_false!(g >= h);
        assert_true!(matches!(g.partial_cmp(&h), None));

        Ok(())
    }

    #[test]
    // FIXME:
    fn new<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let g = G::new::<_, _, i32>([], []);

        assert_eq!(g.order(), 0);
        assert_eq!(g.size(), 0);

        let g = G::new(0..N, []);

        assert_eq!(g.order(), N as usize);
        assert_eq!(g.size(), 0);

        let g = G::new([], (0..N).zip(0..N));

        assert_eq!(g.order(), N as usize);
        assert_eq!(g.size(), N as usize);

        Ok(())
    }

    #[test]
    // FIXME:
    fn null<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        // Test null new call.
        let g = G::null();

        assert_eq!(g.order(), 0);
        assert_eq!(g.size(), 0);

        Ok(())
    }

    #[test]
    // FIXME:
    fn empty<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::null();

        // Test min graph vertex set.
        assert_eq!(g.order(), 0);

        // Test next graph vertex set.
        g = G::empty([0]);
        assert_eq!(g.order(), 1);

        // Test next graph unordered vertex set.
        g = G::empty([0, 4, 2, 3, 1]);
        assert_eq!(g.order(), 5);

        // Test next graph duplicated vertex set.
        let g = G::empty([0, 4, 2, 3, 1, 4, 3]);
        assert_eq!(g.order(), 5);

        Ok(())
    }

    #[test]
    // FIXME:
    fn complete<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let g = G::complete(0..N);

        assert_eq!(g.order(), N as usize);
        assert_eq!(g.size(), (N * N) as usize);

        Ok(())
    }

    #[test]
    // FIXME:
    fn default<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        // Test default call.
        let g = G::null();
        assert_eq!(g.order(), 0_usize);
        assert_eq!(g.size(), 0_usize);

        Ok(())
    }

    #[test]
    // FIXME:
    fn clear<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::null();

        // Test graph
        g.clear();
        assert_eq!(g.order(), 0);
        assert_eq!(g.size(), 0);

        // Test proper graph
        let i = g.add_vertex(0)?;
        let j = g.add_vertex(1)?;
        g.add_edge(&i, &j)?;
        g.clear();
        assert_eq!(g.order(), 0);
        assert_eq!(g.size(), 0);

        Ok(())
    }
    #[test]
    // FIXME:
    fn from_edges<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::null();

        // Test min graph vertex set.
        assert_eq!(g.size(), 0);

        // Test next graph vertex set.
        g = G::new([], [(0, 0)]);
        assert_eq!(g.size(), 1);

        // Test next graph unordered vertex set.
        g = G::new([], E);
        assert_eq!(g.size(), 5);

        // Test high graph vertex set.
        // g = G::new([], (0..N).zip(0..N));
        // assert_eq!(g.size(), &N);

        // Test next graph duplicated vertex set.
        let g = G::new([], E);
        assert_eq!(g.size(), 5);

        Ok(())
    }

    #[test]
    // FIXME:
    fn vertices_iter<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::empty([0]);
        assert_eq!(V!(g).count(), 1);

        g = G::empty(0..N);
        assert_eq!(V!(g).count(), N as usize);

        assert_true!(V!(g).eq(g.vertices_iter()));
        assert_true!(V!(g).all(|x| g.has_vertex(x)));
        assert_true!(is_sorted(V!(g)));

        // Check iterator size hint.
        let (lower, upper) = V!(g).size_hint();
        assert_eq!(lower, N as usize);
        assert_eq!(upper.unwrap(), N as usize);

        Ok(())
    }

    #[test]
    // FIXME:
    fn edges_iter<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::empty([0]);
        assert_eq!(E!(g).count(), 0);

        g = G::empty(0..N);
        g.add_edge(&1, &1)?;
        g.add_edge(&0, &1)?;
        g.add_edge(&0, &0)?;
        assert_eq!(E!(g).count(), 3);

        assert_true!(E!(g).eq(g.edges_iter()));
        assert_true!(E!(g).all(|(&x, &y)| g.has_edge(&x, &y).unwrap()));
        assert_true!(is_sorted(E!(g)));

        // Check iterator size hint.
        let (lower, _) = E!(g).size_hint();
        assert_eq!(lower, 3);

        Ok(())
    }

    #[test]
    // FIXME:
    fn adjacents_iter<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::empty([0]);
        assert_eq!(Adj!(g, &0).count(), 0);

        g = G::empty(0..N);
        g.add_edge(&1, &1)?;
        g.add_edge(&0, &1)?;
        g.add_edge(&0, &0)?;
        assert_eq!(Adj!(g, &0).count(), 2);

        assert_true!(Adj!(g, &0).eq(g.adjacents_iter(&0)));
        assert_true!(Adj!(g, &0).all(|&x| g.has_edge(&0, &x).unwrap()));
        assert_true!(is_sorted(Adj!(g, &0)));

        Ok(())
    }

    #[test]
    #[should_panic]
    fn adjacents_iter_should_panic<G>()
    where
        G: Storage<Vertex = i32>,
    {
        let g = G::null();
        Adj!(g, &0);
    }

    #[test]
    // FIXME:
    fn order<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::null();

        // Test null graph order.
        assert_eq!(g.order(), 0);

        // Test increasing graph order.
        let i = g.add_vertex(0)?;
        assert_eq!(g.order(), 1);

        // Test decreasing graph order.
        g.del_vertex(&i)?;
        assert_eq!(g.order(), 0);

        // Test high graph order.
        g = G::empty(0..N);
        assert_eq!(g.order(), N as usize);

        Ok(())
    }

    #[test]
    // FIXME:
    fn size<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::null();

        // Test null graph has no size by definition.
        assert_eq!(g.size(), 0);

        // Test increasing size graph.
        let i = g.add_vertex(0)?;
        let j = g.add_vertex(1)?;
        g.add_edge(&i, &j)?;
        assert_eq!(g.size(), 1);

        // Test decreasing size graph.
        g.del_edge(&i, &j)?;
        assert_eq!(g.size(), 0);

        // Test sequence size graph.
        g = G::empty(0..N);
        for i in 0..N {
            g.add_edge(&0, &i)?;
            assert_eq!(g.size(), (i + 1) as usize);
        }
        Ok(())
    }

    #[test]
    // FIXME:
    fn has_vertex<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::null();

        // Test null graph has no vertex by definition.
        assert_false!(g.has_vertex(&0));

        // Test add first vertex.
        let i = g.add_vertex(0)?;
        assert_true!(g.has_vertex(&i));

        // Test del first vertex.
        g.del_vertex(&i)?;
        assert_false!(g.has_vertex(&i));

        // Test sequence of vertex.
        g = G::empty(0..N);
        assert_true!((0..N).all(|i| g.has_vertex(&i)));

        Ok(())
    }

    #[test]
    // FIXME:
    fn add_vertex<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        // Add min Vertex.
        let mut g = G::null();

        // Add min Vertex.
        let i = g.add_vertex(G::Vertex::MIN)?;
        assert_true!(g.has_vertex(&i));

        // Test double addition.
        assert_true!(g.add_vertex(i).is_err());

        // Add contiguous Vertex.
        let i = g.add_vertex(1)?;
        assert_true!(g.has_vertex(&i));

        // Add non contiguous Vertex.
        let i = g.add_vertex(N)?;
        assert_true!(g.has_vertex(&i));

        // Add max Vertex.
        let i = g.add_vertex(G::Vertex::MAX)?;
        assert_true!(g.has_vertex(&i));

        // Add contiguous Vertex.
        g = G::empty(0..N);
        assert_eq!(g.add_vertex(N)?, N);

        Ok(())
    }

    #[test]
    // FIXME:
    fn del_vertex<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::null();

        // Del min Vertex.
        let i = g.add_vertex(G::Vertex::MIN)?;
        g.del_vertex(&i)?;
        assert_false!(g.has_vertex(&i));

        // Test double deletion.
        assert_true!(g.del_vertex(&i).is_err());

        // Del contiguous Vertex.
        let i = g.add_vertex(1)?;
        g.del_vertex(&i)?;
        assert_false!(g.has_vertex(&i));

        // Del non contiguous Vertex.
        let i = g.add_vertex(N)?;
        g.del_vertex(&i)?;
        assert_false!(g.has_vertex(&i));

        // Del max Vertex.
        let i = g.add_vertex(G::Vertex::MAX)?;
        g.del_vertex(&i)?;
        assert_false!(g.has_vertex(&i));

        // Del vertex and associated edges.
        let i = g.add_vertex(0)?;
        let j = g.add_vertex(1)?;
        g.add_edge(&i, &j)?;
        g.add_edge(&j, &i)?;
        g.del_vertex(&i)?;
        assert_true!(g.has_edge(&i, &j).is_err());
        assert_true!(g.has_edge(&j, &i).is_err());
        assert_true!(!Adj!(g, &j).any(|&x| x == i));

        Ok(())
    }

    #[test]
    // FIXME:
    fn has_edge<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::null();

        // Test null graph has no edge by definition.
        assert_true!(g.has_edge(&0, &0).is_err());

        // Test add first edge.
        let i = g.add_vertex(0)?;
        g.add_edge(&i, &i)?;
        assert_true!(g.has_edge(&i, &i)?);

        // Test del first edge.
        g.del_edge(&i, &i)?;
        assert_false!(g.has_edge(&i, &i)?);

        // Test sequence of edges.
        g = G::empty(0..N);
        for i in 0..N {
            g.add_edge(&0, &i)?;
        }
        assert_true!((0..N).all(|i| g.has_edge(&0, &i).unwrap()));

        Ok(())
    }

    #[test]
    // FIXME:
    fn add_edge<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::null();

        // Test missing vertex.
        let (i, j) = (0, 0);
        assert_true!(g.add_edge(&i, &j).is_err());

        g.add_vertex(G::Vertex::MIN)?;
        g.add_vertex(G::Vertex::MIN + 1)?;
        g.add_vertex(N)?;
        g.add_vertex(G::Vertex::MAX)?;

        // Add min Edge.
        let (i, j) = (G::Vertex::MIN, G::Vertex::MIN);
        g.add_edge(&i, &j)?;
        assert_true!(g.has_edge(&i, &j)?);

        // Test double addition.
        assert_true!(g.add_edge(&i, &j).is_err());

        // Add contiguous Edge.
        let (i, j) = (G::Vertex::MIN, (G::Vertex::MIN + 1));
        g.add_edge(&i, &j)?;
        assert_true!(g.has_edge(&i, &j)?);

        // Add non contiguous Edge.
        let (i, j) = (N, N);
        g.add_edge(&i, &j)?;
        assert_true!(g.has_edge(&i, &j)?);

        // Add max Vertex.
        let (i, j) = (G::Vertex::MAX, G::Vertex::MAX);
        g.add_edge(&i, &j)?;
        assert_true!(g.has_edge(&i, &j)?);

        Ok(())
    }

    #[test]
    // FIXME:
    fn del_edge<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::null();

        // Test missing vertex.
        let (i, j) = (0, 0);
        assert_true!(g.del_edge(&i, &j).is_err());

        g.add_vertex(G::Vertex::MIN)?;
        g.add_vertex(G::Vertex::MIN + 1)?;
        g.add_vertex(N)?;
        g.add_vertex(G::Vertex::MAX)?;

        // Del min Edge.
        let (i, j) = (G::Vertex::MIN, G::Vertex::MIN);
        g.add_edge(&i, &j)?;
        g.del_edge(&i, &j)?;
        assert_false!(g.has_edge(&i, &j)?);

        // Test double deletion.
        assert_true!(g.del_edge(&i, &j).is_err());

        // Del contiguous Edge.
        let (i, j) = (G::Vertex::MIN, (G::Vertex::MIN + 1));
        g.add_edge(&i, &j)?;
        g.del_edge(&i, &j)?;
        assert_false!(g.has_edge(&i, &j)?);

        // Del non contiguous Edge.
        let (i, j) = (N, N);
        g.add_edge(&i, &j)?;
        g.del_edge(&i, &j)?;
        assert_false!(g.has_edge(&i, &j)?);

        // Del max Vertex.
        let (i, j) = (G::Vertex::MAX, G::Vertex::MAX);
        g.add_edge(&i, &j)?;
        g.del_edge(&i, &j)?;
        assert_false!(g.has_edge(&i, &j)?);

        Ok(())
    }

    #[test]
    // FIXME:
    fn degree_of_and_isolated_pendant<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::null();

        // Test for isolated vertex
        let i = g.add_vertex(0)?;
        assert_eq!(g.degree_of(&i), 0);
        assert_true!(g.is_isolated_vertex(&i));

        // Test for pendant vertex
        let j = g.add_vertex(1)?;
        g.add_edge(&i, &j)?;
        assert_eq!(g.degree_of(&i), 1);
        assert_true!(g.is_pendant_vertex(&i));

        Ok(())
    }

    #[instantiate_tests(<UndirectedAdjacencyList<i32>>)]
    mod undirected_adjacency_list {}
}

#[cfg(test)]
#[generic_tests::define]
mod directed {
    use crate::graphs::storages::DirectedAdjacencyList;
    use crate::traits::Storage;
    use crate::types::Error;
    use crate::{Adj, E, V};
    use all_asserts::*;

    const N: i32 = 1e3 as i32;
    const E: [(i32, i32); 5] = [(4, 3), (0, 1), (2, 3), (5, 6), (7, 2)];

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
    // FIXME:
    fn eq<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::null();
        let mut h = G::null();

        // Null graphs are equals by definition.
        assert_eq!(g, h); // G = (), H = ()

        // Two graphs are equals if the have the same vertex set and the same edge set.
        let i = g.add_vertex(0)?;
        assert_ne!(g, h); // G = (0), H = ()

        h.add_vertex(0)?;
        assert_eq!(g, h); // G = (0), H = (0)

        g.add_edge(&i, &i)?;
        assert_ne!(g, h); // G = (0, (0, 0)), H = (0)

        h.add_vertex(1)?;
        assert_ne!(g, h); // G = (0, (0, 0)), H = (0, 1)

        Ok(())
    }

    #[test]
    // FIXME:
    fn partial_cmp<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::null();
        let mut h = G::null();

        // Null graphs are equals by definition.
        assert_true!(g == h);
        assert_false!(g < h);
        assert_true!(g <= h);
        assert_false!(g > h);
        assert_true!(g >= h);

        // The null graph is subgraph of every graph.
        g.add_vertex(0)?;
        assert_false!(g == h);
        assert_false!(g < h);
        assert_false!(g <= h);
        assert_true!(g > h);
        assert_true!(g >= h);

        // Checks for equal graphs.
        h.add_vertex(0)?;
        assert_true!(g == h);
        assert_false!(g < h);
        assert_true!(g <= h);
        assert_false!(g > h);
        assert_true!(g >= h);

        // Checks for sub- graphs.
        g.add_vertex(1)?;
        g.add_edge(&0, &1)?;
        assert_false!(g == h);
        assert_false!(g < h);
        assert_false!(g <= h);
        assert_true!(g > h);
        assert_true!(g >= h);

        // Checks for sub- graphs.
        h.add_vertex(1)?;
        assert_false!(g == h);
        assert_false!(g < h);
        assert_false!(g <= h);
        assert_true!(g > h);
        assert_true!(g >= h);

        // Checks for non comparable graphs.
        h.add_vertex(2)?;
        assert_false!(g == h);
        assert_false!(g < h);
        assert_false!(g <= h);
        assert_false!(g > h);
        assert_false!(g >= h);
        assert_true!(matches!(g.partial_cmp(&h), None));

        Ok(())
    }

    #[test]
    // FIXME:
    fn new<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let g = G::new::<_, _, i32>([], []);

        assert_eq!(g.order(), 0);
        assert_eq!(g.size(), 0);

        let g = G::new(0..N, []);

        assert_eq!(g.order(), N as usize);
        assert_eq!(g.size(), 0);

        let g = G::new([], (0..N).zip(0..N));

        assert_eq!(g.order(), N as usize);
        assert_eq!(g.size(), N as usize);

        Ok(())
    }

    #[test]
    // FIXME:
    fn null<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        // Test null new call.
        let g = G::null();

        assert_eq!(g.order(), 0);
        assert_eq!(g.size(), 0);

        Ok(())
    }

    #[test]
    // FIXME:
    fn empty<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::null();

        // Test min graph vertex set.
        assert_eq!(g.order(), 0);

        // Test next graph vertex set.
        g = G::empty([0]);
        assert_eq!(g.order(), 1);

        // Test next graph unordered vertex set.
        g = G::empty([0, 4, 2, 3, 1]);
        assert_eq!(g.order(), 5);

        // Test next graph duplicated vertex set.
        let g = G::empty([0, 4, 2, 3, 1, 4, 3]);
        assert_eq!(g.order(), 5);

        Ok(())
    }

    #[test]
    // FIXME:
    fn complete<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let g = G::complete(0..N);

        assert_eq!(g.order(), N as usize);
        assert_eq!(g.size(), (N * N) as usize);

        Ok(())
    }

    #[test]
    // FIXME:
    fn default<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        // Test default call.
        let g = G::null();
        assert_eq!(g.order(), 0_usize);
        assert_eq!(g.size(), 0_usize);

        Ok(())
    }

    #[test]
    // FIXME:
    fn clear<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::null();

        // Test graph
        g.clear();
        assert_eq!(g.order(), 0);
        assert_eq!(g.size(), 0);

        // Test proper graph
        let i = g.add_vertex(0)?;
        let j = g.add_vertex(1)?;
        g.add_edge(&i, &j)?;
        g.clear();
        assert_eq!(g.order(), 0);
        assert_eq!(g.size(), 0);

        Ok(())
    }
    #[test]
    // FIXME:
    fn from_edges<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::null();

        // Test min graph vertex set.
        assert_eq!(g.size(), 0);

        // Test next graph vertex set.
        g = G::new([], [(0, 0)]);
        assert_eq!(g.size(), 1);

        // Test next graph unordered vertex set.
        g = G::new([], E);
        assert_eq!(g.size(), 5);

        // Test high graph vertex set.
        // g = G::new([], (0..N).zip(0..N));
        // assert_eq!(g.size(), &N);

        // Test next graph duplicated vertex set.
        let g = G::new([], E);
        assert_eq!(g.size(), 5);

        Ok(())
    }

    #[test]
    // FIXME:
    fn vertices_iter<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::empty([0]);
        assert_eq!(V!(g).count(), 1);

        g = G::empty(0..N);
        assert_eq!(V!(g).count(), N as usize);

        assert_true!(V!(g).eq(g.vertices_iter()));
        assert_true!(V!(g).all(|x| g.has_vertex(x)));
        assert_true!(is_sorted(V!(g)));

        // Check iterator size hint.
        let (lower, upper) = V!(g).size_hint();
        assert_eq!(lower, N as usize);
        assert_eq!(upper.unwrap(), N as usize);

        Ok(())
    }

    #[test]
    // FIXME:
    fn edges_iter<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::empty([0]);
        assert_eq!(E!(g).count(), 0);

        g = G::empty(0..N);
        g.add_edge(&1, &1)?;
        g.add_edge(&0, &1)?;
        g.add_edge(&0, &0)?;
        assert_eq!(E!(g).count(), 3);

        assert_true!(E!(g).eq(g.edges_iter()));
        assert_true!(E!(g).all(|(&x, &y)| g.has_edge(&x, &y).unwrap()));
        assert_true!(is_sorted(E!(g)));

        // Check iterator size hint.
        let (lower, _) = E!(g).size_hint();
        assert_eq!(lower, 3);

        Ok(())
    }

    #[test]
    // FIXME:
    fn adjacents_iter<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::empty([0]);
        assert_eq!(Adj!(g, &0).count(), 0);

        g = G::empty(0..N);
        g.add_edge(&1, &1)?;
        g.add_edge(&0, &1)?;
        g.add_edge(&0, &0)?;
        assert_eq!(Adj!(g, &0).count(), 2);

        assert_true!(Adj!(g, &0).eq(g.adjacents_iter(&0)));
        assert_true!(Adj!(g, &0).all(|&x| g.has_edge(&0, &x).unwrap()));
        assert_true!(is_sorted(Adj!(g, &0)));

        Ok(())
    }

    #[test]
    #[should_panic]
    fn adjacents_iter_should_panic<G>()
    where
        G: Storage<Vertex = i32>,
    {
        let g = G::null();
        Adj!(g, &0);
    }

    #[test]
    // FIXME:
    fn order<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::null();

        // Test null graph order.
        assert_eq!(g.order(), 0);

        // Test increasing graph order.
        let i = g.add_vertex(0)?;
        assert_eq!(g.order(), 1);

        // Test decreasing graph order.
        g.del_vertex(&i)?;
        assert_eq!(g.order(), 0);

        // Test high graph order.
        g = G::empty(0..N);
        assert_eq!(g.order(), N as usize);

        Ok(())
    }

    #[test]
    // FIXME:
    fn size<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::null();

        // Test null graph has no size by definition.
        assert_eq!(g.size(), 0);

        // Test increasing size graph.
        let i = g.add_vertex(0)?;
        let j = g.add_vertex(1)?;
        g.add_edge(&i, &j)?;
        assert_eq!(g.size(), 1);

        // Test decreasing size graph.
        g.del_edge(&i, &j)?;
        assert_eq!(g.size(), 0);

        // Test sequence size graph.
        g = G::empty(0..N);
        for i in 0..N {
            g.add_edge(&0, &i)?;
            assert_eq!(g.size(), (i + 1) as usize);
        }
        Ok(())
    }

    #[test]
    // FIXME:
    fn has_vertex<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::null();

        // Test null graph has no vertex by definition.
        assert_false!(g.has_vertex(&0));

        // Test add first vertex.
        let i = g.add_vertex(0)?;
        assert_true!(g.has_vertex(&i));

        // Test del first vertex.
        g.del_vertex(&i)?;
        assert_false!(g.has_vertex(&i));

        // Test sequence of vertex.
        g = G::empty(0..N);
        assert_true!((0..N).all(|i| g.has_vertex(&i)));

        Ok(())
    }

    #[test]
    // FIXME:
    fn add_vertex<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        // Add min Vertex.
        let mut g = G::null();

        // Add min Vertex.
        let i = g.add_vertex(G::Vertex::MIN)?;
        assert_true!(g.has_vertex(&i));

        // Test double addition.
        assert_true!(g.add_vertex(i).is_err());

        // Add contiguous Vertex.
        let i = g.add_vertex(1)?;
        assert_true!(g.has_vertex(&i));

        // Add non contiguous Vertex.
        let i = g.add_vertex(N)?;
        assert_true!(g.has_vertex(&i));

        // Add max Vertex.
        let i = g.add_vertex(G::Vertex::MAX)?;
        assert_true!(g.has_vertex(&i));

        // Add contiguous Vertex.
        g = G::empty(0..N);
        assert_eq!(g.add_vertex(N)?, N);

        Ok(())
    }

    #[test]
    // FIXME:
    fn del_vertex<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::null();

        // Del min Vertex.
        let i = g.add_vertex(G::Vertex::MIN)?;
        g.del_vertex(&i)?;
        assert_false!(g.has_vertex(&i));

        // Test double deletion.
        assert_true!(g.del_vertex(&i).is_err());

        // Del contiguous Vertex.
        let i = g.add_vertex(1)?;
        g.del_vertex(&i)?;
        assert_false!(g.has_vertex(&i));

        // Del non contiguous Vertex.
        let i = g.add_vertex(N)?;
        g.del_vertex(&i)?;
        assert_false!(g.has_vertex(&i));

        // Del max Vertex.
        let i = g.add_vertex(G::Vertex::MAX)?;
        g.del_vertex(&i)?;
        assert_false!(g.has_vertex(&i));

        // Del vertex and associated edges.
        let i = g.add_vertex(0)?;
        let j = g.add_vertex(1)?;
        g.add_edge(&i, &j)?;
        g.add_edge(&j, &i)?;
        g.del_vertex(&i)?;
        assert_true!(g.has_edge(&i, &j).is_err());
        assert_true!(g.has_edge(&j, &i).is_err());
        assert_true!(!Adj!(g, &j).any(|&x| x == i));

        Ok(())
    }

    #[test]
    // FIXME:
    fn has_edge<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::null();

        // Test null graph has no edge by definition.
        assert_true!(g.has_edge(&0, &0).is_err());

        // Test add first edge.
        let i = g.add_vertex(0)?;
        g.add_edge(&i, &i)?;
        assert_true!(g.has_edge(&i, &i)?);

        // Test del first edge.
        g.del_edge(&i, &i)?;
        assert_false!(g.has_edge(&i, &i)?);

        // Test sequence of edges.
        g = G::empty(0..N);
        for i in 0..N {
            g.add_edge(&0, &i)?;
        }
        assert_true!((0..N).all(|i| g.has_edge(&0, &i).unwrap()));

        Ok(())
    }

    #[test]
    // FIXME:
    fn add_edge<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::null();

        // Test missing vertex.
        let (i, j) = (0, 0);
        assert_true!(g.add_edge(&i, &j).is_err());

        g.add_vertex(G::Vertex::MIN)?;
        g.add_vertex(G::Vertex::MIN + 1)?;
        g.add_vertex(N)?;
        g.add_vertex(G::Vertex::MAX)?;

        // Add min Edge.
        let (i, j) = (G::Vertex::MIN, G::Vertex::MIN);
        g.add_edge(&i, &j)?;
        assert_true!(g.has_edge(&i, &j)?);

        // Test double addition.
        assert_true!(g.add_edge(&i, &j).is_err());

        // Add contiguous Edge.
        let (i, j) = (G::Vertex::MIN, (G::Vertex::MIN + 1));
        g.add_edge(&i, &j)?;
        assert_true!(g.has_edge(&i, &j)?);

        // Add non contiguous Edge.
        let (i, j) = (N, N);
        g.add_edge(&i, &j)?;
        assert_true!(g.has_edge(&i, &j)?);

        // Add max Vertex.
        let (i, j) = (G::Vertex::MAX, G::Vertex::MAX);
        g.add_edge(&i, &j)?;
        assert_true!(g.has_edge(&i, &j)?);

        Ok(())
    }

    #[test]
    // FIXME:
    fn del_edge<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::null();

        // Test missing vertex.
        let (i, j) = (0, 0);
        assert_true!(g.del_edge(&i, &j).is_err());

        g.add_vertex(G::Vertex::MIN)?;
        g.add_vertex(G::Vertex::MIN + 1)?;
        g.add_vertex(N)?;
        g.add_vertex(G::Vertex::MAX)?;

        // Del min Edge.
        let (i, j) = (G::Vertex::MIN, G::Vertex::MIN);
        g.add_edge(&i, &j)?;
        g.del_edge(&i, &j)?;
        assert_false!(g.has_edge(&i, &j)?);

        // Test double deletion.
        assert_true!(g.del_edge(&i, &j).is_err());

        // Del contiguous Edge.
        let (i, j) = (G::Vertex::MIN, (G::Vertex::MIN + 1));
        g.add_edge(&i, &j)?;
        g.del_edge(&i, &j)?;
        assert_false!(g.has_edge(&i, &j)?);

        // Del non contiguous Edge.
        let (i, j) = (N, N);
        g.add_edge(&i, &j)?;
        g.del_edge(&i, &j)?;
        assert_false!(g.has_edge(&i, &j)?);

        // Del max Vertex.
        let (i, j) = (G::Vertex::MAX, G::Vertex::MAX);
        g.add_edge(&i, &j)?;
        g.del_edge(&i, &j)?;
        assert_false!(g.has_edge(&i, &j)?);

        Ok(())
    }

    #[test]
    // FIXME:
    fn degree_of_and_isolated_pendant<G>() -> Result<(), Error<i32>>
    where
        G: Storage<Vertex = i32>,
    {
        let mut g = G::null();

        // Test for isolated vertex
        let i = g.add_vertex(0)?;
        assert_eq!(g.degree_of(&i), 0);
        assert_true!(g.is_isolated_vertex(&i));

        // Test for pendant vertex
        let j = g.add_vertex(1)?;
        g.add_edge(&i, &j)?;
        assert_eq!(g.degree_of(&i), 1);
        assert_true!(g.is_pendant_vertex(&i));

        Ok(())
    }

    #[instantiate_tests(<DirectedAdjacencyList<i32>>)]
    mod directed_adjacency_list {}
}
