#[cfg(test)]
#[generic_tests::define]
mod tests {
    use crate::directions::DirectedTrait;
    use crate::errors::Error;
    use crate::graphs::DirectedAdjacencyListGraph;
    use crate::{Adj, An, Ch, De, Pa};
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
        T: DirectedTrait<Vertex = i32>,
    {
        let g = T::new();
        assert_true!(g.is_directed());
    }

    #[test]
    fn is_partially_directed<T>()
    where
        T: DirectedTrait<Vertex = i32>,
    {
        let g = T::new();
        assert_false!(g.is_partially_directed());
    }

    #[test]
    fn adjacents_iter<T>() -> Result<(), Error<i32>>
    where
        T: DirectedTrait<Vertex = i32>,
    {
        let mut g = T::new();

        // Test for empty graph
        assert_true!(Adj!(g, &0).is_err());

        // Test for existing vertex
        let i = g.add_vertex(&0)?;
        assert_eq!(Adj!(g, &i)?.count(), 0);

        // Test for existing ancestors
        let j = g.add_vertex(&1)?;
        let k = g.add_vertex(&2)?;
        let l = g.add_vertex(&3)?;
        g.add_edge(&i, &j)?;
        g.add_edge(&k, &j)?;
        g.add_edge(&l, &k)?;
        assert_eq!(Adj!(g, &i)?.count(), 1);
        assert_eq!(Adj!(g, &j)?.count(), 2);
        assert_eq!(Adj!(g, &j)?.collect::<Vec<_>>(), [&i, &k]);

        assert_true!(Adj!(g, &j)?.eq(g.adjacents_iter(&j)?));
        assert_true!(is_sorted(Adj!(g, &j)?));

        Ok(())
    }

    #[test]
    fn ancestors_iter<T>() -> Result<(), Error<i32>>
    where
        T: DirectedTrait<Vertex = i32>,
    {
        let mut g = T::new();

        // Test for empty graph
        assert_true!(An!(g, &0).is_err());

        // Test for existing vertex
        let i = g.add_vertex(&0)?;
        assert_eq!(An!(g, &i)?.count(), 0);

        // Test for existing ancestors
        let j = g.add_vertex(&1)?;
        let k = g.add_vertex(&2)?;
        let l = g.add_vertex(&3)?;
        g.add_edge(&i, &j)?;
        g.add_edge(&k, &j)?;
        g.add_edge(&l, &k)?;
        assert_eq!(An!(g, &i)?.count(), 0);
        assert_eq!(An!(g, &j)?.count(), 3);
        assert_eq!(An!(g, &j)?.collect::<Vec<_>>(), [&i, &k, &l]);

        assert_true!(An!(g, &j)?.eq(g.ancestors_iter(&j)?));
        // assert_true!(An!(g, &j)?.all(|&x| g.has_edge(&x, &j).unwrap() && !g.has_edge(&j, &x).unwrap()));
        assert_true!(is_sorted(An!(g, &j)?));

        Ok(())
    }

    #[test]
    fn parents_iter<T>() -> Result<(), Error<i32>>
    where
        T: DirectedTrait<Vertex = i32>,
    {
        let mut g = T::new();

        // Test for empty graph
        assert_true!(Pa!(g, &0).is_err());

        // Test for existing vertex
        let i = g.add_vertex(&0)?;
        assert_eq!(Pa!(g, &i)?.count(), 0);

        // Test for existing parents
        let j = g.add_vertex(&1)?;
        let k = g.add_vertex(&2)?;
        g.add_edge(&i, &j)?;
        g.add_edge(&k, &j)?;
        assert_eq!(Pa!(g, &i)?.count(), 0);
        assert_eq!(Pa!(g, &j)?.count(), 2);
        assert_eq!(Pa!(g, &j)?.collect::<Vec<_>>(), [&i, &k]);

        assert_true!(Pa!(g, &j)?.eq(g.parents_iter(&j)?));
        assert_true!(Pa!(g, &j)?.all(|&x| g.has_edge(&x, &j).unwrap() && !g.has_edge(&j, &x).unwrap()));
        assert_true!(is_sorted(Pa!(g, &j)?));

        Ok(())
    }

    #[test]
    fn children_iter<T>() -> Result<(), Error<i32>>
    where
        T: DirectedTrait<Vertex = i32>,
    {
        let mut g = T::new();

        // Test for empty graph
        assert_true!(Ch!(g, &0).is_err());

        // Test for existing vertex
        let i = g.add_vertex(&0)?;
        assert_eq!(Ch!(g, &i)?.count(), 0);

        // Test for existing children
        let j = g.add_vertex(&1)?;
        let k = g.add_vertex(&2)?;
        g.add_edge(&j, &i)?;
        g.add_edge(&j, &k)?;
        assert_eq!(Ch!(g, &i)?.count(), 0);
        assert_eq!(Ch!(g, &j)?.count(), 2);
        assert_eq!(Ch!(g, &j)?.collect::<Vec<_>>(), [&i, &k]);

        assert_true!(Ch!(g, &j)?.eq(g.children_iter(&j)?));
        assert_true!(Ch!(g, &j)?.all(|&x| !g.has_edge(&x, &j).unwrap() && g.has_edge(&j, &x).unwrap()));
        assert_true!(is_sorted(Ch!(g, &j)?));

        Ok(())
    }

    #[test]
    fn descendants_iter<T>() -> Result<(), Error<i32>>
    where
        T: DirectedTrait<Vertex = i32>,
    {
        let mut g = T::new();

        // Test for empty graph
        assert_true!(De!(g, &0).is_err());

        // Test for existing vertex
        let i = g.add_vertex(&0)?;
        assert_eq!(De!(g, &i)?.count(), 0);

        // Test for existing descendants
        let j = g.add_vertex(&1)?;
        let k = g.add_vertex(&2)?;
        let l = g.add_vertex(&3)?;
        g.add_edge(&j, &i)?;
        g.add_edge(&j, &k)?;
        g.add_edge(&k, &l)?;
        assert_eq!(De!(g, &i)?.count(), 0);
        assert_eq!(De!(g, &j)?.count(), 3);
        assert_eq!(De!(g, &j)?.collect::<Vec<_>>(), [&i, &k, &l]);

        assert_true!(De!(g, &j)?.eq(g.descendants_iter(&j)?));
        // assert_true!(An!(g, &j)?.all(|&x| g.has_edge(&x, &j).unwrap() && !g.has_edge(&j, &x).unwrap()));
        assert_true!(is_sorted(An!(g, &j)?));

        Ok(())
    }

    #[test]
    fn add_directed_edge<T>() -> Result<(), Error<i32>>
    where
        T: DirectedTrait<Vertex = i32>,
    {
        let mut g = T::new();

        // Test for undirected edges
        let i = g.add_vertex(&0)?;
        let j = g.add_vertex(&1)?;
        g.add_directed_edge(&i, &j)?;
        assert_true!(g.has_edge(&i, &j)?);
        assert_false!(g.has_edge(&j, &i)?);

        // Test for repeated undirected edges addition
        assert_true!(g.add_directed_edge(&i, &j).is_err());

        Ok(())
    }

    #[instantiate_tests(<DirectedAdjacencyListGraph<i32>>)]
    mod directed_adjacency_list_graph {}
}
