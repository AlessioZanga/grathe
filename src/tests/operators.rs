#[cfg(test)]
#[generic_tests::define]
mod tests {
    use crate::graphs::storages::DirectedAdjacencyList;
    use crate::traits::{From, Operators, Storage};
    use crate::{E, V};
    use all_asserts::*;
    use std::ops::{BitAnd, BitOr, BitXor, Not, Sub};

    #[test]
    fn complement<G>()
    where
        G: Storage<Vertex = i32> + From + Operators,
        for<'a> &'a G: Not<Output = G>,
    {
        let g = G::from_edges([(0, 1), (2, 3)]);
        let u = g.complement();

        assert_eq!(u, !&g);
        assert_true!(V!(u).eq(&[0, 1, 2, 3]));
        assert_true!(E!(u).eq([
            (&0, &0),
            (&0, &2),
            (&0, &3),
            (&1, &0),
            (&1, &1),
            (&1, &2),
            (&1, &3),
            (&2, &0),
            (&2, &1),
            (&2, &2),
            (&3, &0),
            (&3, &1),
            (&3, &2),
            (&3, &3)
        ]));
    }

    #[test]
    fn union<G>()
    where
        G: Storage<Vertex = i32> + From + Operators,
        for<'a> &'a G: BitOr<&'a G, Output = G>,
    {
        let g = G::from_edges([(0, 1), (2, 3)]);
        let h = G::from_edges([(0, 2), (3, 4)]);
        let u = g.union(&h);

        assert_eq!(u, &g | &h);
        assert_true!(V!(u).eq(&[0, 1, 2, 3, 4]));
        assert_true!(E!(u).eq([(&0, &1), (&0, &2), (&2, &3), (&3, &4)]));
    }

    #[test]
    fn intersection<G>()
    where
        G: Storage<Vertex = i32> + From + Operators,
        for<'a> &'a G: BitAnd<&'a G, Output = G>,
    {
        let g = G::from_edges([(0, 1), (2, 3), (3, 3)]);
        let h = G::from_edges([(0, 2), (3, 4), (3, 3)]);
        let u = g.intersection(&h);

        assert_eq!(u, &g & &h);
        assert_true!(V!(u).eq(&[0, 2, 3]));
        assert_true!(E!(u).eq([(&3, &3)]));
    }

    #[test]
    fn symmetric_difference<G>()
    where
        G: Storage<Vertex = i32> + From + Operators,
        for<'a> &'a G: BitOr<&'a G, Output = G> + BitXor<&'a G, Output = G> + Sub<&'a G, Output = G>,
    {
        let g = G::from_edges([(0, 1), (2, 3), (3, 3)]);
        let h = G::from_edges([(0, 2), (3, 4), (3, 3)]);
        let u = g.symmetric_difference(&h);

        assert_eq!(u, &g ^ &h);
        assert_eq!(&g ^ &h, &(&g - &h) | &(&h - &g));
        assert_true!(V!(u).eq(&[0, 1, 2, 3, 4]));
        assert_true!(E!(u).eq([(&0, &1), (&0, &2), (&2, &3), (&3, &4)]));
    }

    #[test]
    fn difference<G>()
    where
        G: Storage<Vertex = i32> + From + Operators,
        for<'a> &'a G: Sub<&'a G, Output = G>,
    {
        let g = G::from_edges([(0, 1), (2, 3), (3, 3)]);
        let h = G::from_edges([(0, 2), (3, 4), (3, 3)]);
        let u = g.difference(&h);

        assert_eq!(u, &g - &h);
        assert_true!(V!(u).eq(&[0, 1, 2, 3]));
        assert_true!(E!(u).eq([(&0, &1), (&2, &3)]));
    }

    #[instantiate_tests(<DirectedAdjacencyList<i32>>)]
    mod directed_adjacency_list {}
}
