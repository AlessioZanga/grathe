#[cfg(test)]
#[generic_tests::define]
mod tests {
    use crate::errors::Error;
    use crate::graphs::DirectedAdjacencyListGraph;
    use crate::traits::Directed;
    use crate::{An, Ch, De, Pa};
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
    fn ancestors_iter<T>() -> Result<(), Error<i32>>
    where
        T: Directed<Vertex = i32>,
    {
        let mut g = T::new();

        // Test for existing vertex
        let i = g.add_vertex(&0)?;
        assert_eq!(An!(g, &i).count(), 0);

        // Test for existing ancestors
        let j = g.add_vertex(&1)?;
        let k = g.add_vertex(&2)?;
        let l = g.add_vertex(&3)?;
        g.add_edge(&i, &j)?;
        g.add_edge(&k, &j)?;
        g.add_edge(&l, &k)?;
        assert_eq!(An!(g, &i).count(), 0);
        assert_eq!(An!(g, &j).count(), 3);
        assert_true!(An!(g, &j).eq(&[i, k, l]));

        assert_true!(An!(g, &j).eq(g.ancestors_iter(&j)));
        // assert_true!(An!(g, &j)?.all(|&x| g.has_edge(&x, &j).unwrap() && !g.has_edge(&j, &x).unwrap()));
        assert_true!(is_sorted(An!(g, &j)));

        Ok(())
    }

    #[test]
    #[should_panic]
    fn ancestors_iter_should_panic<T>()
    where
        T: Directed<Vertex = i32>,
    {
        let g = T::new();
        An!(g, &0);
    }

    #[test]
    fn parents_iter<T>() -> Result<(), Error<i32>>
    where
        T: Directed<Vertex = i32>,
    {
        let mut g = T::new();

        // Test for existing vertex
        let i = g.add_vertex(&0)?;
        assert_eq!(Pa!(g, &i).count(), 0);

        // Test for existing parents
        let j = g.add_vertex(&1)?;
        let k = g.add_vertex(&2)?;
        g.add_edge(&i, &j)?;
        g.add_edge(&k, &j)?;
        assert_eq!(Pa!(g, &i).count(), 0);
        assert_eq!(Pa!(g, &j).count(), 2);
        assert_true!(Pa!(g, &j).eq(&[i, k]));

        assert_true!(Pa!(g, &j).eq(g.parents_iter(&j)));
        assert_true!(Pa!(g, &j).all(|&x| g.has_edge(&x, &j).unwrap() && !g.has_edge(&j, &x).unwrap()));
        assert_true!(is_sorted(Pa!(g, &j)));

        Ok(())
    }

    #[test]
    #[should_panic]
    fn parents_iter_should_panic<T>()
    where
        T: Directed<Vertex = i32>,
    {
        let g = T::new();
        Pa!(g, &0);
    }

    #[test]
    fn children_iter<T>() -> Result<(), Error<i32>>
    where
        T: Directed<Vertex = i32>,
    {
        let mut g = T::new();

        // Test for existing vertex
        let i = g.add_vertex(&0)?;
        assert_eq!(Ch!(g, &i).count(), 0);

        // Test for existing children
        let j = g.add_vertex(&1)?;
        let k = g.add_vertex(&2)?;
        g.add_edge(&j, &i)?;
        g.add_edge(&j, &k)?;
        assert_eq!(Ch!(g, &i).count(), 0);
        assert_eq!(Ch!(g, &j).count(), 2);
        assert_true!(Ch!(g, &j).eq(&[i, k]));

        assert_true!(Ch!(g, &j).eq(g.children_iter(&j)));
        assert_true!(Ch!(g, &j).all(|&x| !g.has_edge(&x, &j).unwrap() && g.has_edge(&j, &x).unwrap()));
        assert_true!(is_sorted(Ch!(g, &j)));

        Ok(())
    }

    #[test]
    #[should_panic]
    fn children_iter_should_panic<T>()
    where
        T: Directed<Vertex = i32>,
    {
        let g = T::new();
        Ch!(g, &0);
    }

