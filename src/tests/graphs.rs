#[cfg(test)]
#[generic_tests::define]
mod tests {
    use crate::errors::*;
    use crate::graphs::{GraphTrait, UndirectedAdjacencyListGraph};
    use std::path::Path;
    use tempfile::NamedTempFile;

    // Set DOT string
    const DOT: &str =
        "graph {\n\t0 [label=\"A\"];\n\t1 [label=\"B\"];\n\t0 -- 1 [label=\"A -- B\"];\n}\n";

    #[test]
    fn from_dot<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32>,
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
    fn to_dot<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32>,
    {
        // Init graph
        let mut g = T::default();
        let i = g.add_vertex_label("A")?;
        let j = g.add_vertex_label("B")?;
        g.add_edge_label(&(i, j), "A -- B")?;
        // Test
        assert_eq!(g.to_dot()?, DOT);

        Ok(())
    }

    #[test]
    fn read_dot<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32>,
    {
        // Read DOT file to string
        let dot = Path::new("src/tests/data/14.dot");
        // Load graph using io function
        let g = crate::io::read_dot::<T>(&dot).unwrap().pop().unwrap();
        // Test from DOT string constructor
        assert_eq!(T::read_dot(&dot)?, g);

        Ok(())
    }

    #[test]
    fn write_dot<T>() -> Result<(), Error<u32>>
    where
        T: GraphTrait<Vertex = u32>,
    {
        // Init graph
        let mut g = T::default();
        let i = g.add_vertex_label("A")?;
        let j = g.add_vertex_label("B")?;
        g.add_edge_label(&(i, j), "A -- B")?;
        // Get temporary file path
        let path = NamedTempFile::new().unwrap().into_temp_path();
        // Write to DOT file
        g.write_dot(&path)?;
        // Test
        assert_eq!(g, T::read_dot(&path)?);

        Ok(())
    }

    #[instantiate_tests(<UndirectedAdjacencyListGraph<u32>>)]
    mod undirected_adjacency_list_graph {}
}
