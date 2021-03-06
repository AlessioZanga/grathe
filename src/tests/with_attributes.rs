#[cfg(test)]
#[generic_tests::define]
mod tests {
    use crate::graphs::attributes::AttributesMap;
    use crate::graphs::UndirectedAdjacencyListGraph;
    use crate::traits::{From, WithAttributes};
    use all_asserts::*;

    #[test]
    fn has_graph_attrs<T>()
    where
        T: Default
            + From<Vertex = i32>
            + WithAttributes<i32, GraphAttributes = (bool,), VertexAttributes = (usize,), EdgeAttributes = (f64,)>,
    {
        let mut g: T = Default::default();
        assert_false!(g.has_graph_attrs());

        g.set_graph_attrs((false,));
        assert_true!(g.has_graph_attrs());
    }

    #[test]
    fn get_graph_attrs<T>()
    where
        T: Default
            + From<Vertex = i32>
            + WithAttributes<i32, GraphAttributes = (bool,), VertexAttributes = (usize,), EdgeAttributes = (f64,)>,
    {
        let mut g: T = Default::default();
        assert_true!(g.get_graph_attrs().is_err());

        g.set_graph_attrs((false,));
        assert_eq!(g.get_graph_attrs(), Ok(&(false,)));
    }

    #[test]
    fn get_mut_graph_attrs<T>()
    where
        T: Default
            + From<Vertex = i32>
            + WithAttributes<i32, GraphAttributes = (bool,), VertexAttributes = (usize,), EdgeAttributes = (f64,)>,
    {
        let mut g: T = Default::default();
        assert_true!(g.get_mut_graph_attrs().is_err());

        g.set_graph_attrs((false,));
        assert_eq!(g.get_mut_graph_attrs(), Ok(&mut (false,)));

        g.get_mut_graph_attrs().unwrap().0 = true;
        assert_eq!(g.get_mut_graph_attrs(), Ok(&mut (true,)));
    }

    #[test]
    fn set_graph_attrs<T>()
    where
        T: Default
            + From<Vertex = i32>
            + WithAttributes<i32, GraphAttributes = (bool,), VertexAttributes = (usize,), EdgeAttributes = (f64,)>,
    {
        let mut g: T = Default::default();
        assert_false!(g.has_graph_attrs());

        g.set_graph_attrs((false,));
        assert_true!(g.has_graph_attrs());
        assert_eq!(g.get_graph_attrs(), Ok(&(false,)));
    }

    #[test]
    fn unset_graph_attrs<T>()
    where
        T: Default
            + From<Vertex = i32>
            + WithAttributes<i32, GraphAttributes = (bool,), VertexAttributes = (usize,), EdgeAttributes = (f64,)>,
    {
        let mut g: T = Default::default();
        assert_true!(g.unset_graph_attrs().is_err());
        assert_false!(g.has_graph_attrs());

        g.set_graph_attrs((false,));
        assert_eq!(g.unset_graph_attrs(), Ok((false,)));
        assert_false!(g.has_graph_attrs());
    }

    #[test]
    fn has_vertex_attrs<T>()
    where
        T: Default
            + From<Vertex = i32>
            + WithAttributes<i32, GraphAttributes = (bool,), VertexAttributes = (usize,), EdgeAttributes = (f64,)>,
    {
        let mut g = T::from_vertices([0]);
        assert_false!(g.has_vertex_attrs(&0));

        g.set_vertex_attrs(&0, (42,));
        assert_true!(g.has_vertex_attrs(&0));
    }

    #[test]
    #[should_panic]
    fn has_vertex_attrs_should_panic<T>()
    where
        T: Default
            + From<Vertex = i32>
            + WithAttributes<i32, GraphAttributes = (bool,), VertexAttributes = (usize,), EdgeAttributes = (f64,)>,
    {
        let g: T = Default::default();
        g.has_vertex_attrs(&0);
    }

    #[test]
    fn get_vertex_attrs<T>()
    where
        T: Default
            + From<Vertex = i32>
            + WithAttributes<i32, GraphAttributes = (bool,), VertexAttributes = (usize,), EdgeAttributes = (f64,)>,
    {
        let mut g = T::from_vertices([0]);
        assert_true!(g.get_vertex_attrs(&0).is_err());

        g.set_vertex_attrs(&0, (42,));
        assert_eq!(g.get_vertex_attrs(&0), Ok(&(42,)));
    }

