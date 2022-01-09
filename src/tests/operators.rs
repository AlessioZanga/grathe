#[cfg(test)]
#[generic_tests::define]
mod tests {
    use crate::graphs::{DirectedAdjacencyListGraph, UndirectedAdjacencyListGraph};
    use crate::traits::Operators;
    use crate::traits::Storage;
    use crate::{E, V};
    use all_asserts::*;
    use std::ops::{BitAnd, BitOr, BitXor, Not, Sub};

    #[test]
    fn complement<T>()
    where
        T: Storage<Vertex = i32> + Operators,
        for<'a> &'a T: Not<Output = T>,
    {
        let g = T::from_edges(&[(0, 1), (2, 3)]);
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
    fn union<T>()
    where
        T: Storage<Vertex = i32> + Operators,
        for<'a> &'a T: BitOr<&'a T, Output = T>,
    {
        let g = T::from_edges(&[(0, 1), (2, 3)]);
        let h = T::from_edges(&[(0, 2), (3, 4)]);
        let u = g.union(&h);

        assert_eq!(u, &g | &h);
        assert_true!(V!(u).eq(&[0, 1, 2, 3, 4]));
        assert_true!(E!(u).eq([(&0, &1), (&0, &2), (&2, &3), (&3, &4)]));
    }

    #[test]
    fn intersection<T>()
    where
        T: Storage<Vertex = i32> + Operators,
        for<'a> &'a T: BitAnd<&'a T, Output = T>,
    {
        let g = T::from_edges(&[(0, 1), (2, 3), (3, 3)]);
        let h = T::from_edges(&[(0, 2), (3, 4), (3, 3)]);
        let u = g.intersection(&h);

        assert_eq!(u, &g & &h);
        assert_true!(V!(u).eq(&[0, 2, 3]));
        assert_true!(E!(u).eq([(&3, &3)]));
    }

    #[test]
    fn symmetric_difference<T>()
    where
        T: Storage<Vertex = i32> + Operators,
        for<'a> &'a T: BitOr<&'a T, Output = T> + BitXor<&'a T, Output = T> + Sub<&'a T, Output = T>,
    {
        let g = T::from_edges(&[(0, 1), (2, 3), (3, 3)]);
        let h = T::from_edges(&[(0, 2), (3, 4), (3, 3)]);
        let u = g.symmetric_difference(&h);

        assert_eq!(u, &g ^ &h);
        assert_eq!(&g ^ &h, &(&g - &h) | &(&h - &g));
        assert_true!(V!(u).eq(&[0, 1, 2, 3, 4]));
        assert_true!(E!(u).eq([(&0, &1), (&0, &2), (&2, &3), (&3, &4)]));
    }

    #[test]
    fn difference<T>()
    where
        T: Storage<Vertex = i32> + Operators,
        for<'a> &'a T: Sub<&'a T, Output = T>,
    {
        let g = T::from_edges(&[(0, 1), (2, 3), (3, 3)]);
        let h = T::from_edges(&[(0, 2), (3, 4), (3, 3)]);
        let u = g.difference(&h);

        assert_eq!(u, &g - &h);
        assert_true!(V!(u).eq(&[0, 1, 2, 3]));
        assert_true!(E!(u).eq([(&0, &1), (&2, &3)]));
    }

    #[instantiate_tests(<DirectedAdjacencyListGraph<i32>>)]
    mod directed_adjacency_list_graph {}

    #[instantiate_tests(<UndirectedAdjacencyListGraph<i32>>)]
    mod undirected_adjacency_list_graph {}
}
