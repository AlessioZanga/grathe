#[cfg(test)]
mod tests {
    use grathe::{
        algorithms::AllDiscriminatingPaths, graphs::PartiallyMixedGraph, traits::PartiallyDirected, types::Mark as M,
    };

    #[test]
    fn all_discriminating_paths() {
        // Test for ...
        let data = [
            // ... zero vertices and empty sequence,
            (vec![], vec![], vec![]),
            // ... one valid triplet, but no path:
            //
            //      > 2
            //     /  o
            //    /   |
            //   /    o
            //  0 <-o 1
            //
            (
                vec![0, 1, 2],
                vec![(1, 0, M::CircHead), (0, 2, M::TailHead), (1, 2, M::CircCirc)],
                vec![],
            ),
            // ... shortest path:
            //
            //            > 3
            //           /  o
            //          /   |
            //         /    o
            //  0 --> 1 <-o 2
            //
            (
                vec![0, 1, 2, 3],
                vec![
                    (0, 1, M::TailHead),
                    (2, 1, M::CircHead),
                    (1, 3, M::TailHead),
                    (2, 3, M::CircCirc),
                ],
                vec![vec![&0, &1, &2, &3]],
            ),
            // ... minimal shortest path:
            //
            //            > 3
            //    5      /  o
            //     \    /   |
            //      >  /    o
            //  0 --> 1 <-o 2
            //
            (
                vec![0, 1, 2, 3],
                vec![
                    (0, 1, M::TailHead),
                    (2, 1, M::CircHead),
                    (1, 3, M::TailHead),
                    (2, 3, M::CircCirc),
                    (5, 1, M::TailHead),
                ],
                vec![vec![&0, &1, &2, &3]],
            ),
            // ... extended path:
            //
            //              --->> 4
            //             /   /  o
            //          ---   /   |
            //         /     /    o
            //  0 --> 1 <-> 2 <-o 3
            //
            (
                vec![0, 1, 2, 3],
                vec![
                    (0, 1, M::TailHead),
                    (1, 2, M::HeadHead),
                    (1, 4, M::TailHead),
                    (3, 2, M::CircHead),
                    (2, 4, M::TailHead),
                    (3, 4, M::CircCirc),
                ],
                vec![vec![&0, &1, &2, &4], vec![&0, &1, &2, &3, &4]],
            ),
            // ... extended path, different marks:
            //
            //              --->> 4
            //             /   /  |
            //          ---   /   |
            //         /     /    o
            //  0 --> 1 <-> 2 <-- 3
            //
            (
                vec![0, 1, 2, 3],
                vec![
                    (0, 1, M::TailHead),
                    (1, 2, M::HeadHead),
                    (1, 4, M::TailHead),
                    (3, 2, M::TailHead),
                    (2, 4, M::TailHead),
                    (3, 4, M::CircTail),
                ],
                vec![vec![&0, &1, &2, &4], vec![&0, &1, &2, &3, &4]],
            ),
            // ... multiple extended path:
            //
            //              --->> 4
            //             /   /  o
            //          ---   /   |
            //         /     /    o
            //  0 --> 1 <-> 2 <-o 3
            //         \     \    o
            //          ---   \   |
            //             \   \  o
            //              --->> 5
            //
            (
                vec![0, 1, 2, 3],
                vec![
                    (0, 1, M::TailHead),
                    (1, 2, M::HeadHead),
                    (1, 4, M::TailHead),
                    (1, 5, M::TailHead),
                    (3, 2, M::CircHead),
                    (2, 4, M::TailHead),
                    (2, 5, M::TailHead),
                    (3, 4, M::CircCirc),
                    (3, 5, M::CircCirc),
                ],
                vec![
                    vec![&0, &1, &2, &4],
                    vec![&0, &1, &2, &5],
                    vec![&0, &1, &2, &3, &4],
                    vec![&0, &1, &2, &3, &5],
                ],
            ),
        ];

        // Test for each scenario.
        for (i, j, k) in data {
            let g = PartiallyMixedGraph::new_with_mark(i.clone(), j.clone());

            let search = AllDiscriminatingPaths::new(&g);
            let discriminating_paths = search.collect::<Vec<_>>();

            assert_eq!(discriminating_paths, k);
        }
    }
}
