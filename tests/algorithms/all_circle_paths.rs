#[cfg(test)]
mod tests {
    use grathe::{
        algorithms::AllCirclePaths, graphs::PartiallyMixedGraph, traits::PartiallyDirected, types::Mark as M,
    };

    #[test]
    fn all_circle_paths() {
        // Test for ...
        let data = [
            // ... zero vertices and zero edges,
            // (vec![], vec![], (0, 0, vec![])),
            // ... one edge,
            (vec![0, 1], vec![(0, 1, M::CircCirc)], (0, 1, vec![vec![&0, &1]])),
            // ... minimal path,
            (
                vec![0, 1, 2],
                vec![(0, 1, M::CircCirc), (1, 2, M::CircCirc)],
                (0, 2, vec![vec![&0, &1, &2]]),
            ),
            // ... long path,
            (
                vec![0, 1, 2],
                vec![
                    (0, 1, M::CircCirc),
                    (1, 2, M::CircCirc),
                    (2, 3, M::CircCirc),
                    (3, 4, M::CircCirc),
                    (4, 5, M::CircCirc),
                    (5, 6, M::CircCirc),
                    (6, 7, M::CircCirc),
                    (7, 8, M::CircCirc),
                    (8, 9, M::CircCirc),
                ],
                (0, 9, vec![vec![&0, &1, &2, &3, &4, &5, &6, &7, &8, &9]]),
            ),
            // ... multiple paths,
            (
                vec![0, 1, 2],
                vec![(0, 1, M::CircCirc), (1, 2, M::CircCirc), (2, 0, M::CircCirc)],
                (0, 2, vec![vec![&0, &1, &2], vec![&0, &2]]),
            ),
        ];

        // Test for each scenario.
        for (i, j, k) in data {
            let g = PartiallyMixedGraph::new_with_mark(i.clone(), j.clone());

            let (x, y, p) = k;
            let mut search: AllCirclePaths<_, _> = From::from((&g, &x, &y));
            search.call_mut();

            assert_eq!(search.circle_paths, p);
        }
    }

    #[test]
    #[should_panic]
    fn all_circle_paths_should_panic() {
        let g = PartiallyMixedGraph::new_with_mark([], []);
        let mut search = AllCirclePaths::new(&g, &0, &1);
        search.call_mut();
    }
}