    #[test]
    #[should_panic]
    fn get_vertex_attrs_should_panic<T>()
    where
        T: Default
            + From<Vertex = i32>
            + WithAttributes<i32, GraphAttributes = (bool,), VertexAttributes = (usize,), EdgeAttributes = (f64,)>,
    {
        let g: T = Default::default();
        g.get_vertex_attrs(&0).ok();
    }

    #[test]
    fn get_mut_vertex_attrs<T>()
    where
        T: Default
            + From<Vertex = i32>
            + WithAttributes<i32, GraphAttributes = (bool,), VertexAttributes = (usize,), EdgeAttributes = (f64,)>,
    {
        let mut g = T::from_vertices([0]);
        assert_true!(g.get_mut_vertex_attrs(&0).is_err());

        g.set_vertex_attrs(&0, (42,));
        assert_eq!(g.get_mut_vertex_attrs(&0), Ok(&mut (42,)));

        g.get_mut_vertex_attrs(&0).unwrap().0 = 42;
        assert_eq!(g.get_mut_vertex_attrs(&0), Ok(&mut (42,)));
    }

    #[test]
    #[should_panic]
    fn get_mut_vertex_attrs_should_panic<T>()
    where
        T: Default
            + From<Vertex = i32>
            + WithAttributes<i32, GraphAttributes = (bool,), VertexAttributes = (usize,), EdgeAttributes = (f64,)>,
    {
        let mut g: T = Default::default();
        g.get_mut_vertex_attrs(&0).ok();
    }

    #[test]
    fn set_vertex_attrs<T>()
    where
        T: Default
            + From<Vertex = i32>
            + WithAttributes<i32, GraphAttributes = (bool,), VertexAttributes = (usize,), EdgeAttributes = (f64,)>,
    {
        let mut g = T::from_vertices([0]);
        assert_false!(g.has_vertex_attrs(&0));

        g.set_vertex_attrs(&0, (42,));
        assert_true!(g.has_vertex_attrs(&0));
        assert_eq!(g.get_vertex_attrs(&0), Ok(&(42,)));
    }

    #[test]
    #[should_panic]
    fn set_vertex_attrs_should_panic<T>()
    where
        T: Default
            + From<Vertex = i32>
            + WithAttributes<i32, GraphAttributes = (bool,), VertexAttributes = (usize,), EdgeAttributes = (f64,)>,
    {
        let mut g: T = Default::default();
        g.set_vertex_attrs(&0, (42,));
    }

    #[test]
    fn unset_vertex_attrs<T>()
    where
        T: Default
            + From<Vertex = i32>
            + WithAttributes<i32, GraphAttributes = (bool,), VertexAttributes = (usize,), EdgeAttributes = (f64,)>,
    {
        let mut g = T::from_vertices([0]);
        assert_true!(g.unset_vertex_attrs(&0).is_err());
        assert_false!(g.has_vertex_attrs(&0));

        g.set_vertex_attrs(&0, (42,));
        assert_eq!(g.unset_vertex_attrs(&0), Ok((42,)));
        assert_false!(g.has_vertex_attrs(&0));
    }

    #[test]
    #[should_panic]
    fn unset_vertex_attrs_should_panic<T>()
    where
        T: Default
            + From<Vertex = i32>
            + WithAttributes<i32, GraphAttributes = (bool,), VertexAttributes = (usize,), EdgeAttributes = (f64,)>,
    {
        let mut g: T = Default::default();
        g.unset_vertex_attrs(&0).ok();
    }

    #[test]
    fn has_edge_attrs<T>()
    where
        T: Default
            + From<Vertex = i32>
            + WithAttributes<i32, GraphAttributes = (bool,), VertexAttributes = (usize,), EdgeAttributes = (f64,)>,
    {
        let mut g = T::from_edges([(0, 1)]);
        assert_false!(g.has_edge_attrs(&0, &1));

        g.set_edge_attrs(&0, &1, (42.0,));
        assert_true!(g.has_edge_attrs(&0, &1));
    }

    #[test]
    #[should_panic]
    fn has_edge_attrs_should_panic<T>()
    where
        T: Default
            + From<Vertex = i32>
            + WithAttributes<i32, GraphAttributes = (bool,), VertexAttributes = (usize,), EdgeAttributes = (f64,)>,
    {
        let g: T = Default::default();
        g.has_edge_attrs(&0, &1);
    }

