#[cfg(test)]
mod tests {
    use crate::errors::Error;
    use crate::graphs::UndirectedAdjacencyListGraph;
    use crate::traits::convert::{FromDOT, IntoDOT};
    use crate::traits::Base;
    use crate::traits::Storage;
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
        g.add_edge(&i, &j)?;
        let h = crate::io::from_dot::<UndirectedAdjacencyListGraph<String>>(DOT)
            .unwrap()
            .pop()
            .unwrap();
        assert_eq!(g, h);

        Ok(())
    }

    #[test]
    fn to_dot() -> Result<(), Error<i32>> {
        let mut g = UndirectedAdjacencyListGraph::<i32>::new();
        let i = g.add_vertex(&0)?;
        let j = g.add_vertex(&1)?;
        g.set_vertex_attr(&i, "label", "A")?;
        g.set_vertex_attr(&j, "label", String::from("B"))?;
        assert_eq!(
            g.into_dot().as_str(),
            "graph {\n\t0 [label=\"A\"];\n\t1 [label=\"B\"];\n}\n"
        );

        Ok(())
    }

    #[test]
    fn read_dot() {
        for path in load_test_data() {
            let parsed = UndirectedAdjacencyListGraph::<String>::read_dot(&path).unwrap();
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
        g.write_dot(&path).unwrap();
        // Read from DOT file
        let h = UndirectedAdjacencyListGraph::<String>::read_dot(&path).unwrap();
        // Compare
        assert_eq!(g, h);
    }
}
