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
            // ... base case,
            (
                vec![0, 1, 2, 3],
                vec![
                    (0, 1, M::TailHead),
                    (1, 2, M::HeadHead),
                    (1, 3, M::TailHead),
                    (2, 3, M::CircCirc),
                ],
                vec![vec![&0, &1, &2, &3]],
            ),
            // ... extend case,
            (
                vec![0, 1, 2, 3],
                vec![
                    (0, 1, M::TailHead),
                    (1, 2, M::HeadHead),
                    (1, 4, M::TailHead),
                    (2, 3, M::HeadHead),
                    (2, 4, M::TailHead),
                    (3, 4, M::CircCirc),
                ],
                vec![vec![&0, &1, &2, &3, &4]],
            ),
        ];

        // Test for each scenario.
        for (i, j, k) in data {
            let g = PartiallyMixedGraph::new_with_mark(i.clone(), j.clone());

            assert_eq!(AllDiscriminatingPaths::new(&g).collect::<Vec<_>>(), k);
        }
    }
}
