#[cfg(test)]
#[generic_tests::define]
mod tests_undirected {
    use crate::directions::{DirectionalTrait, UndirectedTrait};
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
        T: UndirectedTrait<Vertex = u32>,
    {
        let g = T::default();
        assert_false!(g.is_directed());
    }

    #[test]
    fn is_partially_directed<T>()
    where
        T: UndirectedTrait<Vertex = u32>,
    {
        let g = T::default();
        assert_false!(g.is_partially_directed());
    }

    #[test]
    fn add_edge<T>() -> Result<(), Error<u32>>
    where
        T: UndirectedTrait<Vertex = u32>,
    {
        let mut g = T::default();

        // Test for undirected edges
        let i = g.add_vertex()?;
        let j = g.add_vertex()?;
        let e = g.add_edge(&(i, j))?;
        assert_true!(g.has_edge(&(i, j))?);
        assert_true!(g.has_edge(&(j, i))?);

        // Test for repeated undirected edges addition
        assert_true!(g.add_edge(&e).is_err());

        // Test for loops
        let e = g.add_edge(&(i, i))?;
        assert_true!(g.has_edge(&(i, i))?);
        g.del_edge(&e)?;

        // Del vertex and associated edges.
        g.del_vertex(&i)?;
        assert_true!(g.has_edge(&(i, j)).is_err());
        assert_true!(g.has_edge(&(j, i)).is_err());
        assert_true!(Ne!(g, &i).is_err());
        assert_true!(!Ne!(g, &j)?.any(|x| x == i));

        Ok(())
    }

    #[test]
    fn del_edge<T>() -> Result<(), Error<u32>>
    where
        T: UndirectedTrait<Vertex = u32>,
    {
        let mut g = T::default();

        // Test for undirected edges
        let i = g.add_vertex()?;
        let j = g.add_vertex()?;
        let e = g.add_edge(&(i, j))?;
        assert_true!(g.has_edge(&(i, j))?);
        assert_true!(g.has_edge(&(j, i))?);
        assert_false!(g.del_edge(&e).is_err());

        // Test for repeated undirected edges deletion
        assert_true!(g.del_edge(&e).is_err());
        assert_false!(g.has_edge(&(i, j))?);
        assert_false!(g.has_edge(&(j, i))?);

        Ok(())
    }

    #[test]
    fn neighbors_iter<T>() -> Result<(), Error<u32>>
    where
        T: UndirectedTrait<Vertex = u32>,
    {
        let mut g = T::default();

        // Test for empty graph
        assert_true!(Ne!(g, &0).is_err());

        // Test for existing vertex
        let i = g.add_vertex()?;
        assert_eq!(Ne!(g, &i)?.count(), 0);

        // Test for existing neighbors
        let j = g.add_vertex()?;
        g.add_edge(&(i, i))?;
        g.add_edge(&(i, j))?;
        g.add_edge(&(j, j))?;
        assert_eq!(Ne!(g, &i)?.count(), 2);

        assert_true!(Ne!(g, &i)?.eq(g.neighbors_iter(&i)?));
        assert_true!(
            Ne!(g, &i)?.all(|x| g.has_edge(&(i, x)).unwrap() && g.has_edge(&(x, i)).unwrap())
        );
        assert_true!(is_sorted(Ne!(g, &i)?));

        Ok(())
    }

    #[test]
    fn add_undirected_edge<T>() -> Result<(), Error<u32>>
    where
        T: UndirectedTrait<Vertex = u32>,
    {
        let mut g = T::default();

        // Test for undirected edges
        let i = g.add_vertex()?;
        let j = g.add_vertex()?;
        let e = g.add_undirected_edge(&(i, j))?;
        assert_true!(g.has_edge(&(i, j))?);
        assert_true!(g.has_edge(&(j, i))?);

        // Test for repeated undirected edges addition
        assert_true!(g.add_undirected_edge(&e).is_err());

        Ok(())
    }

    #[test]
    fn reserve_undirected_edge<T>() -> Result<(), Error<u32>>
    where
        T: UndirectedTrait<Vertex = u32>,
    {
        let mut g = T::default();

        // Test for undirected edges
        let i = 0;
        let j = 1;
        let e = g.reserve_undirected_edge(&(i, j))?;
        assert_true!(g.has_edge(&(i, j))?);
        assert_true!(g.has_edge(&(j, i))?);

        // Test for repeated undirected edges addition
        assert_true!(g.reserve_undirected_edge(&e).is_err());

        Ok(())
    }

    #[test]
    fn add_undirected_edge_label<T>() -> Result<(), Error<u32>>
    where
        T: UndirectedTrait<Vertex = u32>,
    {
        let mut g = T::default();

        // Test for undirected edges
        let i = g.add_vertex()?;
        let j = g.add_vertex()?;
        let e = g.add_undirected_edge_label(&(i, j), "(0, 1)")?;
        assert_true!(g.has_edge(&(i, j))?);
        assert_true!(g.has_edge(&(j, i))?);
        assert_true!(g.has_edge_label("(0, 1)"));
        assert_eq!(g.get_edge_id("(0, 1)")?, e);
        assert_eq!(g.get_edge_label(&e)?, "(0, 1)");

        // Test for repeated undirected edges addition
        assert_true!(g.add_undirected_edge(&e).is_err());

        Ok(())
    }

    #[test]
    fn reserve_undirected_edge_label<T>() -> Result<(), Error<u32>>
    where
        T: UndirectedTrait<Vertex = u32>,
    {
        let mut g = T::default();

        // Test for undirected edges
        let i = 0;
        let j = 1;
        let e = g.reserve_undirected_edge_label(&(i, j), "(0, 1)")?;
        assert_true!(g.has_edge(&(i, j))?);
        assert_true!(g.has_edge(&(j, i))?);
        assert_true!(g.has_edge_label("(0, 1)"));
        assert_eq!(g.get_edge_id("(0, 1)")?, e);
        assert_eq!(g.get_edge_label(&e)?, "(0, 1)");

        // Test for repeated undirected edges addition
        assert_true!(g.reserve_undirected_edge(&e).is_err());

        Ok(())
    }

    #[instantiate_tests(<UndirectedAdjacencyListGraph<u32>>)]
    mod undirected_adjacency_list_graph {}
}
