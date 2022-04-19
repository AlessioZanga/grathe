#[cfg(test)]
mod io {
    mod dot {
        use all_asserts::*;
        use grathe::{
            graphs::{attributes::AttributesMap, storages::UndirectedAdjacencyList},
            io::{DOT, IO},
            traits::{Storage, WithAttributes},
        };
        use tempfile::NamedTempFile;

        const DATA: [(&str, &str); 13] = [
            ("graph {}", "graph {\n}\n"),
            ("digraph {}", "digraph {\n}\n"),
            ("strict graph {}", "strict graph {\n}\n"),
            ("strict graph G {}", "strict graph \"G\" {\n}\n"),
            ("strict graph \"G\" {}", "strict graph \"G\" {\n}\n"),
            ("graph { A; B; }", "graph {\n\t\"A\";\n\t\"B\";\n}\n"),
            ("graph { A -- B; }", "graph {\n\t\"A\" -- \"B\";\n}\n"),
            (
                "graph { A; B; A -- B; }",
                "graph {\n\t\"A\";\n\t\"B\";\n\t\"A\" -- \"B\";\n}\n",
            ),
            (
                "graph { \"A\"; \"B\"; \"A\" -- \"B\"; }",
                "graph {\n\t\"A\";\n\t\"B\";\n\t\"A\" -- \"B\";\n}\n",
            ),
            (
                "graph { \"A\"; \"B\"; \"C\"; \"A\" -- \"B\" -- \"C\"; }",
                "graph {\n\t\"A\";\n\t\"B\";\n\t\"C\";\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
            ),
            (
                "graph { \"color\" = \"blue\"; \"A\"; \"B\"; \"A\" -- \"B\"; }",
                "graph {\n\t\"color\" = \"blue\";\n\t\"A\";\n\t\"B\";\n\t\"A\" -- \"B\";\n}\n",
            ),
            (
                "graph { \"A\" [\"color\" = \"blue\"]; \"B\"; \"A\" -- \"B\"; }",
                "graph {\n\t\"A\" [\"color\" = \"blue\"];\n\t\"B\";\n\t\"A\" -- \"B\";\n}\n",
            ),
            (
                "digraph { \"A\"; \"B\"; \"A\" -> \"B\" [\"color\" = \"blue\"]; }",
                "digraph {\n\t\"A\";\n\t\"B\";\n\t\"A\" -> \"B\" [\"color\" = \"blue\"];\n}\n",
            ),
        ];

        #[test]
        #[ignore]
        fn map() -> Result<(), std::io::Error> {
            let g = "graph G { 1 -- 2; }";
            let g = DOT::try_from(g.to_string()).unwrap();
            let g = g.map::<UndirectedAdjacencyList<i32>, _>(|x| x.parse::<i32>().unwrap());
            let g = &g[0];

            assert_eq!(g.order(), 2);
            assert_eq!(g.size(), 0);
            assert_true!(g.vertices_iter().eq(&[1, 2]));

            Ok(())
        }

        #[test]
        #[ignore]
        // FIXME:
        fn map_with_attributes() -> Result<(), std::io::Error> {
            let g = "graph G { 1 -- 2; }";
            let g = DOT::try_from(g.to_string()).unwrap();
            let g = g
                .map_with_attributes::<UndirectedAdjacencyList<i32, AttributesMap<i32, String, (), ()>>, _, _, _, _>(
                    |t| t.parse::<i32>().unwrap(),
                    |mut x| x.remove("graph_id").unwrap(),
                    |_| (),
                    |_| (),
                );
            let g = &g[0];

            assert_eq!(g.order(), 2);
            assert_eq!(g.size(), 1);
            assert_true!(g.vertices_iter().eq(&[1, 2]));
            assert_true!(g.has_graph_attrs());

            Ok(())
        }

        #[test]
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

            // Test for each scenario.
            for (i, j) in data {
                let g = DOT::try_from(i.to_string());
                let g: UndirectedAdjacencyList<_> = g
                    .expect("DOT parser failed")
                    .map(|x| x)
                    .pop()
                    .expect("Result vec is empty");
                assert_eq!((g.order(), g.size()), j, "with test data: '{:?}'", (i, j));
            }
        }

        #[test]
        #[ignore]
        // FIXME:
        fn try_into() {
            for (k, v) in DATA {
                let dot = DOT::try_from(k.to_string()).unwrap();
                let dot = TryInto::<String>::try_into(dot);
                assert!(dot.is_ok());
                assert_eq!(dot.unwrap(), v);
            }
        }

        #[test]
        fn read() {
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
            // Test `DOT::try_from(String) -> Self`.

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
