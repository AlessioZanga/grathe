#[cfg(test)]
mod tests {
    mod dot {
        use crate::graphs::attributes::AttributesMap;
        use crate::graphs::storages::UndirectedAdjacencyList;
        use crate::io::{DOT, IO};
        use crate::traits::{Storage, WithAttributes};
        use all_asserts::*;
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
        fn map() -> Result<(), std::io::Error> {
            let g = "graph G { 1 -- 2; }";
            let g = DOT::try_from(g.to_string()).unwrap();
            let g = g.map::<UndirectedAdjacencyList<i32>, _>(|x| x.parse::<i32>().unwrap());
            let g = &g[0];

            assert_eq!(g.order(), 2);
            assert_eq!(g.size(), 1);
            assert_true!(g.vertices_iter().eq(&[1, 2]));

            Ok(())
        }

        #[test]
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
            for (k, _) in DATA {
                assert!(DOT::try_from(k.to_string()).is_ok());
            }
        }

        #[test]
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
            std::fs::read_dir("src/tests/data")
                .expect("No such file or directory.")
                .map(|x| x.unwrap().path())
                .filter(|x| !x.extension().unwrap().eq("ignore"))
                .for_each(|x| {
                    assert!(DOT::read(&x).is_ok());
                });
        }

        #[test]
        fn write() {
            for (k, v) in DATA {
                let path = NamedTempFile::new().unwrap().into_temp_path();
                let dot = DOT::try_from(k.to_string()).unwrap();
                assert!(dot.write(&path).is_ok());
                let dot = DOT::read(&path).unwrap();
                let dot = TryInto::<String>::try_into(dot);
                assert!(dot.is_ok());
                assert_eq!(dot.unwrap(), v);
            }
        }
    }
}
