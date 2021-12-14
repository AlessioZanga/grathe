#[cfg(test)]
#[generic_tests::define]
mod tests {
    use crate::errors::Error;
    use crate::storages::{AdjacencyList, StorageTrait};
    use crate::types::*;
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
    fn eq<T>() -> Result<(), Error<i32>>
    where
        T: StorageTrait<Vertex = i32>,
    {
        let mut g = T::new();
        let mut h = T::new();

        // Null graphs are equals by definition.
        assert_eq!(g, h); // G = (), H = ()

        // Two graphs are equals if the have the same vertex set and the same edge set.
        let i = g.add_vertex(&0)?;
        assert_ne!(g, h); // G = (0), H = ()

        h.add_vertex(&0)?;
        assert_eq!(g, h); // G = (0), H = (0)

        g.add_edge((i, i))?;
        assert_ne!(g, h); // G = (0, (0, 0)), H = (0)

        h.add_vertex(&1)?;
        assert_ne!(g, h); // G = (0, (0, 0)), H = (0, 1)

        Ok(())
    }

    #[test]
    fn partial_cmp<T>() -> Result<(), Error<i32>>
    where
        T: StorageTrait<Vertex = i32>,
    {
        let mut g = T::new();
        let h = T::new();

        // Null graphs are equals by definition.
        assert_false!(g < h);
        assert_true!(g <= h);
        assert_false!(g > h);
        assert_true!(g >= h);

        // The null graph is subgraph of every graph.
        g.add_vertex(&0)?;
        assert_false!(g < h);
        assert_false!(g <= h);
        assert_true!(g > h);
        assert_true!(g >= h);

        Ok(())
    }

    #[test]
    fn new<T>() -> Result<(), Error<i32>>
    where
        T: StorageTrait<Vertex = i32>,
    {
        // Test empty new call.
        T::new();

        Ok(())
    }

    #[test]
    fn default<T>() -> Result<(), Error<i32>>
    where
        T: StorageTrait<Vertex = i32>,
    {
        // Test default call.
        let g = T::new();
        assert_eq!(g.order(), 0_usize);
        assert_eq!(g.size(), 0_usize);

        Ok(())
    }

    #[test]
    fn clear<T>() -> Result<(), Error<i32>>
    where
        T: StorageTrait<Vertex = i32>,
    {
        let mut g = T::new();

        // Test empty graph
        g.clear();
        assert_eq!(g.order(), 0);
        assert_eq!(g.size(), 0);

        // Test proper graph
        let i = g.add_vertex(&0)?;
        let j = g.add_vertex(&1)?;
        g.add_edge((i, j))?;
        g.clear();
        assert_eq!(g.order(), 0);
        assert_eq!(g.size(), 0);

        Ok(())
    }

    #[test]
    fn with_capacity<T>()
    where
        T: StorageTrait<Vertex = i32>,
    {
        let g = T::with_capacity(3);
        // FIXME: capacity constraints is soft-enforced.
        assert_le!(g.capacity(), 3);

        // The order is still zero.
        assert_eq!(g.order(), 0);

        // The size is still zero.
        assert_eq!(g.size(), 0);
    }

    #[test]
    fn reserve<T>()
    where
        T: StorageTrait<Vertex = i32>,
    {
        let mut g = T::new();

        // Reserve additional capacity.
        g.reserve(3);

        // The order is still zero.
        assert_eq!(g.order(), 0);

        // The size is still zero.
        assert_eq!(g.size(), 0);
    }

    #[test]
    fn from_order<T>() -> Result<(), Error<i32>>
    where
        T: StorageTrait<Vertex = i32>,
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