    #[test]
    fn get_edge_attrs<T>()
    where
        T: Default
            + From<Vertex = i32>
            + WithAttributes<i32, GraphAttributes = (bool,), VertexAttributes = (usize,), EdgeAttributes = (f64,)>,
    {
        let mut g = T::from_edges([(0, 1)]);
        assert_true!(g.get_edge_attrs(&0, &1).is_err());

        g.set_edge_attrs(&0, &1, (42.0,));
        assert_eq!(g.get_edge_attrs(&0, &1), Ok(&(42.0,)));
    }

    #[test]
    #[should_panic]
    fn get_edge_attrs_should_panic<T>()
    where
        T: Default
            + From<Vertex = i32>
            + WithAttributes<i32, GraphAttributes = (bool,), VertexAttributes = (usize,), EdgeAttributes = (f64,)>,
    {
        let g: T = Default::default();
        g.get_edge_attrs(&0, &1).ok();
    }

    #[test]
    fn get_mut_edge_attrs<T>()
    where
        T: Default
            + From<Vertex = i32>
            + WithAttributes<i32, GraphAttributes = (bool,), VertexAttributes = (usize,), EdgeAttributes = (f64,)>,
    {
        let mut g = T::from_edges([(0, 1)]);
        assert_true!(g.get_mut_edge_attrs(&0, &1).is_err());

        g.set_edge_attrs(&0, &1, (42.0,));
        assert_eq!(g.get_mut_edge_attrs(&0, &1), Ok(&mut (42.0,)));

        g.get_mut_edge_attrs(&0, &1).unwrap().0 = 42.0;
        assert_eq!(g.get_mut_edge_attrs(&0, &1), Ok(&mut (42.0,)));
    }

    #[test]
    #[should_panic]
    fn get_mut_edge_attrs_should_panic<T>()
    where
        T: Default
            + From<Vertex = i32>
            + WithAttributes<i32, GraphAttributes = (bool,), VertexAttributes = (usize,), EdgeAttributes = (f64,)>,
    {
        let mut g: T = Default::default();
        g.get_mut_edge_attrs(&0, &1).ok();
    }

    #[test]
    fn set_edge_attrs<T>()
    where
        T: Default
            + From<Vertex = i32>
            + WithAttributes<i32, GraphAttributes = (bool,), VertexAttributes = (usize,), EdgeAttributes = (f64,)>,
    {
        let mut g = T::from_edges([(0, 1)]);
        assert_false!(g.has_edge_attrs(&0, &1));

        g.set_edge_attrs(&0, &1, (42.0,));
        assert_true!(g.has_edge_attrs(&0, &1));
        assert_eq!(g.get_edge_attrs(&0, &1), Ok(&(42.0,)));
    }

    #[test]
    #[should_panic]
    fn set_edge_attrs_should_panic<T>()
    where
        T: Default
            + From<Vertex = i32>
            + WithAttributes<i32, GraphAttributes = (bool,), VertexAttributes = (usize,), EdgeAttributes = (f64,)>,
    {
        let mut g: T = Default::default();
        g.set_edge_attrs(&0, &1, (42.0,));
    }

    #[test]
    fn unset_edge_attrs<T>()
    where
        T: Default
            + From<Vertex = i32>
            + WithAttributes<i32, GraphAttributes = (bool,), VertexAttributes = (usize,), EdgeAttributes = (f64,)>,
    {
        let mut g = T::from_edges([(0, 1)]);
        assert_true!(g.unset_edge_attrs(&0, &1).is_err());
        assert_false!(g.has_edge_attrs(&0, &1));

        g.set_edge_attrs(&0, &1, (42.0,));
        assert_eq!(g.unset_edge_attrs(&0, &1), Ok((42.0,)));
        assert_false!(g.has_edge_attrs(&0, &1));
    }

    #[test]
    #[should_panic]
    fn unset_edge_attrs_should_panic<T>()
    where
        T: Default
            + From<Vertex = i32>
            + WithAttributes<i32, GraphAttributes = (bool,), VertexAttributes = (usize,), EdgeAttributes = (f64,)>,
    {
        let mut g: T = Default::default();
        g.unset_edge_attrs(&0, &1).ok();
    }

    #[instantiate_tests(<UndirectedAdjacencyListGraph<i32, AttributesMap<i32, (bool, ), (usize, ), (f64, )>>>)]
    mod attribute_map {}
}
