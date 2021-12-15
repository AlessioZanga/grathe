#[cfg(test)]
#[generic_tests::define]
mod tests {
    use crate::errors::Error;
    use crate::graphs::{GraphTrait, UndirectedAdjacencyListGraph};
    use std::path::Path;
    use tempfile::NamedTempFile;

    // Set DOT string
    const DOT: &str = "graph {\n\t\"A\";\n\t\"B\";\n\t\"A\" -- \"B\";\n}\n";

    #[test]
    fn from_dot<T>() -> Result<(), Error<String>>
    where
        T: GraphTrait<Vertex = String>,
    {
        // Read DOT file to string
        let dot = std::fs::read_to_string("src/tests/data/14.dot").unwrap();
        // Load graph using io function
        let g = crate::io::from_dot::<T>(&dot).unwrap().pop().unwrap();
        // Test from DOT string constructor
        assert_eq!(T::from_dot(&dot)?, g);

        Ok(())
    }

    #[test]
    fn to_dot<T>() -> Result<(), Error<String>>
    where
        T: GraphTrait<Vertex = String>,
    {
        // Init graph
        let mut g = T::new();
        let i = g.add_vertex(&"A")?;
        let j = g.add_vertex(&"B")?;
        g.add_edge(&i, &j)?;
        // Test
        assert_eq!(g.to_dot()?, DOT);

        Ok(())
    }

    #[test]
    fn read_dot<T>() -> Result<(), Error<String>>
    where
        T: GraphTrait<Vertex = String>,
    {
        // Read DOT file to string
        let dot = Path::new("src/tests/data/14.dot");
        // Load graph using io function
        let g = crate::io::read_dot::<T>(dot).unwrap().pop().unwrap();
        // Test from DOT string constructor
        assert_eq!(T::read_dot(dot)?, g);

        Ok(())
    }

    #[test]
    fn write_dot<T>() -> Result<(), Error<String>>
    where
        T: GraphTrait<Vertex = String>,
    {
        // Init graph
        let mut g = T::new();
        let i = g.add_vertex(&"A")?;
        let j = g.add_vertex(&"B")?;
        g.add_edge(&i, &j)?;
        // Get temporary file path
        let path = NamedTempFile::new().unwrap().into_temp_path();
        // Write to DOT file
        g.write_dot(&path)?;
        // Test
        assert_eq!(g, T::read_dot(&path)?);

        Ok(())
    }

    #[instantiate_tests(<UndirectedAdjacencyListGraph<String>>)]
    mod undirected_adjacency_list_graph {}
}