        Ok(())
    }

    #[test]
    fn from_vertices<T>() -> Result<(), Error<i32>>
    where
        T: StorageTrait<Vertex = i32>,
    {
        let mut g = T::from_vertices::<_, i32>([]);

        // Test min graph vertex set.
        assert_eq!(g.order(), 0);

        // Test next graph vertex set.
        g = T::from_vertices(&[0]);
        assert_eq!(g.order(), 1);

        // Test next graph unordered vertex set.
        g = T::from_vertices(&[0, 4, 2, 3, 1]);
        assert_eq!(g.order(), 5);

        // Test next graph duplicated vertex set.
        let g = T::from_vertices(&[0, 4, 2, 3, 1, 4, 3]);
        assert_eq!(g.order(), 5);

        Ok(())
    }

    #[test]
    fn from_edges<T>() -> Result<(), Error<i32>>
    where
        T: StorageTrait<Vertex = i32>,
    {
        let mut g = T::from_edges::<_, i32>([]);

        // Test min graph vertex set.
        assert_eq!(g.size(), 0);

        // Test next graph vertex set.
        g = T::from_edges(&[(0, 0)]);
        assert_eq!(g.size(), 1);

        // Test next graph unordered vertex set.
        g = T::from_edges(&E);
        assert_eq!(g.size(), 5);

        // Test high graph vertex set.
        // g = T::from_edges(&(0..N).zip(0..N));
        // assert_eq!(g.size(), &N);

        // Test next graph duplicated vertex set.
        let g = T::from_edges(&E);
        assert_eq!(g.size(), 5);

        Ok(())
    }

    #[test]
    fn to_edge_list<T>() -> Result<(), Error<i32>>
    where
        T: StorageTrait<Vertex = i32>,
    {
        let g = T::from_edges(&E);
        assert_eq!(g.to_edge_list(), EdgeList::from(E));

        Ok(())
    }

    #[test]
    fn to_adjacency_list<T>() -> Result<(), Error<i32>>
    where
        T: StorageTrait<Vertex = i32>,
    {
        let g = T::from_edges(&E);
        let mut a = AdjacencyList::new();
        for (x, y) in E {
            a.entry(x).or_default().insert(y);
        }
        assert_eq!(g.to_adjacency_list(), a);

        Ok(())
    }

    #[test]
    fn to_dense_adjacency_matrix<T>() -> Result<(), Error<i32>>
    where
        T: StorageTrait<Vertex = i32>,
    {
        let g = T::from_edges(&E);
        let mut a = DenseAdjacencyMatrix::zeros(8, 8);
        for (x, y) in E {
            a[(x as usize, y as usize)] = 1;
        }
        assert_eq!(g.to_dense_adjacency_matrix(), a);

        Ok(())
    }

    #[test]
    fn to_sparse_adjacency_matrix<T>() -> Result<(), Error<i32>>
    where
        T: StorageTrait<Vertex = i32>,
    {
        let g = T::from_edges(&E);
        let mut a = DenseAdjacencyMatrix::zeros(8, 8);
        for (x, y) in E {
            a[(x as usize, y as usize)] = 1;
        }
        assert_eq!(
            g.to_sparse_adjacency_matrix(),
            SparseAdjacencyMatrix::from(&a)
        );

        Ok(())
    }

    #[test]
    fn vertices_iter<T>() -> Result<(), Error<i32>>
    where
        T: StorageTrait<Vertex = i32>,
    {
        let mut g = T::from_order(0);
        assert_eq!(V!(g).count(), 0);

        g = T::from_order(N as usize);
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
    fn edges_iter<T>() -> Result<(), Error<i32>>
    where
        T: StorageTrait<Vertex = i32>,
    {
        let mut g = T::from_order(0);
        assert_eq!(E!(g).count(), 0);

        g = T::from_order(N as usize);
        g.add_edge((&1, &1))?;
        g.add_edge((&0, &1))?;
        g.add_edge((&0, &0))?;
        assert_eq!(E!(g).count(), 3);

        assert_true!(E!(g).eq(g.edges_iter()));
        assert_true!(E!(g).all(|x| g.has_edge(x).unwrap()));
        assert_true!(is_sorted(E!(g)));

        // Check iterator size hint (optional for edges).
        let (lower, _) = E!(g).size_hint();
        assert_le!(lower, 3);

        Ok(())
    }

    #[test]
    fn adjacents_iter<T>() -> Result<(), Error<i32>>
    where
        T: StorageTrait<Vertex = i32>,
    {
        let mut g = T::from_order(1);
        assert_eq!(Adj!(g, &0)?.count(), 0);

        // Test missing vertex identifier
        assert_true!(Adj!(g, &1).is_err());

        g = T::from_order(N as usize);
        g.add_edge((&1, &1))?;
        g.add_edge((&0, &1))?;
        g.add_edge((&0, &0))?;
        assert_eq!(Adj!(g, &0)?.count(), 2);

        assert_true!(Adj!(g, &0)?.eq(g.adjacents_iter(&0)?));
        assert_true!(Adj!(g, &0)?.all(|x| g.has_edge((&0, x)).unwrap()));
        assert_true!(is_sorted(Adj!(g, &0)?));

        Ok(())
    }

    #[test]
    #[should_panic]
    fn adjacents_iter_panics<T>()
    where
        T: StorageTrait<Vertex = i32>,
    {
        let g = T::from_order(0);
        assert_eq!(Adj!(g, &0).unwrap().count(), 0);
    }

    #[test]
    fn order<T>() -> Result<(), Error<i32>>
    where
        T: StorageTrait<Vertex = i32>,
    {
        let mut g = T::new();

        // Test null graph order.
        assert_eq!(g.order(), 0);

        // Test increasing graph order.
        let i = g.add_vertex(&0)?;
        assert_eq!(g.order(), 1);

        // Test decreasing graph order.
        g.del_vertex(i)?;
        assert_eq!(g.order(), 0);

        // Test high graph order.
        g = T::from_order(N as usize);
        assert_eq!(g.order(), N as usize);

        Ok(())
    }

    #[test]
    fn size<T>() -> Result<(), Error<i32>>
    where
        T: StorageTrait<Vertex = i32>,
    {
        let mut g = T::new();

        // Test null graph has no size by definition.
        assert_eq!(g.size(), 0);

        // Test increasing size graph.
        let i = g.add_vertex(&0)?;
        let j = g.add_vertex(&1)?;
        let e = g.add_edge((i, j))?;
        assert_eq!(g.size(), 1);

        // Test decreasing size graph.
        g.del_edge(e)?;
        assert_eq!(g.size(), 0);

        // Test sequence size graph.
        g = T::from_order(N as usize);
        for i in 0..N {
            g.add_edge((&0, &i))?;
            assert_eq!(g.size(), (i + 1) as usize);
        }
        Ok(())
    }

    #[test]
    fn has_vertex<T>() -> Result<(), Error<i32>>
    where
        T: StorageTrait<Vertex = i32>,
    {
        let mut g = T::new();

        // Test null graph has no vertex by definition.
        assert_false!(g.has_vertex(&0));

        // Test add first vertex.
        let i = g.add_vertex(&0)?;
        assert_true!(g.has_vertex(i));

        // Test del first vertex.
        g.del_vertex(i)?;
        assert_false!(g.has_vertex(i));

        // Test sequence of vertex.
        g = T::from_order(N as usize);
        assert_true!((0..N).all(|i| g.has_vertex(&i)));

        Ok(())
    }

    #[test]
    fn add_vertex<T>() -> Result<(), Error<i32>>
    where
        T: StorageTrait<Vertex = i32>,
    {
        // Add min Vertex.
        let mut g = T::new();

        // Add min Vertex.
        g.add_vertex(&T::Vertex::MIN)?;
        assert_true!(g.has_vertex(&T::Vertex::MIN));

        // Test double addition.
        assert_true!(g.add_vertex(&T::Vertex::MIN).is_err());

        // Add contiguous Vertex.
        g.add_vertex(&1)?;
        assert_true!(g.has_vertex(&1));

        // Add non contiguous Vertex.
        g.add_vertex(&N)?;
        assert_true!(g.has_vertex(&N));

        // Add max Vertex.
        g.add_vertex(&T::Vertex::MAX)?;
        assert_true!(g.has_vertex(&T::Vertex::MAX));

        // Add contiguous Vertex.
        g = T::from_order(N as usize);
        assert_eq!(g.add_vertex(&N).unwrap(), &N);

        Ok(())
    }

    #[test]
    fn extend_vertices<T>() -> Result<(), Error<i32>>
    where
        T: StorageTrait<Vertex = i32>,
    {
        let mut g = T::new();

        // Extend graph with vertices.
        g.extend_vertices(&[0, 3, 1, 2])?;
        assert_eq!(g.order(), 4);
        assert_eq!(g.size(), 0);
        // Extending with existing vertices yields an error.
        assert_true!(g.extend_vertices(&[0]).is_err());
        Ok(())
    }

    #[test]
    fn del_vertex<T>() -> Result<(), Error<i32>>
    where
        T: StorageTrait<Vertex = i32>,
    {
        let mut g = T::new();

        // Del min Vertex.
        g.add_vertex(&T::Vertex::MIN)?;
        g.del_vertex(&T::Vertex::MIN)?;
        assert_false!(g.has_vertex(&T::Vertex::MIN));

        // Test double deletion.
        assert_true!(g.del_vertex(&T::Vertex::MIN).is_err());

        // Del contiguous Vertex.
        g.add_vertex(&1)?;
        g.del_vertex(&1)?;
        assert_false!(g.has_vertex(&1));

        // Del non contiguous Vertex.
        g.add_vertex(&N)?;
        g.del_vertex(&N)?;
        assert_false!(g.has_vertex(&N));

        // Del max Vertex.
        g.add_vertex(&T::Vertex::MAX)?;
        g.del_vertex(&T::Vertex::MAX)?;
        assert_false!(g.has_vertex(&T::Vertex::MAX));

        // Del vertex and associated edges.
        let i = g.add_vertex(&0)?;
        let j = g.add_vertex(&1)?;
        g.add_edge((i, j))?;
        g.add_edge((j, i))?;
        g.del_vertex(i)?;
        assert_true!(g.has_edge((i, j)).is_err());
        assert_true!(g.has_edge((j, i)).is_err());
        assert_true!(Adj!(g, i).is_err());
        assert_true!(!Adj!(g, j)?.any(|x| x == i));

        Ok(())
    }

    #[test]
    fn has_edge<T>() -> Result<(), Error<i32>>
    where
        T: StorageTrait<Vertex = i32>,
    {
        let mut g = T::new();

        // Test null graph has no edge by definition.
        assert_true!(g.has_edge((&0, &0)).is_err());

        // Test add first edge.
        let i = g.add_vertex(&0)?;
        let e = g.add_edge((i, i))?;
        assert_true!(g.has_edge(e)?);

        // Test del first edge.
        let e = g.del_edge(e)?;
        assert_false!(g.has_edge(e)?);

        // Test sequence of edges.
        g = T::from_order(N as usize);
        for i in 0..N {
            g.add_edge((&0, &i))?;
        }
        assert_true!((0..N).all(|i| g.has_edge((&0, &i)).unwrap()));

        Ok(())
    }

    #[test]
    fn add_edge<T>() -> Result<(), Error<i32>>
    where
        T: StorageTrait<Vertex = i32>,
    {
        let mut g = T::new();

        // Test missing vertex.
        let e = (&0, &0);
        assert_true!(g.add_edge(e).is_err());

        g.add_vertex(&T::Vertex::MIN)?;
        g.add_vertex(&(T::Vertex::MIN + 1))?;
        g.add_vertex(&N)?;
        g.add_vertex(&T::Vertex::MAX)?;

        // Add min Edge.
        let e = (&T::Vertex::MIN, &T::Vertex::MIN);
        g.add_edge(e)?;
        assert_true!(g.has_edge(e)?);

        // Test double addition.
        assert_true!(g.add_edge(e).is_err());

        // Add contiguous Edge.
        let e = (&T::Vertex::MIN, &(T::Vertex::MIN + 1));
        g.add_edge(e)?;
        assert_true!(g.has_edge(e)?);

        // Add non contiguous Edge.
        let e = (&N, &N);
        g.add_edge(e)?;
        assert_true!(g.has_edge(e)?);

        // Add max Vertex.
        let e = (&T::Vertex::MAX, &T::Vertex::MAX);
        g.add_edge(e)?;
        assert_true!(g.has_edge(e)?);

        Ok(())
    }

    #[test]
    fn reserve_edge<T>() -> Result<(), Error<i32>>
    where
        T: StorageTrait<Vertex = i32>,
    {
        let mut g = T::new();

        // Test missing edge and vertex.
        let e = g.reserve_edge((&0, &1))?;
        assert_true!(g.has_vertex(e.0));
        assert_true!(g.has_vertex(e.1));
        assert_true!(g.has_edge(e).unwrap());

        // Test already existing vertex
        let i = g.add_vertex(&2)?;
        assert_true!(g.reserve_edge((i, &3)).is_err());
        assert_true!(g.reserve_edge((&3, i)).is_err());

        Ok(())
    }

    #[test]
    fn extend_edges<T>() -> Result<(), Error<i32>>
    where
        T: StorageTrait<Vertex = i32>,
    {
        let mut g = T::new();

        // Extend graph with edges.
        g.extend_edges(&[(0, 3), (1, 2)])?;
        assert_eq!(g.order(), 4);
        assert_eq!(g.size(), 2);

        // Extending with existing edges yields an error.
        assert_true!(g.extend_edges(&[(0, 3)]).is_err());
        Ok(())
    }

    #[test]
    fn del_edge<T>() -> Result<(), Error<i32>>
    where
        T: StorageTrait<Vertex = i32>,
    {
        let mut g = T::new();

        // Test missing vertex.
        let mut e = (&0, &0);
        assert_true!(g.del_edge(e).is_err());

        g.add_vertex(&T::Vertex::MIN)?;
        g.add_vertex(&(T::Vertex::MIN + 1))?;
        g.add_vertex(&N)?;
        g.add_vertex(&T::Vertex::MAX)?;

        // Del min Edge.
        e = (&T::Vertex::MIN, &T::Vertex::MIN);
        g.add_edge(e)?;
        g.del_edge(e)?;
        assert_false!(g.has_edge(e)?);

        // Test double deletion.
        assert_true!(g.del_edge(e).is_err());

        // Del contiguous Edge.
        e = (&T::Vertex::MIN, &(T::Vertex::MIN + 1));
        g.add_edge(e)?;
        g.del_edge(e)?;
        assert_false!(g.has_edge(e)?);

        // Del non contiguous Edge.
        e = (&N, &N);
        g.add_edge(e)?;
        g.del_edge(e)?;
        assert_false!(g.has_edge(e)?);

        // Del max Vertex.
        e = (&T::Vertex::MAX, &T::Vertex::MAX);
        g.add_edge(e)?;
        g.del_edge(e)?;
        assert_false!(g.has_edge(e)?);

        Ok(())
    }

    #[test]
    fn is_subgraph<T>() -> Result<(), Error<i32>>
    where
        T: StorageTrait<Vertex = i32>,
    {
        let g = T::from_edges(&[(0, 1)]);
        let h = T::from_edges(&[(0, 1), (0, 2)]);

        assert_le!(g, h);
        assert_eq!((g <= h), g.is_subgraph(&h));

        Ok(())
    }

    #[test]
    fn is_supergraph<T>() -> Result<(), Error<i32>>
    where
        T: StorageTrait<Vertex = i32>,
    {
        let g = T::from_edges(&[(0, 1), (0, 2)]);
        let h = T::from_edges(&[(0, 1)]);

        assert_ge!(g, h);
        assert_eq!((g >= h), g.is_supergraph(&h));

        Ok(())
    }

    #[test]
    fn degree_of_and_isolated_pendant<T>() -> Result<(), Error<i32>>
    where
        T: StorageTrait<Vertex = i32>,
    {
        let mut g = T::new();

        // Test for missing vertex
        assert_true!(g.degree_of(&0).is_err());

        // Test for isolated vertex
        let i = g.add_vertex(&0)?;
        assert_eq!(g.degree_of(i)?, 0);
        assert_true!(g.is_isolated_vertex(i)?);

        // Test for pendant vertex
        let j = g.add_vertex(&1)?;
        g.add_edge((i, j))?;
        assert_eq!(g.degree_of(i)?, 1);
        assert_true!(g.is_pendant_vertex(i)?);

        Ok(())
    }

    #[instantiate_tests(<AdjacencyList<i32>>)]
    mod adjacency_list_storage {}
}
