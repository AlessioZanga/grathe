#[cfg(test)]
mod tests {
    macro_rules! generic_tests {
        ($G: ident) => {
            paste::item! {
                use all_asserts::*;
                use grathe::graphs::attributes::AttributesMap;
                use grathe::traits::{From, WithAttributes};

                #[test]
                #[ignore]
                // FIXME:
                fn has_graph_attrs()
                {
                    let mut g: $G<i32, AttributesMap<i32, (bool, ), (usize, ), (f64, )>> =Default::default();
                    assert_false!(g.has_graph_attrs());

                    g.set_graph_attrs((false,));
                    assert_true!(g.has_graph_attrs());
                }

                #[test]
                #[ignore]
                // FIXME:
                fn get_graph_attrs()
                {
                    let mut g: $G<i32, AttributesMap<i32, (bool, ), (usize, ), (f64, )>> =Default::default();
                    assert_true!(g.get_graph_attrs().is_err());

                    g.set_graph_attrs((false,));
                    assert_eq!(g.get_graph_attrs(), Ok(&(false,)));
                }

                #[test]
                #[ignore]
                // FIXME:
                fn get_mut_graph_attrs()
                {
                    let mut g: $G<i32, AttributesMap<i32, (bool, ), (usize, ), (f64, )>> =Default::default();
                    assert_true!(g.get_mut_graph_attrs().is_err());

                    g.set_graph_attrs((false,));
                    assert_eq!(g.get_mut_graph_attrs(), Ok(&mut (false,)));

                    g.get_mut_graph_attrs().unwrap().0 = true;
                    assert_eq!(g.get_mut_graph_attrs(), Ok(&mut (true,)));
                }

                #[test]
                #[ignore]
                // FIXME:
                fn set_graph_attrs()
                {
                    let mut g: $G<i32, AttributesMap<i32, (bool, ), (usize, ), (f64, )>> =Default::default();
                    assert_false!(g.has_graph_attrs());

                    g.set_graph_attrs((false,));
                    assert_true!(g.has_graph_attrs());
                    assert_eq!(g.get_graph_attrs(), Ok(&(false,)));
                }

                #[test]
                #[ignore]
                // FIXME:
                fn unset_graph_attrs()
                {
                    let mut g: $G<i32, AttributesMap<i32, (bool, ), (usize, ), (f64, )>> =Default::default();
                    assert_true!(g.unset_graph_attrs().is_err());
                    assert_false!(g.has_graph_attrs());

                    g.set_graph_attrs((false,));
                    assert_eq!(g.unset_graph_attrs(), Ok((false,)));
                    assert_false!(g.has_graph_attrs());
                }

                #[test]
                #[ignore]
                // FIXME:
                fn has_vertex_attrs()
                {
                    let mut g = $G::<i32, AttributesMap<i32, (bool, ), (usize, ), (f64, )>>::from_vertices([0]);
                    assert_false!(g.has_vertex_attrs(&0));

                    g.set_vertex_attrs(&0, (42,));
                    assert_true!(g.has_vertex_attrs(&0));
                }

                #[test]
                #[should_panic]
                fn has_vertex_attrs_should_panic()
                {
                    let g: $G<i32, AttributesMap<i32, (bool, ), (usize, ), (f64, )>> =Default::default();
                    g.has_vertex_attrs(&0);
                }

                #[test]
                #[ignore]
                // FIXME:
                fn get_vertex_attrs()
                {
                    let mut g = $G::<i32, AttributesMap<i32, (bool, ), (usize, ), (f64, )>>::from_vertices([0]);
                    assert_true!(g.get_vertex_attrs(&0).is_err());

                    g.set_vertex_attrs(&0, (42,));
                    assert_eq!(g.get_vertex_attrs(&0), Ok(&(42,)));
                }

                #[test]
                #[should_panic]
                fn get_vertex_attrs_should_panic()
                {
                    let g: $G<i32, AttributesMap<i32, (bool, ), (usize, ), (f64, )>> =Default::default();
                    g.get_vertex_attrs(&0).ok();
                }

                #[test]
                #[ignore]
                // FIXME:
                fn get_mut_vertex_attrs()
                {
                    let mut g = $G::<i32, AttributesMap<i32, (bool, ), (usize, ), (f64, )>>::from_vertices([0]);
                    assert_true!(g.get_mut_vertex_attrs(&0).is_err());

                    g.set_vertex_attrs(&0, (42,));
                    assert_eq!(g.get_mut_vertex_attrs(&0), Ok(&mut (42,)));

                    g.get_mut_vertex_attrs(&0).unwrap().0 = 42;
                    assert_eq!(g.get_mut_vertex_attrs(&0), Ok(&mut (42,)));
                }

                #[test]
                #[should_panic]
                fn get_mut_vertex_attrs_should_panic()
                {
                    let mut g: $G<i32, AttributesMap<i32, (bool, ), (usize, ), (f64, )>> =Default::default();
                    g.get_mut_vertex_attrs(&0).ok();
                }

                #[test]
                #[ignore]
                // FIXME:
                fn set_vertex_attrs()
                {
                    let mut g = $G::<i32, AttributesMap<i32, (bool, ), (usize, ), (f64, )>>::from_vertices([0]);
                    assert_false!(g.has_vertex_attrs(&0));

                    g.set_vertex_attrs(&0, (42,));
                    assert_true!(g.has_vertex_attrs(&0));
                    assert_eq!(g.get_vertex_attrs(&0), Ok(&(42,)));
                }

                #[test]
                #[should_panic]
                fn set_vertex_attrs_should_panic()
                {
                    let mut g: $G<i32, AttributesMap<i32, (bool, ), (usize, ), (f64, )>> =Default::default();
                    g.set_vertex_attrs(&0, (42,));
                }

                #[test]
                #[ignore]
                // FIXME:
                fn unset_vertex_attrs()
                {
                    let mut g = $G::<i32, AttributesMap<i32, (bool, ), (usize, ), (f64, )>>::from_vertices([0]);
                    assert_true!(g.unset_vertex_attrs(&0).is_err());
                    assert_false!(g.has_vertex_attrs(&0));

                    g.set_vertex_attrs(&0, (42,));
                    assert_eq!(g.unset_vertex_attrs(&0), Ok((42,)));
                    assert_false!(g.has_vertex_attrs(&0));
                }

                #[test]
                #[should_panic]
                fn unset_vertex_attrs_should_panic()
                {
                    let mut g: $G<i32, AttributesMap<i32, (bool, ), (usize, ), (f64, )>> =Default::default();
                    g.unset_vertex_attrs(&0).ok();
                }

                #[test]
                #[ignore]
                // FIXME:
                fn has_edge_attrs()
                {
                    let mut g = $G::<i32, AttributesMap<i32, (bool, ), (usize, ), (f64, )>>::from_edges([(0, 1)]);
                    assert_false!(g.has_edge_attrs(&0, &1));

                    g.set_edge_attrs(&0, &1, (42.0,));
                    assert_true!(g.has_edge_attrs(&0, &1));
                }

                #[test]
                #[should_panic]
                fn has_edge_attrs_should_panic()
                {
                    let g: $G<i32, AttributesMap<i32, (bool, ), (usize, ), (f64, )>> =Default::default();
                    g.has_edge_attrs(&0, &1);
                }

                #[test]
                #[ignore]
                // FIXME:
                fn get_edge_attrs()
                {
                    let mut g = $G::<i32, AttributesMap<i32, (bool, ), (usize, ), (f64, )>>::from_edges([(0, 1)]);
                    assert_true!(g.get_edge_attrs(&0, &1).is_err());

                    g.set_edge_attrs(&0, &1, (42.0,));
                    assert_eq!(g.get_edge_attrs(&0, &1), Ok(&(42.0,)));
                }

                #[test]
                #[should_panic]
                fn get_edge_attrs_should_panic()
                {
                    let g: $G<i32, AttributesMap<i32, (bool, ), (usize, ), (f64, )>> =Default::default();
                    g.get_edge_attrs(&0, &1).ok();
                }

                #[test]
                #[ignore]
                // FIXME:
                fn get_mut_edge_attrs()
                {
                    let mut g = $G::<i32, AttributesMap<i32, (bool, ), (usize, ), (f64, )>>::from_edges([(0, 1)]);
                    assert_true!(g.get_mut_edge_attrs(&0, &1).is_err());

                    g.set_edge_attrs(&0, &1, (42.0,));
                    assert_eq!(g.get_mut_edge_attrs(&0, &1), Ok(&mut (42.0,)));

                    g.get_mut_edge_attrs(&0, &1).unwrap().0 = 42.0;
                    assert_eq!(g.get_mut_edge_attrs(&0, &1), Ok(&mut (42.0,)));
                }

                #[test]
                #[should_panic]
                fn get_mut_edge_attrs_should_panic()
                {
                    let mut g: $G<i32, AttributesMap<i32, (bool, ), (usize, ), (f64, )>> =Default::default();
                    g.get_mut_edge_attrs(&0, &1).ok();
                }

                #[test]
                #[ignore]
                // FIXME:
                fn set_edge_attrs()
                {
                    let mut g = $G::<i32, AttributesMap<i32, (bool, ), (usize, ), (f64, )>>::from_edges([(0, 1)]);
                    assert_false!(g.has_edge_attrs(&0, &1));

                    g.set_edge_attrs(&0, &1, (42.0,));
                    assert_true!(g.has_edge_attrs(&0, &1));
                    assert_eq!(g.get_edge_attrs(&0, &1), Ok(&(42.0,)));
                }

                #[test]
                #[should_panic]
                fn set_edge_attrs_should_panic()
                {
                    let mut g: $G<i32, AttributesMap<i32, (bool, ), (usize, ), (f64, )>> =Default::default();
                    g.set_edge_attrs(&0, &1, (42.0,));
                }

                #[test]
                #[ignore]
                // FIXME:
                fn unset_edge_attrs()
                {
                    let mut g = $G::<i32, AttributesMap<i32, (bool, ), (usize, ), (f64, )>>::from_edges([(0, 1)]);
                    assert_true!(g.unset_edge_attrs(&0, &1).is_err());
                    assert_false!(g.has_edge_attrs(&0, &1));

                    g.set_edge_attrs(&0, &1, (42.0,));
                    assert_eq!(g.unset_edge_attrs(&0, &1), Ok((42.0,)));
                    assert_false!(g.has_edge_attrs(&0, &1));
                }

                #[test]
                #[should_panic]
                fn unset_edge_attrs_should_panic()
                {
                    let mut g: $G<i32, AttributesMap<i32, (bool, ), (usize, ), (f64, )>> =Default::default();
                    g.unset_edge_attrs(&0, &1).ok();
                }
            }
        };
    }

    mod attribute_map {
        use grathe::graphs::storages::UndirectedAdjacencyList;
        generic_tests!(UndirectedAdjacencyList);
    }
}
