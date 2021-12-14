#[cfg(test)]
mod tests {
    use crate::errors::Error;
    use crate::graphs::UndirectedAdjacencyListGraph;
    use crate::storages::StorageTrait;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;

    // Set DOT string
    const DOT: &str = "graph {\n\t\"A\";\n\t\"B\";\n\t\"A\" -- \"B\";\n}\n";

    fn load_test_data() -> Vec<PathBuf> {
        std::fs::read_dir("src/tests/data")
            .expect("No such file or directory.")
            .map(|x| x.unwrap().path())
            .filter(|x| !x.extension().unwrap().eq("ignore"))
            .collect()
    }

    #[test]
    fn from_dot() -> Result<(), Error<String>> {
        let mut g = UndirectedAdjacencyListGraph::<String>::new();
        let i = g.add_vertex(&"A")?;
        let j = g.add_vertex(&"B")?;
        g.add_edge((i, j))?;
        let h = crate::io::from_dot::<UndirectedAdjacencyListGraph<String>>(DOT)
            .unwrap()
            .pop()
            .unwrap();
        assert_eq!(g, h);

        Ok(())
    }

    #[test]
    fn read_dot() {
        for path in load_test_data() {
            let parsed =
                crate::io::read_dot::<UndirectedAdjacencyListGraph<String>>(&path).unwrap();
            println!("{:?}", parsed);
        }
    }

    #[test]
    fn write_dot() {
        // Load graph from DOT string
        let g = crate::io::from_dot::<UndirectedAdjacencyListGraph<String>>(DOT)
            .unwrap()
            .pop()
            .unwrap();
        // Get temporary file path
        let path = NamedTempFile::new().unwrap().into_temp_path();
        // Write to DOT file
        crate::io::write_dot(&path, &g).unwrap();
        // Read from DOT file
        let h = crate::io::read_dot::<UndirectedAdjacencyListGraph<String>>(&path)
            .unwrap()
            .pop()
            .unwrap();
        // Compare
        assert_eq!(g, h);
    }
}
