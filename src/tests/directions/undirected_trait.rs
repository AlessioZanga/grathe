#[cfg(test)]
#[generic_tests::define]
mod tests {
    use crate::directions::UndirectedTrait;
    use crate::errors::Error;
    use crate::graphs::UndirectedAdjacencyListGraph;
    use crate::Ne;
    use all_asserts::*;

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
    fn is_directed<T>()
    where
        T: UndirectedTrait<Vertex = i32>,
    {
        let g = T::new();
        assert_false!(g.is_directed());
    }

    #[test]
    fn is_partially_directed<T>()
    where
        T: UndirectedTrait<Vertex = i32>,
    {
        let g = T::new();
        assert_false!(g.is_partially_directed());
    }

    #[test]
    fn add_edge<T>() -> Result<(), Error<i32>>
    where
        T: UndirectedTrait<Vertex = i32>,
    {
        let mut g = T::new();

        // Test for undirected edges
        let i = g.add_vertex(&0)?;
        let j = g.add_vertex(&1)?;
        g.add_edge(&i, &j)?;
        assert_true!(g.has_edge(&i, &j)?);
        assert_true!(g.has_edge(&j, &i)?);

        // Test for repeated undirected edges addition
        assert_true!(g.add_edge(&i, &j).is_err());

        // Test for loops
        g.add_edge(&i, &i)?;
        assert_true!(g.has_edge(&i, &i)?);
        g.del_edge(&i, &j)?;

        // Del vertex and associated edges.
        g.del_vertex(&i)?;
        assert_true!(g.has_edge(&i, &j).is_err());
        assert_true!(g.has_edge(&j, &i).is_err());
        assert_true!(!Ne!(g, &j).any(|&x| x == i));

        Ok(())
    }

    #[test]
    fn del_edge<T>() -> Result<(), Error<i32>>
    where
        T: UndirectedTrait<Vertex = i32>,
    {
        let mut g = T::new();

        // Test for undirected edges
        let i = g.add_vertex(&0)?;
        let j = g.add_vertex(&1)?;
        g.add_edge(&i, &j)?;
        assert_true!(g.has_edge(&i, &j)?);
        assert_true!(g.has_edge(&j, &i)?);
        assert_false!(g.del_edge(&i, &j).is_err());

        // Test for repeated undirected edges deletion
        assert_true!(g.del_edge(&i, &j).is_err());
        assert_false!(g.has_edge(&i, &j)?);
        assert_false!(g.has_edge(&j, &i)?);

        Ok(())
    }

    #[test]
    fn neighbors_iter<T>() -> Result<(), Error<i32>>
    where
        T: UndirectedTrait<Vertex = i32>,
    {
        let mut g = T::new();

        // Test for existing vertex
        let i = g.add_vertex(&0)?;
        assert_eq!(Ne!(g, &i).count(), 0);

        // Test for existing neighbors
        let j = g.add_vertex(&1)?;
        g.add_edge(&i, &i)?;
        g.add_edge(&i, &j)?;
        g.add_edge(&j, &j)?;
        assert_eq!(Ne!(g, &j).count(), 2);

        assert_true!(Ne!(g, &j).eq(g.neighbors_iter(&j)));
        assert_true!(Ne!(g, &j).all(|&x| g.has_edge(&x, &j).unwrap() && g.has_edge(&j, &x).unwrap()));
        assert_true!(is_sorted(Ne!(g, &j)));

        Ok(())
    }

    #[test]
    #[should_panic]
    fn neighbors_iter_panics<T>()
    where
        T: UndirectedTrait<Vertex = i32>,
    {
        let g = T::new();
        Ne!(g, &0);
    }

    #[test]
    fn add_undirected_edge<T>() -> Result<(), Error<i32>>
    where
        T: UndirectedTrait<Vertex = i32>,
    {
        let mut g = T::new();

        // Test for undirected edges
        let i = g.add_vertex(&0)?;
        let j = g.add_vertex(&1)?;
        g.add_undirected_edge(&i, &j)?;
        assert_true!(g.has_edge(&i, &j)?);
        assert_true!(g.has_edge(&j, &i)?);

        // Test for repeated undirected edges addition
        assert_true!(g.add_undirected_edge(&i, &j).is_err());

        Ok(())
    }

    #[instantiate_tests(<UndirectedAdjacencyListGraph<i32>>)]
    mod undirected_adjacency_list_graph {}
}
