#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use grathe::{algorithms::PossibleDSep, graphs::PartiallyMixedGraph, traits::PartiallyDirected, types::Mark as M};

    #[test]
    fn possible_d_sep() {
        // Test for ...
        let data = [
            // ... zero vertices and zero edges,
            // (vec![], vec![], (0, 0, vec![])),
            // ... one edge,
            (vec![0, 1], vec![(0, 1, M::CircCirc)], (0, vec![])),
            // ... minimal path (triangle),
            (
                vec![0, 1, 2],
                vec![(0, 1, M::CircCirc), (1, 2, M::CircCirc), (0, 2, M::CircCirc)],
                (0, vec![1, 2]),
            ),
            // ... minimal path (collider),
            (
                vec![0, 1, 2],
                vec![(0, 1, M::CircHead), (1, 2, M::HeadHead)],
                (0, vec![2]),
            ),
            // ... long path,
            (
                vec![0, 1, 2, 3, 4, 5],
                vec![
                    (0, 1, M::CircCirc),
                    (1, 2, M::CircCirc),
                    (0, 2, M::CircCirc),
                    (2, 3, M::CircCirc),
                    (3, 1, M::CircCirc),
                    (0, 4, M::CircCirc),
                    (1, 4, M::CircCirc),
                    (4, 1, M::CircCirc),
                ],
                (0, vec![1, 2, 3, 4]),
            ),
        ];

        // Test for each scenario.
        for (i, j, k) in data {
            let g = PartiallyMixedGraph::new_with_mark(i.clone(), j.clone());

            let (x, p) = k;
            let p: BTreeSet<_> = p.iter().collect();
            let mut search: PossibleDSep<_, _> = From::from((&g, &x));
            search.call_mut();

            assert_eq!(search.possible_d_sep, p);
        }
    }
}
