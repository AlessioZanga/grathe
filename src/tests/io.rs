#[cfg(test)]
mod tests {
    mod dot {
        use crate::io::{DOT, IO};
        use tempfile::NamedTempFile;

        const DATA: [(&str, &str); 7] = [
            (
                "graph {\n}\n",
                "graph {\n}\n",
            ),
            (
                "digraph {\n}\n",
                "digraph {\n}\n",
            ),
            (
                "strict graph {\n}\n",
                "strict graph {\n}\n",
            ),
            (
                "strict graph \"G\" {\n}\n",
                "strict graph \"G\" {\n}\n",
            ),
            (
                "graph {\n\t\"A\";\n\t\"B\";\n\t\"A\" -- \"B\";\n}\n",
                "graph {\n\t\"A\";\n\t\"B\";\n\t\"A\" -- \"B\";\n}\n",
            ),
            (
                "graph {\n\t\"A\";\n\t\"B\";\n\t\"C\";\n\t\"A\" -- \"B\" -- \"C\";\n}\n",
                "graph {\n\t\"A\";\n\t\"B\";\n\t\"C\";\n\t\"A\" -- \"B\";\n\t\"B\" -- \"C\";\n}\n",
            ),
            (
                "graph {\n\t\"color\"=\"blue\";\n\t\"A\";\n\t\"B\";\n\t\"A\" -- \"B\";\n}\n",
                "graph {\n\t\"color\"=\"blue\";\n\t\"A\";\n\t\"B\";\n\t\"A\" -- \"B\";\n}\n",
            ),
        ];

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