    #[test]
    fn descendants_iter<T>() -> Result<(), Error<i32>>
    where
        T: Directed<Vertex = i32>,
    {
        let mut g = T::new();

        // Test for existing vertex
        let i = g.add_vertex(&0)?;
        assert_eq!(De!(g, &i).count(), 0);

        // Test for existing descendants
        let j = g.add_vertex(&1)?;
        let k = g.add_vertex(&2)?;
        let l = g.add_vertex(&3)?;
        g.add_edge(&j, &i)?;
        g.add_edge(&j, &k)?;
        g.add_edge(&k, &l)?;
        assert_eq!(De!(g, &i).count(), 0);
        assert_eq!(De!(g, &j).count(), 3);
        assert_true!(De!(g, &j).eq(&[i, k, l]));

        assert_true!(De!(g, &j).eq(g.descendants_iter(&j)));
        // assert_true!(An!(g, &j)?.all(|&x| g.has_edge(&x, &j).unwrap() && !g.has_edge(&j, &x).unwrap()));
        assert_true!(is_sorted(An!(g, &j)));

        Ok(())
    }

    #[test]
    #[should_panic]
    fn descendants_iter_should_panic<T>()
    where
        T: Directed<Vertex = i32>,
    {
        let g = T::new();
        De!(g, &0);
    }

    #[test]
    fn add_directed_edge<T>() -> Result<(), Error<i32>>
    where
        T: Directed<Vertex = i32>,
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

    #[test]
    fn in_degree_of<T>() -> Result<(), Error<i32>>
    where
        T: Directed<Vertex = i32>,
    {
        let mut g = T::new();
        let i = g.add_vertex(&0)?;

        assert_eq!(g.in_degree_of(&i), 0);

        let j = g.add_vertex(&1)?;
        g.add_edge(&i, &j)?;

        assert_eq!(g.in_degree_of(&i), 0);
        assert_eq!(g.in_degree_of(&j), 1);

        let k = g.add_vertex(&2)?;
        g.add_edge(&i, &k)?;
        g.add_edge(&j, &k)?;

        assert_eq!(g.in_degree_of(&i), 0);
        assert_eq!(g.in_degree_of(&j), 1);
        assert_eq!(g.in_degree_of(&k), 2);

        Ok(())
    }

    #[test]
    #[should_panic]
    fn in_degree_of_should_panic<T>()
    where
        T: Directed<Vertex = i32>,
    {
        let g = T::new();
        g.in_degree_of(&0);
    }

    #[test]
    fn out_degree_of<T>() -> Result<(), Error<i32>>
    where
        T: Directed<Vertex = i32>,
    {
        let mut g = T::new();
        let i = g.add_vertex(&0)?;

        assert_eq!(g.out_degree_of(&i), 0);

        let j = g.add_vertex(&1)?;
        g.add_edge(&i, &j)?;

        assert_eq!(g.out_degree_of(&i), 1);
        assert_eq!(g.out_degree_of(&j), 0);

        let k = g.add_vertex(&2)?;
        g.add_edge(&i, &k)?;
        g.add_edge(&j, &k)?;

        assert_eq!(g.out_degree_of(&i), 2);
        assert_eq!(g.out_degree_of(&j), 1);
        assert_eq!(g.out_degree_of(&k), 0);

        Ok(())
    }

    #[test]
    #[should_panic]
    fn out_degree_of_should_panic<T>()
    where
        T: Directed<Vertex = i32>,
    {
        let g = T::new();
        g.out_degree_of(&0);
    }

    #[test]
    fn topological_sort<T>() -> Result<(), Error<i32>>
    where
        T: Directed<Vertex = i32>,
    {
        let mut g = T::new();
        assert_eq!(g.topological_sort(), Some(vec![]));

        let i = g.add_vertex(&0)?;
        assert_eq!(g.topological_sort(), Some(vec![&i]));

        let j = g.add_vertex(&1)?;
        g.add_edge(&i, &j)?;
        assert_eq!(g.topological_sort(), Some(vec![&i, &j]));

        g.add_edge(&j, &i)?;
        assert_eq!(g.topological_sort(), None);

        Ok(())
    }

    #[instantiate_tests(<DirectedAdjacencyListGraph<i32>>)]
    mod adjacency_list_graph {}
}
