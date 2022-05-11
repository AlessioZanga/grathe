#[cfg(test)]
mod io {
    mod dot {
        use grathe::{
            graphs::storages::UndirectedAdjacencyList,
            io::{DOT, IO},
            traits::Storage,
            E, V,
        };
        use tempfile::NamedTempFile;

        #[test]
        fn map() {
            // Test `DOT::map<G>() -> G`.

            // Test for ...
            let data = [
                // ... zero vertices and zero edges,
                ("graph {}", (0, 0, vec![], vec![])),
                ("graph { }", (0, 0, vec![], vec![])),
                ("graph {  }", (0, 0, vec![], vec![])),
                ("graph {\t}", (0, 0, vec![], vec![])),
                ("graph {\n}", (0, 0, vec![], vec![])),
                ("graph \"G\" {}", (0, 0, vec![], vec![])),
                ("digraph {}", (0, 0, vec![], vec![])),
                ("strict digraph {}", (0, 0, vec![], vec![])),
                // ... one vertex and zero edges,
                ("graph {1}", (1, 0, vec![1], vec![])),
                ("graph { 1}", (1, 0, vec![1], vec![])),
                ("graph {1 }", (1, 0, vec![1], vec![])),
                ("graph { 1 }", (1, 0, vec![1], vec![])),
                ("graph {\t1}", (1, 0, vec![1], vec![])),
                ("graph {\n1}", (1, 0, vec![1], vec![])),
                ("graph {1;}", (1, 0, vec![1], vec![])),
                ("graph { 1;}", (1, 0, vec![1], vec![])),
                ("graph {1; }", (1, 0, vec![1], vec![])),
                ("graph { 1; }", (1, 0, vec![1], vec![])),
                ("graph {\t1;}", (1, 0, vec![1], vec![])),
                ("graph {\n1;}", (1, 0, vec![1], vec![])),
                // ... zero vertices and one edge,
                ("graph {1 -- 2}", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("graph { 1 -- 2}", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("graph {1 -- 2 }", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("graph { 1 -- 2 }", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("graph {\t1 -- 2}", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("graph {\n1 -- 2}", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("graph {1 -- 2;}", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("graph { 1 -- 2;}", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("graph {1 -- 2; }", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("graph { 1 -- 2; }", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("graph {\t1 -- 2;}", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("graph {\n1 -- 2;}", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("digraph {1 -> 2}", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("digraph { 1 -> 2}", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("digraph {1 -> 2 }", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("digraph { 1 -> 2 }", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("digraph {\t1 -> 2}", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("digraph {\n1 -> 2}", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("digraph {1 -> 2;}", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("digraph { 1 -> 2;}", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("digraph {1 -> 2; }", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("digraph { 1 -> 2; }", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("digraph {\t1 -> 2;}", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("digraph {\n1 -> 2;}", (2, 1, vec![1, 2], vec![(1, 2)])),
                // ... one vertex and one edge,
                ("graph {1 -- 2; 3}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("graph { 1 -- 2; 3}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("graph {1 -- 2 ; 3}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("graph { 1 -- 2 ; 3}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("graph {\t1 -- 2; 3}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("graph {\n1 -- 2; 3}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("graph {1 -- 2; 3;}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("graph { 1 -- 2; 3;}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("graph {1 -- 2; 3; }", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("graph { 1 -- 2; 3; }", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("graph {\t1 -- 2; 3;}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("graph {\n1 -- 2; 3;}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("digraph {1 -> 2; 3}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("digraph { 1 -> 2; 3}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("digraph {1 -> 2; 3 }", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("digraph { 1 -> 2; 3 }", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("digraph {\t1 -> 2; 3}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("digraph {\n1 -> 2; 3}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("digraph {1 -> 2; 3;}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("digraph { 1 -> 2; 3;}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("digraph {1 -> 2; 3; }", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("digraph { 1 -> 2; 3; }", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("digraph {\t1 -> 2; 3;}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("digraph {\n1 -> 2; 3;}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                // ... multiple vertices and zero edges,
                ("graph {1\n2\n3}", (3, 0, vec![1, 2, 3], vec![])),
                ("graph { 1\n 2\n 3}", (3, 0, vec![1, 2, 3], vec![])),
                ("graph {1\n 2\n 3 }", (3, 0, vec![1, 2, 3], vec![])),
                ("graph { 1\n 2\n 3 }", (3, 0, vec![1, 2, 3], vec![])),
                ("graph {\t1\n\t2\n\t3}", (3, 0, vec![1, 2, 3], vec![])),
                ("graph {\n1\n2\n3}", (3, 0, vec![1, 2, 3], vec![])),
                ("graph {1;2;3;}", (3, 0, vec![1, 2, 3], vec![])),
                ("graph { 1; 2; 3;}", (3, 0, vec![1, 2, 3], vec![])),
                ("graph {1; 2; 3; }", (3, 0, vec![1, 2, 3], vec![])),
                ("graph { 1; 2; 3; }", (3, 0, vec![1, 2, 3], vec![])),
                ("graph {\t1;\t2;\t3;}", (3, 0, vec![1, 2, 3], vec![])),
                ("graph {\n1;\n2;\n3;}", (3, 0, vec![1, 2, 3], vec![])),
                // ... zero vertices and multiple edges,
                ("graph {1 -- 2\n 2 -- 3}", (3, 2, vec![1, 2, 3], vec![(1, 2), (2, 3)])),
                (
                    "graph { 1 -- 2\n 2 -- 3\n}",
                    (3, 2, vec![1, 2, 3], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph {1 -- 2\n 2 -- 3 \n}",
                    (3, 2, vec![1, 2, 3], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph { 1 -- 2\n 2 -- 3 \n}",
                    (3, 2, vec![1, 2, 3], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph {\t1 -- 2\n \t2 -- 3\n}",
                    (3, 2, vec![1, 2, 3], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph {\n1 -- 2\n2 -- 3\n}",
                    (3, 2, vec![1, 2, 3], vec![(1, 2), (2, 3)]),
                ),
                ("graph {1 -- 2; 2 -- 3;}", (3, 2, vec![1, 2, 3], vec![(1, 2), (2, 3)])),
                ("graph { 1 -- 2; 2 -- 3;}", (3, 2, vec![1, 2, 3], vec![(1, 2), (2, 3)])),
                ("graph {1 -- 2; 2 -- 3; }", (3, 2, vec![1, 2, 3], vec![(1, 2), (2, 3)])),
                ("graph { 1 -- 2; 2 -- 3; }", (3, 2, vec![1, 2, 3], vec![(1, 2), (2, 3)])),
                (
                    "graph {\t1 -- 2; \t2 -- 3;}",
                    (3, 2, vec![1, 2, 3], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph {\n1 -- 2; \n2 -- 3;}",
                    (3, 2, vec![1, 2, 3], vec![(1, 2), (2, 3)]),
                ),
                // ... multiple vertices and multiple edges,
                (
                    "graph {1 -- 2\n 2 -- 3 \n4 \n5 \n6}",
                    (6, 2, vec![1, 2, 3, 4, 5, 6], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph { 1 -- 2\n 2 -- 3\n \n4 \n5 \n6}",
                    (6, 2, vec![1, 2, 3, 4, 5, 6], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph {1 -- 2\n 2 -- 3 \n \n4 \n5 \n6 }",
                    (6, 2, vec![1, 2, 3, 4, 5, 6], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph { 1 -- 2\n 2 -- 3 \n \n4 \n5 \n6}",
                    (6, 2, vec![1, 2, 3, 4, 5, 6], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph {\t1 -- 2\n\t2 -- 3\n\n\t4\n\t5\n\t6}",
                    (6, 2, vec![1, 2, 3, 4, 5, 6], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph {\n1 -- 2\n2 -- 3\n \n4 \n5 \n6}",
                    (6, 2, vec![1, 2, 3, 4, 5, 6], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph {1 -- 2; 2 -- 3; 4; 5; 6;}",
                    (6, 2, vec![1, 2, 3, 4, 5, 6], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph { 1 -- 2; 2 -- 3; 4; 5; 6;}",
                    (6, 2, vec![1, 2, 3, 4, 5, 6], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph {1 -- 2; 2 -- 3;  4; 5; 6; }",
                    (6, 2, vec![1, 2, 3, 4, 5, 6], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph { 1 -- 2; 2 -- 3; 4; 5; 6; }",
                    (6, 2, vec![1, 2, 3, 4, 5, 6], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph {\t1 -- 2;\t2 -- 3;\t4;\t5;\t6;}",
                    (6, 2, vec![1, 2, 3, 4, 5, 6], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph {\n1 -- 2;\n2 -- 3;\n4;\n5;\n6;}",
                    (6, 2, vec![1, 2, 3, 4, 5, 6], vec![(1, 2), (2, 3)]),
                ),
            ];

            // Test for each scenario.
            for (i, j) in data {
                let g = DOT::try_from(i.to_string());
                let g: UndirectedAdjacencyList<_> = g
                    .expect("DOT parser failed")
                    .map(|x| x.parse::<i32>().unwrap())
                    .pop()
                    .expect("Result vec is empty");
                assert_eq!(
                    (
                        g.order(),
                        g.size(),
                        V!(g).cloned().collect::<Vec<i32>>(),
                        E!(g).map(|(&x, &y)| (x, y)).collect::<Vec<(i32, i32)>>()
                    ),
                    j.clone(),
                    "with test data: '{:?}'",
                    (i, j)
                );
            }
        }

        #[test]
        fn map_with_attributes() {
            // Test `DOT::map_with_attributes<G>() -> G`.

            // Test for ...
            let data = [
                // ... zero vertices and zero edges,
                ("graph {}", (0, 0, vec![], vec![])),
                ("graph { }", (0, 0, vec![], vec![])),
                ("graph {  }", (0, 0, vec![], vec![])),
                ("graph {\t}", (0, 0, vec![], vec![])),
                ("graph {\n}", (0, 0, vec![], vec![])),
                ("graph \"G\" {}", (0, 0, vec![], vec![])),
                ("digraph {}", (0, 0, vec![], vec![])),
                ("strict digraph {}", (0, 0, vec![], vec![])),
                // ... one vertex and zero edges,
                ("graph {1}", (1, 0, vec![1], vec![])),
                ("graph { 1}", (1, 0, vec![1], vec![])),
                ("graph {1 }", (1, 0, vec![1], vec![])),
                ("graph { 1 }", (1, 0, vec![1], vec![])),
                ("graph {\t1}", (1, 0, vec![1], vec![])),
                ("graph {\n1}", (1, 0, vec![1], vec![])),
                ("graph {1;}", (1, 0, vec![1], vec![])),
                ("graph { 1;}", (1, 0, vec![1], vec![])),
                ("graph {1; }", (1, 0, vec![1], vec![])),
                ("graph { 1; }", (1, 0, vec![1], vec![])),
                ("graph {\t1;}", (1, 0, vec![1], vec![])),
                ("graph {\n1;}", (1, 0, vec![1], vec![])),
                // ... zero vertices and one edge,
                ("graph {1 -- 2}", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("graph { 1 -- 2}", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("graph {1 -- 2 }", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("graph { 1 -- 2 }", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("graph {\t1 -- 2}", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("graph {\n1 -- 2}", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("graph {1 -- 2;}", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("graph { 1 -- 2;}", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("graph {1 -- 2; }", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("graph { 1 -- 2; }", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("graph {\t1 -- 2;}", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("graph {\n1 -- 2;}", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("digraph {1 -> 2}", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("digraph { 1 -> 2}", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("digraph {1 -> 2 }", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("digraph { 1 -> 2 }", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("digraph {\t1 -> 2}", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("digraph {\n1 -> 2}", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("digraph {1 -> 2;}", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("digraph { 1 -> 2;}", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("digraph {1 -> 2; }", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("digraph { 1 -> 2; }", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("digraph {\t1 -> 2;}", (2, 1, vec![1, 2], vec![(1, 2)])),
                ("digraph {\n1 -> 2;}", (2, 1, vec![1, 2], vec![(1, 2)])),
                // ... one vertex and one edge,
                ("graph {1 -- 2; 3}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("graph { 1 -- 2; 3}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("graph {1 -- 2 ; 3}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("graph { 1 -- 2 ; 3}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("graph {\t1 -- 2; 3}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("graph {\n1 -- 2; 3}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("graph {1 -- 2; 3;}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("graph { 1 -- 2; 3;}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("graph {1 -- 2; 3; }", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("graph { 1 -- 2; 3; }", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("graph {\t1 -- 2; 3;}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("graph {\n1 -- 2; 3;}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("digraph {1 -> 2; 3}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("digraph { 1 -> 2; 3}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("digraph {1 -> 2; 3 }", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("digraph { 1 -> 2; 3 }", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("digraph {\t1 -> 2; 3}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("digraph {\n1 -> 2; 3}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("digraph {1 -> 2; 3;}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("digraph { 1 -> 2; 3;}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("digraph {1 -> 2; 3; }", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("digraph { 1 -> 2; 3; }", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("digraph {\t1 -> 2; 3;}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                ("digraph {\n1 -> 2; 3;}", (3, 1, vec![1, 2, 3], vec![(1, 2)])),
                // ... multiple vertices and zero edges,
                ("graph {1\n2\n3}", (3, 0, vec![1, 2, 3], vec![])),
                ("graph { 1\n 2\n 3}", (3, 0, vec![1, 2, 3], vec![])),
                ("graph {1\n 2\n 3 }", (3, 0, vec![1, 2, 3], vec![])),
                ("graph { 1\n 2\n 3 }", (3, 0, vec![1, 2, 3], vec![])),
                ("graph {\t1\n\t2\n\t3}", (3, 0, vec![1, 2, 3], vec![])),
                ("graph {\n1\n2\n3}", (3, 0, vec![1, 2, 3], vec![])),
                ("graph {1;2;3;}", (3, 0, vec![1, 2, 3], vec![])),
                ("graph { 1; 2; 3;}", (3, 0, vec![1, 2, 3], vec![])),
                ("graph {1; 2; 3; }", (3, 0, vec![1, 2, 3], vec![])),
                ("graph { 1; 2; 3; }", (3, 0, vec![1, 2, 3], vec![])),
                ("graph {\t1;\t2;\t3;}", (3, 0, vec![1, 2, 3], vec![])),
                ("graph {\n1;\n2;\n3;}", (3, 0, vec![1, 2, 3], vec![])),
                // ... zero vertices and multiple edges,
                ("graph {1 -- 2\n 2 -- 3}", (3, 2, vec![1, 2, 3], vec![(1, 2), (2, 3)])),
                (
                    "graph { 1 -- 2\n 2 -- 3\n}",
                    (3, 2, vec![1, 2, 3], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph {1 -- 2\n 2 -- 3 \n}",
                    (3, 2, vec![1, 2, 3], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph { 1 -- 2\n 2 -- 3 \n}",
                    (3, 2, vec![1, 2, 3], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph {\t1 -- 2\n \t2 -- 3\n}",
                    (3, 2, vec![1, 2, 3], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph {\n1 -- 2\n2 -- 3\n}",
                    (3, 2, vec![1, 2, 3], vec![(1, 2), (2, 3)]),
                ),
                ("graph {1 -- 2; 2 -- 3;}", (3, 2, vec![1, 2, 3], vec![(1, 2), (2, 3)])),
                ("graph { 1 -- 2; 2 -- 3;}", (3, 2, vec![1, 2, 3], vec![(1, 2), (2, 3)])),
                ("graph {1 -- 2; 2 -- 3; }", (3, 2, vec![1, 2, 3], vec![(1, 2), (2, 3)])),
                ("graph { 1 -- 2; 2 -- 3; }", (3, 2, vec![1, 2, 3], vec![(1, 2), (2, 3)])),
                (
                    "graph {\t1 -- 2; \t2 -- 3;}",
                    (3, 2, vec![1, 2, 3], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph {\n1 -- 2; \n2 -- 3;}",
                    (3, 2, vec![1, 2, 3], vec![(1, 2), (2, 3)]),
                ),
                // ... multiple vertices and multiple edges,
                (
                    "graph {1 -- 2\n 2 -- 3 \n4 \n5 \n6}",
                    (6, 2, vec![1, 2, 3, 4, 5, 6], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph { 1 -- 2\n 2 -- 3\n \n4 \n5 \n6}",
                    (6, 2, vec![1, 2, 3, 4, 5, 6], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph {1 -- 2\n 2 -- 3 \n \n4 \n5 \n6 }",
                    (6, 2, vec![1, 2, 3, 4, 5, 6], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph { 1 -- 2\n 2 -- 3 \n \n4 \n5 \n6}",
                    (6, 2, vec![1, 2, 3, 4, 5, 6], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph {\t1 -- 2\n\t2 -- 3\n\n\t4\n\t5\n\t6}",
                    (6, 2, vec![1, 2, 3, 4, 5, 6], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph {\n1 -- 2\n2 -- 3\n \n4 \n5 \n6}",
                    (6, 2, vec![1, 2, 3, 4, 5, 6], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph {1 -- 2; 2 -- 3; 4; 5; 6;}",
                    (6, 2, vec![1, 2, 3, 4, 5, 6], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph { 1 -- 2; 2 -- 3; 4; 5; 6;}",
                    (6, 2, vec![1, 2, 3, 4, 5, 6], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph {1 -- 2; 2 -- 3;  4; 5; 6; }",
                    (6, 2, vec![1, 2, 3, 4, 5, 6], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph { 1 -- 2; 2 -- 3; 4; 5; 6; }",
                    (6, 2, vec![1, 2, 3, 4, 5, 6], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph {\t1 -- 2;\t2 -- 3;\t4;\t5;\t6;}",
                    (6, 2, vec![1, 2, 3, 4, 5, 6], vec![(1, 2), (2, 3)]),
                ),
                (
                    "graph {\n1 -- 2;\n2 -- 3;\n4;\n5;\n6;}",
                    (6, 2, vec![1, 2, 3, 4, 5, 6], vec![(1, 2), (2, 3)]),
                ),
            ];

            // Test for each scenario.
            for (i, j) in data {
                let g = DOT::try_from(i.to_string());
                let g: UndirectedAdjacencyList<_> = g
                    .expect("DOT parser failed")
                    .map_with_attributes(|x| x.parse::<i32>().unwrap(), |_| (), |_| (), |_| ())
                    .pop()
                    .expect("Result vec is empty");
                assert_eq!(
                    (
                        g.order(),
                        g.size(),
                        V!(g).cloned().collect::<Vec<i32>>(),
                        E!(g).map(|(&x, &y)| (x, y)).collect::<Vec<(i32, i32)>>()
                    ),
                    j.clone(),
                    "with test data: '{:?}'",
                    (i, j)
                );
            }
        }

        #[test]
        #[ignore]
        fn try_from() {
            // Test `DOT::try_from(String) -> Self`.

            // Test for ...
            let data = [
                // ... zero vertices and zero edges,
                ("graph {}", (0, 0)),
                ("graph { }", (0, 0)),
                ("graph {  }", (0, 0)),
                ("graph {\t}", (0, 0)),
                ("graph {\n}", (0, 0)),
                ("graph \"G\" {}", (0, 0)),
                ("digraph {}", (0, 0)),
                ("strict digraph {}", (0, 0)),
                // ... one vertex and zero edges,
                ("graph {A}", (1, 0)),
                ("graph { A}", (1, 0)),
                ("graph {A }", (1, 0)),
                ("graph { A }", (1, 0)),
                ("graph {\tA}", (1, 0)),
                ("graph {\nA}", (1, 0)),
                ("graph {A;}", (1, 0)),
                ("graph { A;}", (1, 0)),
                ("graph {A; }", (1, 0)),
                ("graph { A; }", (1, 0)),
                ("graph {\tA;}", (1, 0)),
                ("graph {\nA;}", (1, 0)),
                // ... zero vertices and one edge,
                ("graph {A -- B}", (2, 1)),
                ("graph { A -- B}", (2, 1)),
                ("graph {A -- B }", (2, 1)),
                ("graph { A -- B }", (2, 1)),
                ("graph {\tA -- B}", (2, 1)),
                ("graph {\nA -- B}", (2, 1)),
                ("graph {A -- B;}", (2, 1)),
                ("graph { A -- B;}", (2, 1)),
                ("graph {A -- B; }", (2, 1)),
                ("graph { A -- B; }", (2, 1)),
                ("graph {\tA -- B;}", (2, 1)),
                ("graph {\nA -- B;}", (2, 1)),
                ("digraph {A -> B}", (2, 1)),
                ("digraph { A -> B}", (2, 1)),
                ("digraph {A -> B }", (2, 1)),
                ("digraph { A -> B }", (2, 1)),
                ("digraph {\tA -> B}", (2, 1)),
                ("digraph {\nA -> B}", (2, 1)),
                ("digraph {A -> B;}", (2, 1)),
                ("digraph { A -> B;}", (2, 1)),
                ("digraph {A -> B; }", (2, 1)),
                ("digraph { A -> B; }", (2, 1)),
                ("digraph {\tA -> B;}", (2, 1)),
                ("digraph {\nA -> B;}", (2, 1)),
                // ... one vertex and one edge,
                ("graph {A -- B; C}", (3, 1)),
                ("graph { A -- B; C}", (3, 1)),
                ("graph {A -- B ; C}", (3, 1)),
                ("graph { A -- B ; C}", (3, 1)),
                ("graph {\tA -- B; C}", (3, 1)),
                ("graph {\nA -- B; C}", (3, 1)),
                ("graph {A -- B; C;}", (3, 1)),
                ("graph { A -- B; C;}", (3, 1)),
                ("graph {A -- B; C; }", (3, 1)),
                ("graph { A -- B; C; }", (3, 1)),
                ("graph {\tA -- B; C;}", (3, 1)),
                ("graph {\nA -- B; C;}", (3, 1)),
                ("digraph {A -> B; C}", (3, 1)),
                ("digraph { A -> B; C}", (3, 1)),
                ("digraph {A -> B; C }", (3, 1)),
                ("digraph { A -> B; C }", (3, 1)),
                ("digraph {\tA -> B; C}", (3, 1)),
                ("digraph {\nA -> B; C}", (3, 1)),
                ("digraph {A -> B; C;}", (3, 1)),
                ("digraph { A -> B; C;}", (3, 1)),
                ("digraph {A -> B; C; }", (3, 1)),
                ("digraph { A -> B; C; }", (3, 1)),
                ("digraph {\tA -> B; C;}", (3, 1)),
                ("digraph {\nA -> B; C;}", (3, 1)),
                // ... multiple vertices and zero edges,
                ("graph {A\nB\nC}", (3, 0)),
                ("graph { A\n B\n C}", (3, 0)),
                ("graph {A\n B\n C }", (3, 0)),
                ("graph { A\n B\n C }", (3, 0)),
                ("graph {\tA\n\tB\n\tC}", (3, 0)),
                ("graph {\nA\nB\nC}", (3, 0)),
                ("graph {A;B;C;}", (3, 0)),
                ("graph { A; B; C;}", (3, 0)),
                ("graph {A; B; C; }", (3, 0)),
                ("graph { A; B; C; }", (3, 0)),
                ("graph {\tA;\tB;\tC;}", (3, 0)),
                ("graph {\nA;\nB;\nC;}", (3, 0)),
                // ... zero vertices and multiple edges,
                ("graph {A -- B\n B -- C}", (3, 2)),
                ("graph { A -- B\n B -- C\n}", (3, 2)),
                ("graph {A -- B\n B -- C \n}", (3, 2)),
                ("graph { A -- B\n B -- C \n}", (3, 2)),
                ("graph {\tA -- B\n \tB -- C\n}", (3, 2)),
                ("graph {\nA -- B\nB -- C\n}", (3, 2)),
                ("graph {A -- B; B -- C;}", (3, 2)),
                ("graph { A -- B; B -- C;}", (3, 2)),
                ("graph {A -- B; B -- C; }", (3, 2)),
                ("graph { A -- B; B -- C; }", (3, 2)),
                ("graph {\tA -- B; \tB -- C;}", (3, 2)),
                ("graph {\nA -- B; \nB -- C;}", (3, 2)),
                // ... multiple vertices and multiple edges,
                ("graph {A -- B\n B -- C \nD \nE \nF}", (6, 2)),
                ("graph { A -- B\n B -- C\n \nD \nE \nF}", (6, 2)),
                ("graph {A -- B\n B -- C \n \nD \nE \nF }", (6, 2)),
                ("graph { A -- B\n B -- C \n \nD \nE \nF}", (6, 2)),
                ("graph {\tA -- B\n\tB -- C\n\n\tD\n\tE\n\tF}", (6, 2)),
                ("graph {\nA -- B\nB -- C\n \nD \nE \nF}", (6, 2)),
                ("graph {A -- B; B -- C; D; E; F;}", (6, 2)),
                ("graph { A -- B; B -- C; D; E; F;}", (6, 2)),
                ("graph {A -- B; B -- C;  D; E; F; }", (6, 2)),
                ("graph { A -- B; B -- C; D; E; F; }", (6, 2)),
                ("graph {\tA -- B;\tB -- C;\tD;\tE;\tF;}", (6, 2)),
                ("graph {\nA -- B;\nB -- C;\nD;\nE;\nF;}", (6, 2)),
            ];

            // FIXME: Test for each scenario.
            /* for (i, j) in data {
                let g = DOT::try_from(i.to_string());
                let g: UndirectedAdjacencyList<_> = g
                    .expect("DOT parser failed")
                    .map(|x| x)
                    .pop()
                    .expect("Result vec is empty");
                assert_eq!((g.order(), g.size()), j, "with test data: '{:?}'", (i, j));
            } */
        }

        #[test]
        fn try_into() {
            // Test `DOT::write(String) -> Result`.

            // Test for ...
            let data = [
                // ... zero vertices and zero edges,
                ("graph {}", "graph {\n}\n"),
                ("graph { }", "graph {\n}\n"),
                ("graph {  }", "graph {\n}\n"),
                ("graph {\t}", "graph {\n}\n"),
                ("graph {\n}", "graph {\n}\n"),
                ("graph \"G\" {}", "graph \"G\" {\n}\n"),
                ("digraph {}", "digraph {\n}\n"),
                ("strict digraph {}", "strict digraph {\n}\n"),
                // ... one vertex and zero edges,
                ("graph {A}", "graph {\n\t\"A\";\n}\n"),
                ("graph { A}", "graph {\n\t\"A\";\n}\n"),
                ("graph {A }", "graph {\n\t\"A\";\n}\n"),
                ("graph { A }", "graph {\n\t\"A\";\n}\n"),
                ("graph {\tA}", "graph {\n\t\"A\";\n}\n"),
                ("graph {\nA}", "graph {\n\t\"A\";\n}\n"),
                ("graph {A;}", "graph {\n\t\"A\";\n}\n"),
                ("graph { A;}", "graph {\n\t\"A\";\n}\n"),
                ("graph {A; }", "graph {\n\t\"A\";\n}\n"),
                ("graph { A; }", "graph {\n\t\"A\";\n}\n"),
                ("graph {\tA;}", "graph {\n\t\"A\";\n}\n"),
                ("graph {\nA;}", "graph {\n\t\"A\";\n}\n"),
                // ... zero vertices and one edge,
                ("graph {A -- B}", "graph {\n\t\"A\" -- \"B\";\n}\n"),
                ("graph { A -- B}", "graph {\n\t\"A\" -- \"B\";\n}\n"),
                ("graph {A -- B }", "graph {\n\t\"A\" -- \"B\";\n}\n"),
                ("graph { A -- B }", "graph {\n\t\"A\" -- \"B\";\n}\n"),
                ("graph {\tA -- B}", "graph {\n\t\"A\" -- \"B\";\n}\n"),
                ("graph {\nA -- B}", "graph {\n\t\"A\" -- \"B\";\n}\n"),
                ("graph {A -- B;}", "graph {\n\t\"A\" -- \"B\";\n}\n"),
                ("graph { A -- B;}", "graph {\n\t\"A\" -- \"B\";\n}\n"),
                ("graph {A -- B; }", "graph {\n\t\"A\" -- \"B\";\n}\n"),
                ("graph { A -- B; }", "graph {\n\t\"A\" -- \"B\";\n}\n"),
                ("graph {\tA -- B;}", "graph {\n\t\"A\" -- \"B\";\n}\n"),
                ("graph {\nA -- B;}", "graph {\n\t\"A\" -- \"B\";\n}\n"),
                ("digraph {A -> B}", "digraph {\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph { A -> B}", "digraph {\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph {A -> B }", "digraph {\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph { A -> B }", "digraph {\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph {\tA -> B}", "digraph {\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph {\nA -> B}", "digraph {\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph {A -> B;}", "digraph {\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph { A -> B;}", "digraph {\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph {A -> B; }", "digraph {\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph { A -> B; }", "digraph {\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph {\tA -> B;}", "digraph {\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph {\nA -> B;}", "digraph {\n\t\"A\" -> \"B\";\n}\n"),
                // ... one vertex and one edge,
                ("graph {A -- B; C}", "graph {\n\t\"C\";\n\t\"A\" -- \"B\";\n}\n"),
                ("graph { A -- B; C}", "graph {\n\t\"C\";\n\t\"A\" -- \"B\";\n}\n"),
                ("graph {A -- B ; C}", "graph {\n\t\"C\";\n\t\"A\" -- \"B\";\n}\n"),
                ("graph { A -- B ; C}", "graph {\n\t\"C\";\n\t\"A\" -- \"B\";\n}\n"),
                ("graph {\tA -- B; C}", "graph {\n\t\"C\";\n\t\"A\" -- \"B\";\n}\n"),
                ("graph {\nA -- B; C}", "graph {\n\t\"C\";\n\t\"A\" -- \"B\";\n}\n"),
                ("graph {A -- B; C;}", "graph {\n\t\"C\";\n\t\"A\" -- \"B\";\n}\n"),
                ("graph { A -- B; C;}", "graph {\n\t\"C\";\n\t\"A\" -- \"B\";\n}\n"),
                ("graph {A -- B; C; }", "graph {\n\t\"C\";\n\t\"A\" -- \"B\";\n}\n"),
                ("graph { A -- B; C; }", "graph {\n\t\"C\";\n\t\"A\" -- \"B\";\n}\n"),
                ("graph {\tA -- B; C;}", "graph {\n\t\"C\";\n\t\"A\" -- \"B\";\n}\n"),
                ("graph {\nA -- B; C;}", "graph {\n\t\"C\";\n\t\"A\" -- \"B\";\n}\n"),
                ("digraph {A -> B; C}", "digraph {\n\t\"C\";\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph { A -> B; C}", "digraph {\n\t\"C\";\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph {A -> B; C }", "digraph {\n\t\"C\";\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph { A -> B; C }", "digraph {\n\t\"C\";\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph {\tA -> B; C}", "digraph {\n\t\"C\";\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph {\nA -> B; C}", "digraph {\n\t\"C\";\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph {A -> B; C;}", "digraph {\n\t\"C\";\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph { A -> B; C;}", "digraph {\n\t\"C\";\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph {A -> B; C; }", "digraph {\n\t\"C\";\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph { A -> B; C; }", "digraph {\n\t\"C\";\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph {\tA -> B; C;}", "digraph {\n\t\"C\";\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph {\nA -> B; C;}", "digraph {\n\t\"C\";\n\t\"A\" -> \"B\";\n}\n"),
                // ... multiple vertices and zero edges,
                ("graph {A\nB\nC}", "graph {\n\t\"A\";\n\t\"B\";\n\t\"C\";\n}\n"),
                ("graph { A\n B\n C}", "graph {\n\t\"A\";\n\t\"B\";\n\t\"C\";\n}\n"),
                ("graph {A\n B\n C }", "graph {\n\t\"A\";\n\t\"B\";\n\t\"C\";\n}\n"),
                ("graph { A\n B\n C }", "graph {\n\t\"A\";\n\t\"B\";\n\t\"C\";\n}\n"),
                ("graph {\tA\n\tB\n\tC}", "graph {\n\t\"A\";\n\t\"B\";\n\t\"C\";\n}\n"),
                ("graph {\nA\nB\nC}", "graph {\n\t\"A\";\n\t\"B\";\n\t\"C\";\n}\n"),
                ("graph {A;B;C;}", "graph {\n\t\"A\";\n\t\"B\";\n\t\"C\";\n}\n"),
                ("graph { A; B; C;}", "graph {\n\t\"A\";\n\t\"B\";\n\t\"C\";\n}\n"),
                ("graph {A; B; C; }", "graph {\n\t\"A\";\n\t\"B\";\n\t\"C\";\n}\n"),
                ("graph { A; B; C; }", "graph {\n\t\"A\";\n\t\"B\";\n\t\"C\";\n}\n"),
                ("graph {\tA;\tB;\tC;}", "graph {\n\t\"A\";\n\t\"B\";\n\t\"C\";\n}\n"),
                ("graph {\nA;\nB;\nC;}", "graph {\n\t\"A\";\n\t\"B\";\n\t\"C\";\n}\n"),
                // ... zero vertices and multiple edges,
                (
                    "graph {A -- B\n B -- C}",
                    "graph {\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph { A -- B\n B -- C\n}",
                    "graph {\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph {A -- B\n B -- C \n}",
                    "graph {\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph { A -- B\n B -- C \n}",
                    "graph {\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph {\tA -- B\n \tB -- C\n}",
                    "graph {\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph {\nA -- B\nB -- C\n}",
                    "graph {\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph {A -- B; B -- C;}",
                    "graph {\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph { A -- B; B -- C;}",
                    "graph {\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph {A -- B; B -- C; }",
                    "graph {\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph { A -- B; B -- C; }",
                    "graph {\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph {\tA -- B; \tB -- C;}",
                    "graph {\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph {\nA -- B; \nB -- C;}",
                    "graph {\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                // ... multiple vertices and multiple edges,
                (
                    "graph {A -- B\n B -- C \nD \nE \nF}",
                    "graph {\n\t\"D\";\n\t\"E\";\n\t\"F\";\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph { A -- B\n B -- C\n \nD \nE \nF}",
                    "graph {\n\t\"D\";\n\t\"E\";\n\t\"F\";\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph {A -- B\n B -- C \n \nD \nE \nF }",
                    "graph {\n\t\"D\";\n\t\"E\";\n\t\"F\";\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph { A -- B\n B -- C \n \nD \nE \nF}",
                    "graph {\n\t\"D\";\n\t\"E\";\n\t\"F\";\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph {\tA -- B\n\tB -- C\n\n\tD\n\tE\n\tF}",
                    "graph {\n\t\"D\";\n\t\"E\";\n\t\"F\";\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph {\nA -- B\nB -- C\n \nD \nE \nF}",
                    "graph {\n\t\"D\";\n\t\"E\";\n\t\"F\";\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph {A -- B; B -- C; D; E; F;}",
                    "graph {\n\t\"D\";\n\t\"E\";\n\t\"F\";\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph { A -- B; B -- C; D; E; F;}",
                    "graph {\n\t\"D\";\n\t\"E\";\n\t\"F\";\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph {A -- B; B -- C;  D; E; F; }",
                    "graph {\n\t\"D\";\n\t\"E\";\n\t\"F\";\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph { A -- B; B -- C; D; E; F; }",
                    "graph {\n\t\"D\";\n\t\"E\";\n\t\"F\";\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph {\tA -- B;\tB -- C;\tD;\tE;\tF;}",
                    "graph {\n\t\"D\";\n\t\"E\";\n\t\"F\";\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph {\nA -- B;\nB -- C;\nD;\nE;\nF;}",
                    "graph {\n\t\"D\";\n\t\"E\";\n\t\"F\";\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
            ];

            // Test for each scenario.
            for (i, j) in data {
                let g = DOT::try_from(i.to_string()).expect("DOT try_from failed");
                let g = TryInto::<String>::try_into(g).expect("DOT try_into failed");

                assert_eq!(g, j);
            }
        }

        #[test]
        fn read() {
            // Test for each scenario.
            std::fs::read_dir("tests/data")
                .expect("No such file or directory")
                .map(|x| x.unwrap().path())
                .filter(|x| !x.extension().unwrap().eq("ignore"))
                .for_each(|x| {
                    assert!(DOT::read(&x).is_ok());
                });
        }

        #[test]
        fn write() {
            // Test `DOT::write(String) -> Result`.

            // Test for ...
            let data = [
                // ... zero vertices and zero edges,
                ("graph {}", "graph {\n}\n"),
                ("graph { }", "graph {\n}\n"),
                ("graph {  }", "graph {\n}\n"),
                ("graph {\t}", "graph {\n}\n"),
                ("graph {\n}", "graph {\n}\n"),
                ("graph \"G\" {}", "graph \"G\" {\n}\n"),
                ("digraph {}", "digraph {\n}\n"),
                ("strict digraph {}", "strict digraph {\n}\n"),
                // ... one vertex and zero edges,
                ("graph {A}", "graph {\n\t\"A\";\n}\n"),
                ("graph { A}", "graph {\n\t\"A\";\n}\n"),
                ("graph {A }", "graph {\n\t\"A\";\n}\n"),
                ("graph { A }", "graph {\n\t\"A\";\n}\n"),
                ("graph {\tA}", "graph {\n\t\"A\";\n}\n"),
                ("graph {\nA}", "graph {\n\t\"A\";\n}\n"),
                ("graph {A;}", "graph {\n\t\"A\";\n}\n"),
                ("graph { A;}", "graph {\n\t\"A\";\n}\n"),
                ("graph {A; }", "graph {\n\t\"A\";\n}\n"),
                ("graph { A; }", "graph {\n\t\"A\";\n}\n"),
                ("graph {\tA;}", "graph {\n\t\"A\";\n}\n"),
                ("graph {\nA;}", "graph {\n\t\"A\";\n}\n"),
                // ... zero vertices and one edge,
                ("graph {A -- B}", "graph {\n\t\"A\" -- \"B\";\n}\n"),
                ("graph { A -- B}", "graph {\n\t\"A\" -- \"B\";\n}\n"),
                ("graph {A -- B }", "graph {\n\t\"A\" -- \"B\";\n}\n"),
                ("graph { A -- B }", "graph {\n\t\"A\" -- \"B\";\n}\n"),
                ("graph {\tA -- B}", "graph {\n\t\"A\" -- \"B\";\n}\n"),
                ("graph {\nA -- B}", "graph {\n\t\"A\" -- \"B\";\n}\n"),
                ("graph {A -- B;}", "graph {\n\t\"A\" -- \"B\";\n}\n"),
                ("graph { A -- B;}", "graph {\n\t\"A\" -- \"B\";\n}\n"),
                ("graph {A -- B; }", "graph {\n\t\"A\" -- \"B\";\n}\n"),
                ("graph { A -- B; }", "graph {\n\t\"A\" -- \"B\";\n}\n"),
                ("graph {\tA -- B;}", "graph {\n\t\"A\" -- \"B\";\n}\n"),
                ("graph {\nA -- B;}", "graph {\n\t\"A\" -- \"B\";\n}\n"),
                ("digraph {A -> B}", "digraph {\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph { A -> B}", "digraph {\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph {A -> B }", "digraph {\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph { A -> B }", "digraph {\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph {\tA -> B}", "digraph {\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph {\nA -> B}", "digraph {\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph {A -> B;}", "digraph {\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph { A -> B;}", "digraph {\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph {A -> B; }", "digraph {\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph { A -> B; }", "digraph {\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph {\tA -> B;}", "digraph {\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph {\nA -> B;}", "digraph {\n\t\"A\" -> \"B\";\n}\n"),
                // ... one vertex and one edge,
                ("graph {A -- B; C}", "graph {\n\t\"C\";\n\t\"A\" -- \"B\";\n}\n"),
                ("graph { A -- B; C}", "graph {\n\t\"C\";\n\t\"A\" -- \"B\";\n}\n"),
                ("graph {A -- B ; C}", "graph {\n\t\"C\";\n\t\"A\" -- \"B\";\n}\n"),
                ("graph { A -- B ; C}", "graph {\n\t\"C\";\n\t\"A\" -- \"B\";\n}\n"),
                ("graph {\tA -- B; C}", "graph {\n\t\"C\";\n\t\"A\" -- \"B\";\n}\n"),
                ("graph {\nA -- B; C}", "graph {\n\t\"C\";\n\t\"A\" -- \"B\";\n}\n"),
                ("graph {A -- B; C;}", "graph {\n\t\"C\";\n\t\"A\" -- \"B\";\n}\n"),
                ("graph { A -- B; C;}", "graph {\n\t\"C\";\n\t\"A\" -- \"B\";\n}\n"),
                ("graph {A -- B; C; }", "graph {\n\t\"C\";\n\t\"A\" -- \"B\";\n}\n"),
                ("graph { A -- B; C; }", "graph {\n\t\"C\";\n\t\"A\" -- \"B\";\n}\n"),
                ("graph {\tA -- B; C;}", "graph {\n\t\"C\";\n\t\"A\" -- \"B\";\n}\n"),
                ("graph {\nA -- B; C;}", "graph {\n\t\"C\";\n\t\"A\" -- \"B\";\n}\n"),
                ("digraph {A -> B; C}", "digraph {\n\t\"C\";\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph { A -> B; C}", "digraph {\n\t\"C\";\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph {A -> B; C }", "digraph {\n\t\"C\";\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph { A -> B; C }", "digraph {\n\t\"C\";\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph {\tA -> B; C}", "digraph {\n\t\"C\";\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph {\nA -> B; C}", "digraph {\n\t\"C\";\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph {A -> B; C;}", "digraph {\n\t\"C\";\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph { A -> B; C;}", "digraph {\n\t\"C\";\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph {A -> B; C; }", "digraph {\n\t\"C\";\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph { A -> B; C; }", "digraph {\n\t\"C\";\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph {\tA -> B; C;}", "digraph {\n\t\"C\";\n\t\"A\" -> \"B\";\n}\n"),
                ("digraph {\nA -> B; C;}", "digraph {\n\t\"C\";\n\t\"A\" -> \"B\";\n}\n"),
                // ... multiple vertices and zero edges,
                ("graph {A\nB\nC}", "graph {\n\t\"A\";\n\t\"B\";\n\t\"C\";\n}\n"),
                ("graph { A\n B\n C}", "graph {\n\t\"A\";\n\t\"B\";\n\t\"C\";\n}\n"),
                ("graph {A\n B\n C }", "graph {\n\t\"A\";\n\t\"B\";\n\t\"C\";\n}\n"),
                ("graph { A\n B\n C }", "graph {\n\t\"A\";\n\t\"B\";\n\t\"C\";\n}\n"),
                ("graph {\tA\n\tB\n\tC}", "graph {\n\t\"A\";\n\t\"B\";\n\t\"C\";\n}\n"),
                ("graph {\nA\nB\nC}", "graph {\n\t\"A\";\n\t\"B\";\n\t\"C\";\n}\n"),
                ("graph {A;B;C;}", "graph {\n\t\"A\";\n\t\"B\";\n\t\"C\";\n}\n"),
                ("graph { A; B; C;}", "graph {\n\t\"A\";\n\t\"B\";\n\t\"C\";\n}\n"),
                ("graph {A; B; C; }", "graph {\n\t\"A\";\n\t\"B\";\n\t\"C\";\n}\n"),
                ("graph { A; B; C; }", "graph {\n\t\"A\";\n\t\"B\";\n\t\"C\";\n}\n"),
                ("graph {\tA;\tB;\tC;}", "graph {\n\t\"A\";\n\t\"B\";\n\t\"C\";\n}\n"),
                ("graph {\nA;\nB;\nC;}", "graph {\n\t\"A\";\n\t\"B\";\n\t\"C\";\n}\n"),
                // ... zero vertices and multiple edges,
                (
                    "graph {A -- B\n B -- C}",
                    "graph {\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph { A -- B\n B -- C\n}",
                    "graph {\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph {A -- B\n B -- C \n}",
                    "graph {\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph { A -- B\n B -- C \n}",
                    "graph {\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph {\tA -- B\n \tB -- C\n}",
                    "graph {\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph {\nA -- B\nB -- C\n}",
                    "graph {\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph {A -- B; B -- C;}",
                    "graph {\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph { A -- B; B -- C;}",
                    "graph {\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph {A -- B; B -- C; }",
                    "graph {\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph { A -- B; B -- C; }",
                    "graph {\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph {\tA -- B; \tB -- C;}",
                    "graph {\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph {\nA -- B; \nB -- C;}",
                    "graph {\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                // ... multiple vertices and multiple edges,
                (
                    "graph {A -- B\n B -- C \nD \nE \nF}",
                    "graph {\n\t\"D\";\n\t\"E\";\n\t\"F\";\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph { A -- B\n B -- C\n \nD \nE \nF}",
                    "graph {\n\t\"D\";\n\t\"E\";\n\t\"F\";\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph {A -- B\n B -- C \n \nD \nE \nF }",
                    "graph {\n\t\"D\";\n\t\"E\";\n\t\"F\";\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph { A -- B\n B -- C \n \nD \nE \nF}",
                    "graph {\n\t\"D\";\n\t\"E\";\n\t\"F\";\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph {\tA -- B\n\tB -- C\n\n\tD\n\tE\n\tF}",
                    "graph {\n\t\"D\";\n\t\"E\";\n\t\"F\";\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph {\nA -- B\nB -- C\n \nD \nE \nF}",
                    "graph {\n\t\"D\";\n\t\"E\";\n\t\"F\";\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph {A -- B; B -- C; D; E; F;}",
                    "graph {\n\t\"D\";\n\t\"E\";\n\t\"F\";\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph { A -- B; B -- C; D; E; F;}",
                    "graph {\n\t\"D\";\n\t\"E\";\n\t\"F\";\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph {A -- B; B -- C;  D; E; F; }",
                    "graph {\n\t\"D\";\n\t\"E\";\n\t\"F\";\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph { A -- B; B -- C; D; E; F; }",
                    "graph {\n\t\"D\";\n\t\"E\";\n\t\"F\";\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph {\tA -- B;\tB -- C;\tD;\tE;\tF;}",
                    "graph {\n\t\"D\";\n\t\"E\";\n\t\"F\";\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
                (
                    "graph {\nA -- B;\nB -- C;\nD;\nE;\nF;}",
                    "graph {\n\t\"D\";\n\t\"E\";\n\t\"F\";\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
                ),
            ];

            // Test for each scenario.
            for (i, j) in data {
                // Create temporary file.
                let path = NamedTempFile::new().expect("TempFile failed").into_temp_path();
                // Write DOT to file.
                let g = DOT::try_from(i.to_string()).unwrap();

                assert!(g.write(&path).is_ok());

                // Read back file into DOT.
                let g = DOT::read(&path).expect("DOT read failed");
                let g = TryInto::<String>::try_into(g).expect("DOT try_into failed");

                assert_eq!(g, j);
            }
        }
    }
}
