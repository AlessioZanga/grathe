#[cfg(test)]
#[generic_tests::define]
mod tests {
    use crate::errors::*;
    use crate::graphs::{AdjacencyListGraph, GraphTrait};
    use crate::types::*;
    use crate::{Adj, E, V};
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
    fn eq<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        let mut g = T::default();
        let mut h = T::default();

        // Null graphs are equals by definition.
        assert_eq!(g, h); // G = (), H = ()

        // Two graphs are equals if the have the same vertex set and the same edge set.
        g.add_vertex(&0)?;
        assert_ne!(g, h); // G = (0), H = ()

        h.add_vertex(&0)?;
        assert_eq!(g, h); // G = (0), H = (0)

        g.add_edge(&(0, 0))?;
        assert_ne!(g, h); // G = (0, (0, 0)), H = (0)

        h.add_vertex(&1)?;
        assert_ne!(g, h); // G = (0, (0, 0)), H = (0, 1)

        Ok(())
    }

    #[test]
    fn partial_cmp<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        let mut g = T::default();
        let h = T::default();

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
    fn new<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        // Test empty new call.
        T::default();

        Ok(())
    }

    #[test]
    fn default<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait,
    {
        // Test default call.
        let g = T::default();
        assert_eq!(g.order(), 0 as usize);
        assert_eq!(g.size(), 0 as usize);

        Ok(())
    }

    #[test]
    fn from_order<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
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
    fn from_vertices<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
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

        Ok(())
    }

    #[test]
    #[should_panic]
    fn from_vertices_panics<T>()
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        // Test next graph duplicated vertex set.
        let g = T::from_vertices([0, 4, 2, 3, 1, 4, 3]);
        assert_eq!(g.order(), 5);
    }

    #[test]
    fn from_edges<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
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

        Ok(())
    }

    #[test]
    #[should_panic]
    fn from_edges_panics<T>()
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        // Test next graph duplicated vertex set.
        let g = T::from_edges(E);
        assert_eq!(g.order(), 5);
    }

    #[test]
    fn vertices_iter<T>() -> Result<(), Error<u32>>
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

        Ok(())
    }

    #[test]
    fn edges_iter<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        let mut g = T::from_order(0);
        assert_eq!(E!(g).count(), 0);

        g = T::from_order(N as usize);
        g.add_edge(&(1, 1))?;
        g.add_edge(&(0, 1))?;
        g.add_edge(&(0, 0))?;
        assert_eq!(E!(g).count(), 3);

        assert_true!(E!(g).eq(g.edges_iter()));
        assert_true!(E!(g).all(|x| g.has_edge(&x).unwrap()));
        assert_true!(is_sorted(E!(g)));

        Ok(())
    }

    #[test]
    fn adjacents_iter<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        let mut g = T::from_order(1);
        assert_eq!(Adj!(g, &0).count(), 0);

        g = T::from_order(N as usize);
        g.add_edge(&(1, 1))?;
        g.add_edge(&(0, 1))?;
        g.add_edge(&(0, 0))?;
        assert_eq!(Adj!(g, &0).count(), 2);

        assert_true!(Adj!(g, &0).eq(g.adjacents_iter(&0)));
        assert_true!(Adj!(g, &0).all(|x| g.has_edge(&(0, x)).unwrap()));
        assert_true!(is_sorted(Adj!(g, &0)));

        Ok(())
    }

    #[test]
    #[should_panic]
    fn adjacents_iter_panics<T>()
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        let g = T::from_order(0);
        assert_eq!(Adj!(g, &0).count(), 0);
    }

    #[test]
    fn as_data<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        let g = T::from_edges(E);
        g.as_data();

        Ok(())
    }

    #[test]
    fn as_vertices_labels<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        let g = T::default();
        g.as_vertices_labels();

        Ok(())
    }

    #[test]
    fn as_mut_vertices_labels<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        let mut g = T::default();
        g.as_mut_vertices_labels();

        Ok(())
    }

    #[test]
    fn as_edges_labels<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        let g = T::default();
        g.as_edges_labels();

        Ok(())
    }

    #[test]
    fn as_mut_edges_labels<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        let mut g = T::default();
        g.as_mut_edges_labels();

        Ok(())
    }

    #[test]
    fn as_edge_list<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        let g = T::from_edges(E);
        assert_eq!(g.as_edge_list(), EdgeList::from(E));

        Ok(())
    }

    #[test]
    fn as_adjacency_list<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        let g = T::from_edges(E);
        let mut a = AdjacencyList::default();
        for (x, y) in E {
            a.entry(x).or_default().insert(y);
        }
        assert_eq!(g.as_adjacency_list(), a);

        Ok(())
    }

    #[test]
    fn as_dense_adjacency_matrix<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        let g = T::from_edges(E);
        let mut a = DenseAdjacencyMatrix::zeros(8, 8);
        for (x, y) in E {
            a[(x as usize, y as usize)] = 1;
        }
        assert_eq!(g.as_dense_adjacency_matrix(), a);

        Ok(())
    }

    #[test]
    fn as_sparse_adjacency_matrix<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        let g = T::from_edges(E);
        let mut a = DenseAdjacencyMatrix::zeros(8, 8);
        for (x, y) in E {
            a[(x as usize, y as usize)] = 1;
        }
        assert_eq!(
            g.as_sparse_adjacency_matrix(),
            SparseAdjacencyMatrix::from(&a)
        );

        Ok(())
    }

    #[test]
    fn order<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        let mut g = T::default();

        // Test null graph order.
        assert_eq!(g.order(), 0);

        // Test increasing graph order.
        g.add_vertex(&0)?;
        assert_eq!(g.order(), 1);

        // Test decreasing graph order.
        g.del_vertex(&0)?;
        assert_eq!(g.order(), 0);

        // Test high graph order.
        g = T::from_order(N as usize);
        assert_eq!(g.order(), N as usize);

        Ok(())
    }

    #[test]
    fn size<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        let mut g = T::default();

        // Test null graph has no size by definition.
        assert_eq!(g.size(), 0);

        // Test increasing size graph.
        g.add_vertex(&0)?;
        g.add_vertex(&1)?;
        g.add_edge(&(0, 1))?;
        assert_eq!(g.size(), 1);

        // Test decreasing size graph.
        g.del_edge(&(0, 1))?;
        assert_eq!(g.size(), 0);

        // Test sequence size graph.
        g = T::from_order(N as usize);
        for i in 0..N {
            g.add_edge(&(0, i))?;
            assert_eq!(g.size(), (i + 1) as usize);
        }
        Ok(())
    }

    #[test]
    fn has_vertex<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        let mut g = T::default();

        // Test null graph has no vertex by definition.
        assert_false!(g.has_vertex(&0));

        // Test add first vertex.
        g.add_vertex(&0)?;
        assert_true!(g.has_vertex(&0));

        // Test del first vertex.
        g.del_vertex(&0)?;
        assert_false!(g.has_vertex(&0));

        // Test sequence of vertices.
        g = T::from_order(N as usize);
        assert_true!((0..N).all(|i| g.has_vertex(&i)));

        Ok(())
    }

    #[test]
    fn reserve_vertex<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        // Add min Vertex.
        let mut g = T::default();
        assert_eq!(g.reserve_vertex().unwrap() as usize, 0);

        // Add contiguous Vertex.
        g = T::from_order(N as usize);
        assert_eq!(g.reserve_vertex().unwrap() as usize, N as usize);

        Ok(())
    }

    #[test]
    #[should_panic]
    fn reserve_vertex_panics<T>()
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        // Panics on overflow.
        let mut g = T::default();
        g.add_vertex(&T::Vertex::MAX).unwrap();
        assert_true!(g.reserve_vertex().is_err());
    }

    #[test]
    fn add_vertex<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        let mut g = T::default();

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

        Ok(())
    }

    #[test]
    fn del_vertex<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        let mut g = T::default();

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
        g.add_vertex(&N)?;
        g.add_edge(&(N, N))?;
        g.del_vertex(&N)?;
        assert_true!(g.has_edge(&(N, N)).is_err());

        Ok(())
    }

    #[test]
    fn has_edge<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        let mut g = T::default();

        // Test null graph has no edge by definition.
        assert_true!(g.has_edge(&(0, 0)).is_err());

        // Test add first edge.
        let i = g.add_vertex(&0)?;
        let e = g.add_edge(&(i, i))?;
        assert_true!(g.has_edge(&e)?);

        // Test del first edge.
        let e = g.del_edge(&e)?;
        assert_false!(g.has_edge(&e)?);

        // Test sequence of edges.
        g = T::from_order(N as usize);
        for i in 0..N {
            g.add_edge(&(0, i))?;
        }
        assert_true!((0..N).all(|i| g.has_edge(&(0, i)).unwrap()));

        Ok(())
    }

    #[test]
    fn add_edge<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        let mut g = T::default();

        // Test missing vertex.
        let mut e = (0, 0);
        assert_true!(g.add_edge(&e).is_err());

        g.add_vertex(&T::Vertex::MIN)?;
        g.add_vertex(&(T::Vertex::MIN + 1))?;
        g.add_vertex(&N)?;
        g.add_vertex(&T::Vertex::MAX)?;

        // Add min Edge.
        e = (T::Vertex::MIN, T::Vertex::MIN);
        g.add_edge(&e)?;
        assert_true!(g.has_edge(&e)?);

        // Test double addition.
        assert_true!(g.add_edge(&e).is_err());

        // Add contiguous Edge.
        e = (T::Vertex::MIN, T::Vertex::MIN + 1);
        g.add_edge(&e)?;
        assert_true!(g.has_edge(&e)?);

        // Add non contiguous Edge.
        e = (N, N);
        g.add_edge(&e)?;
        assert_true!(g.has_edge(&e)?);

        // Add max Vertex.
        e = (T::Vertex::MAX, T::Vertex::MAX);
        g.add_edge(&e)?;
        assert_true!(g.has_edge(&e)?);

        Ok(())
    }

    #[test]
    fn del_edge<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        let mut g = T::default();

        // Test missing vertex.
        let mut e = (0, 0);
        assert_true!(g.del_edge(&e).is_err());

        g.add_vertex(&T::Vertex::MIN)?;
        g.add_vertex(&(T::Vertex::MIN + 1))?;
        g.add_vertex(&N)?;
        g.add_vertex(&T::Vertex::MAX)?;

        // Del min Edge.
        e = (T::Vertex::MIN, T::Vertex::MIN);
        g.add_edge(&e)?;
        g.del_edge(&e)?;
        assert_false!(g.has_edge(&e)?);

        // Test double deletion.
        assert_true!(g.del_edge(&e).is_err());

        // Del contiguous Edge.
        e = (T::Vertex::MIN, T::Vertex::MIN + 1);
        g.add_edge(&e)?;
        g.del_edge(&e)?;
        assert_false!(g.has_edge(&e)?);

        // Del non contiguous Edge.
        e = (N, N);
        g.add_edge(&e)?;
        g.del_edge(&e)?;
        assert_false!(g.has_edge(&e)?);

        // Del max Vertex.
        e = (T::Vertex::MAX, T::Vertex::MAX);
        g.add_edge(&e)?;
        g.del_edge(&e)?;
        assert_false!(g.has_edge(&e)?);

        Ok(())
    }

    #[test]
    fn get_vertex_id<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        // Test for missing vertex label.
        let mut g = T::default();
        assert_true!(g.get_vertex_id("0").is_err());

        // Test for existing vertex label.
        let i = g.add_vertex(&0)?;
        assert_eq!(g.set_vertex_label(&i, "0")?, i);
        assert_eq!(g.get_vertex_id("0")?, i);

        Ok(())
    }

    #[test]
    fn get_vertex_label<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        // Test for missing vertex identifier.
        let mut g = T::default();
        assert_true!(g.get_vertex_label(&0).is_err());

        // Test for existing vertex identifier.
        let i = g.add_vertex(&0)?;
        assert_eq!(g.set_vertex_label(&i, "0")?, i);
        assert_eq!(g.get_vertex_label(&i)?, "0");

        Ok(())
    }

    #[test]
    fn set_vertex_label<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        // Test for missing vertex identifier.
        let mut g = T::default();
        assert_true!(g.set_vertex_label(&0, "0").is_err());

        // Test for existing vertex label.
        let i = g.add_vertex(&0)?;
        assert_false!(g.set_vertex_label(&i, "0").is_err());
        assert_eq!(g.get_vertex_label(&i)?, "0");

        // Test for vertex label overwriting (identifier).
        assert_true!(g.set_vertex_label(&i, "1").is_err());
        assert_eq!(g.get_vertex_label(&i)?, "0");

        // Test for vertex label overwriting (label).
        let j = g.add_vertex(&1)?;
        assert_true!(g.set_vertex_label(&j, "0").is_err());
        assert_true!(g.get_vertex_label(&j).is_err());

        Ok(())
    }

    #[test]
    fn unset_vertex_label<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        // Test for missing vertex identifier.
        let mut g = T::default();
        assert_true!(g.unset_vertex_label(&0).is_err());

        // Test for existing vertex label.
        let i = g.add_vertex(&0)?;
        assert_false!(g.set_vertex_label(&i, "0").is_err());
        assert_false!(g.unset_vertex_label(&i).is_err());

        // Test for vertex label overdeleting (identifier).
        assert_true!(g.unset_vertex_label(&i).is_err());
        assert_true!(g.get_vertex_label(&i).is_err());

        Ok(())
    }

    #[test]
    fn get_edge_id<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        // Test for missing edge label.
        let mut g = T::default();
        assert_true!(g.get_edge_id("(0, 1)").is_err());

        // Test for existing edge label.
        let i = g.add_vertex(&0)?;
        let j = g.add_vertex(&1)?;
        let e = g.add_edge(&(i, j))?;
        assert_eq!(g.set_edge_label(&e, "(0, 1)")?, e);
        assert_eq!(g.get_edge_id("(0, 1)")?, e);

        Ok(())
    }

    #[test]
    fn get_edge_label<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        // Test for missing edge identifier.
        let mut g = T::default();
        assert_true!(g.get_edge_label(&(0, 1)).is_err());

        // Test for existing edge identifier.
        let i = g.add_vertex(&0)?;
        let j = g.add_vertex(&1)?;
        let e = g.add_edge(&(i, j))?;
        assert_eq!(g.set_edge_label(&e, "(0, 1)")?, e);
        assert_eq!(g.get_edge_label(&e)?, "(0, 1)");

        Ok(())
    }

    #[test]
    fn add_vertex_label<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        let mut g = T::default();
        let i = g.add_vertex_label("0")?;
        assert_eq!(i, 0);
        assert_eq!(g.get_vertex_id("0")?, i);
        assert_eq!(g.get_vertex_label(&i)?, "0");

        Ok(())
    }

    #[test]
    fn set_edge_label<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        // Test for missing edge identifier.
        let mut g = T::default();
        let i = g.add_vertex(&0)?;
        let j = g.add_vertex(&1)?;
        assert_true!(g.set_edge_label(&(i, j), "(0, 1)").is_err());

        // Test for existing edge label.
        let e = g.add_edge(&(i, j))?;
        assert_false!(g.set_edge_label(&e, "(0, 1)").is_err());
        assert_eq!(g.get_edge_label(&e)?, "(0, 1)");

        // Test for edge label overwriting (identifier).
        assert_true!(g.set_edge_label(&e, "(0, 1)").is_err());
        assert_eq!(g.get_edge_label(&e)?, "(0, 1)");

        // Test for edge label overwriting (label).
        let k = g.add_vertex(&2)?;
        let f = g.add_edge(&(i, k))?;
        assert_true!(g.set_edge_label(&f, "(0, 1)").is_err());
        assert_true!(g.get_edge_label(&f).is_err());

        Ok(())
    }

    #[test]
    fn unset_edge_label<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        // Test for missing edge identifier.
        let mut g = T::default();
        let i = g.add_vertex(&0)?;
        let j = g.add_vertex(&1)?;
        assert_true!(g.unset_edge_label(&(i, j)).is_err());

        // Test for existing edge label.
        let e = g.add_edge(&(i, j))?;
        assert_false!(g.set_edge_label(&e, "(0, 1)").is_err());
        assert_false!(g.unset_edge_label(&e).is_err());

        // Test for edge label overdeleting (identifier).
        assert_true!(g.unset_edge_label(&e).is_err());
        assert_true!(g.get_edge_label(&e).is_err());

        Ok(())
    }

    #[test]
    fn is_subgraph<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        let g = T::from_edges([(0, 1)]);
        let h = T::from_edges([(0, 1), (0, 2)]);

        assert_le!(g, h);
        assert_eq!((g <= h), g.is_subgraph(&h));

        Ok(())
    }

    #[test]
    fn is_supergraph<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32> + std::fmt::Debug,
    {
        let g = T::from_edges([(0, 1), (0, 2)]);
        let h = T::from_edges([(0, 1)]);

        assert_ge!(g, h);
        assert_eq!((g >= h), g.is_supergraph(&h));

        Ok(())
    }

    #[instantiate_tests(<AdjacencyListGraph<u32>>)]
    mod adjacency_list_graph {}
}
