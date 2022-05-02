use std::{collections::BTreeSet, iter::repeat};

use bimap::BiBTreeMap;
use ndarray::{s, Array2, Axis};
use sprs::TriMat;

use crate::{
    graphs::attributes::AttributesMap,
    traits::{Convert, Directed, PartiallyDirected, Storage, Undirected, WithAttributes},
    types::{
        directions, DenseAdjacencyMatrix, DenseMarkerMatrix, EdgeIterator, ExactSizeIter, Marker,
        SparseAdjacencyMatrix, SparseMarkerMatrix, Vertex, VertexIterator,
    },
};

/// Partially-directed graph based on dense adjacency matrix storage layout.
#[derive(Clone, Debug, Default)]
pub struct PartiallyDirectedDenseAdjacencyMatrix<V, A = AttributesMap<V, (), (), ()>>
where
    V: Vertex,
    A: WithAttributes<V>,
{
    _attributes: A,
    _data: DenseMarkerMatrix,
    _idxs: BiBTreeMap<V, usize>,
    _size: usize,
}

impl<V, A> PartialEq for PartiallyDirectedDenseAdjacencyMatrix<V, A>
where
    V: Vertex,
    A: WithAttributes<V>,
{
    fn eq(&self, other: &Self) -> bool {
        self._data.eq(&other._data)
    }
}

impl<V, A> Eq for PartiallyDirectedDenseAdjacencyMatrix<V, A>
where
    V: Vertex,
    A: WithAttributes<V>,
{
}

impl<V, A> Storage for PartiallyDirectedDenseAdjacencyMatrix<V, A>
where
    V: Vertex,
    A: WithAttributes<V>,
{
    type Vertex = V;

    type Direction = directions::PartiallyDirected;

    fn new<I, J>(v_iter: I, e_iter: J) -> Self
    where
        I: IntoIterator<Item = Self::Vertex>,
        J: IntoIterator<Item = (Self::Vertex, Self::Vertex)>,
    {
        // Initialize the data storage using the vertex set.
        let idxs: BTreeSet<_> = FromIterator::from_iter(v_iter);
        let idxs: BiBTreeMap<_, _> = idxs.into_iter().enumerate().map(|(i, x)| (x, i)).collect();
        let data = DenseMarkerMatrix::from_elem((idxs.len(), idxs.len()), Marker::None);

        let mut g = Self {
            _data: data,
            _idxs: idxs,
            ..Default::default()
        };

        // Fill the data storage using the edge set.
        for (x, y) in e_iter {
            g.add_vertex(x.clone());
            g.add_vertex(y.clone());
            g.add_edge(&x, &y);
        }

        g
    }

    fn null() -> Self {
        Default::default()
    }

    fn empty<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self::Vertex>,
    {
        // Initialize the data storage using the vertex set.
        let idxs: BTreeSet<_> = FromIterator::from_iter(iter);
        let idxs: BiBTreeMap<_, _> = idxs.into_iter().enumerate().map(|(i, x)| (x, i)).collect();
        let data = DenseMarkerMatrix::from_elem((idxs.len(), idxs.len()), Marker::None);

        Self {
            _data: data,
            _idxs: idxs,
            ..Default::default()
        }
    }

    fn complete<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self::Vertex>,
    {
        let idxs: BTreeSet<_> = FromIterator::from_iter(iter);
        let idxs: BiBTreeMap<_, _> = idxs.into_iter().enumerate().map(|(i, x)| (x, i)).collect();
        let data = DenseMarkerMatrix::from_elem((idxs.len(), idxs.len()), Marker::TailTail);
        // Compute the final size.
        let size = (data.shape()[0] * (data.shape()[0] + 1)) / 2;

        Self {
            _data: data,
            _idxs: idxs,
            _size: size,
            ..Default::default()
        }
    }

    fn clear(&mut self) {
        self._data = DenseMarkerMatrix::default((0, 0));
        self._idxs.clear();
        self._size = 0;
    }

    fn vertices_iter<'a>(&'a self) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a> {
        Box::new(self._idxs.left_values())
    }

    fn edges_iter<'a>(&'a self) -> Box<dyn EdgeIterator<'a, Self::Vertex> + 'a> {
        Box::new(ExactSizeIter::new(
            self._data.indexed_iter().filter_map(|((x, y), m)| {
                match m {
                    Marker::None => None,
                    Marker::TailTail => {
                        // Return only first appearance of the edge.
                        if x > y {
                            return None;
                        }
                        // Map matrix index to vertex.
                        let (x, y) = (
                            self._idxs.get_by_right(&x).unwrap(),
                            self._idxs.get_by_right(&y).unwrap(),
                        );

                        Some((x, y))
                    }
                    Marker::TailHead => {
                        // Map matrix index to vertex.
                        let (x, y) = (
                            self._idxs.get_by_right(&x).unwrap(),
                            self._idxs.get_by_right(&y).unwrap(),
                        );

                        Some((x, y))
                    }
                    // Invalid markers have already been filtered out.
                    _ => unreachable!(),
                }
            }),
            self._size,
        ))
    }

    fn adjacents_iter<'a>(&'a self, x: &'a Self::Vertex) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a> {
        // Map vertex to matrix index.
        let x = *self._idxs.get_by_left(x).unwrap();

        Box::new(
            repeat(x)
                .zip(0..self._data.shape()[0])
                .filter_map(|(x, y)| match self._data[[x, y]] {
                    Marker::None => None,
                    _ => Some(self._idxs.get_by_right(&y).unwrap()),
                }),
        )
    }

    fn order(&self) -> usize {
        self._idxs.len()
    }

    fn size(&self) -> usize {
        self._size
    }

    fn has_vertex(&self, x: &Self::Vertex) -> bool {
        self._idxs.contains_left(x)
    }

    fn add_vertex(&mut self, x: Self::Vertex) -> bool {
        // Try to add the vertex from the map.
        if !self._idxs.contains_left(&x) {
            // Find the upper bound index.
            let i = match self._idxs.iter().find(|&(y, _)| &x < y) {
                None => {
                    // If no upper bound index is found, define new one.
                    let i = self._idxs.len();
                    // Insert the vertex directly with the defined one.
                    self._idxs.insert(x, i);

                    i
                }
                Some((_, &i)) => {
                    // Shift the upper indices to the right.
                    self._idxs = self
                        ._idxs
                        .clone()
                        .into_iter()
                        .map(|(y, j)| match j < i {
                            false => (y, j + 1),
                            true => (y, j),
                        })
                        .collect();
                    // Insert the vertex with the found index.
                    self._idxs.insert(x, i);

                    i
                }
            };
            // Compute the next valid index.
            let j = i + 1;
            // Allocate memory for new data matrix.
            let mut data = DenseMarkerMatrix::from_elem((self._idxs.len(), self._idxs.len()), Marker::None);

            // Copy the data from the previous data matrix.
            data.slice_mut(s![..i, ..i]).assign(&self._data.slice(s![..i, ..i])); // Top-left     minor of the matrix.
            data.slice_mut(s![..i, j..]).assign(&self._data.slice(s![..i, i..])); // Top-right    minor of the matrix.
            data.slice_mut(s![j.., ..i]).assign(&self._data.slice(s![i.., ..i])); // Bottom-left  minor of the matrix.
            data.slice_mut(s![j.., j..]).assign(&self._data.slice(s![i.., i..])); // Bottom-right minor of the matrix.

            // Replace previous data matrix.
            self._data = data;

            return true;
        }

        false
    }

    fn del_vertex(&mut self, x: &Self::Vertex) -> bool {
        // Try to remove the vertex from the map.
        if let Some((_, i)) = self._idxs.remove_by_left(x) {
            // Update the vertex to index map.
            self._idxs = self
                ._idxs
                .clone()
                .into_iter()
                .map(|(y, j)| match j < i {
                    false => (y, j - 1),
                    true => (y, j),
                })
                .collect();
            // Remove the vertex from the data matrix.
            self._data.remove_index(Axis(0), i);
            self._data.remove_index(Axis(1), i);
            // Compute the final size of the graph.
            self._size = self._data.mapv(|x| !matches!(x, Marker::None) as usize).sum();

            return true;
        }

        false
    }

    fn has_edge(&self, x: &Self::Vertex, y: &Self::Vertex) -> bool {
        // Map vertex to matrix index.
        let (&x, &y) = (self._idxs.get_by_left(x).unwrap(), self._idxs.get_by_left(y).unwrap());

        !matches!(self._data[[x, y]], Marker::None)
    }

    fn add_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> bool {
        // Map vertex to matrix index.
        let (&x, &y) = (self._idxs.get_by_left(x).unwrap(), self._idxs.get_by_left(y).unwrap());

        match self._data[[x, y]] {
            Marker::None => {
                // Insert the default edge symmetrically.
                self._data[[y, x]] = Marker::TailTail;
                self._data[[x, y]] = Marker::TailTail;

                self._size += 1;

                true
            }
            _ => false,
        }
    }

    fn del_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> bool {
        // Map vertex to matrix index.
        let (&x, &y) = (self._idxs.get_by_left(x).unwrap(), self._idxs.get_by_left(y).unwrap());

        match self._data[[x, y]] {
            Marker::None => false,
            Marker::TailTail => {
                // Delete the edge symmetrically.
                self._data[[y, x]] = Marker::None;
                self._data[[x, y]] = Marker::None;

                self._size -= 1;

                true
            }
            Marker::TailHead => {
                // Delete the edge asymmetrically.
                self._data[[x, y]] = Marker::None;

                self._size -= 1;

                true
            }
            // Invalid markers have already been filtered out.
            _ => unreachable!(),
        }
    }
}

impl<V, A> Undirected for PartiallyDirectedDenseAdjacencyMatrix<V, A>
where
    V: Vertex,
    A: WithAttributes<V>,
{
    fn neighbors_iter<'a>(&'a self, x: &'a Self::Vertex) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a> {
        // Map vertex to matrix index.
        let x = *self._idxs.get_by_left(x).unwrap();

        Box::new(repeat(x).zip(0..self._data.shape()[0]).filter_map(|(x, y)| {
            // If i --- j then j is a neighbor of i.
            match self._data[[x, y]] {
                Marker::TailTail => Some(self._idxs.get_by_right(&y).unwrap()),
                _ => None,
            }
        }))
    }

    fn add_undirected_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> bool {
        self.add_edge(x, y)
    }
}

impl<V, A> Directed for PartiallyDirectedDenseAdjacencyMatrix<V, A>
where
    V: Vertex,
    A: WithAttributes<V>,
{
    fn parents_iter<'a>(&'a self, x: &'a Self::Vertex) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a> {
        // Map vertex to matrix index.
        let i = *self._idxs.get_by_left(x).unwrap();

        Box::new(repeat(i).zip(0..self._data.shape()[0]).filter_map(|(i, j)| {
            // If j --> i then j is a parent of i.
            match self._data[[j, i]] {
                Marker::TailHead => Some(self._idxs.get_by_right(&j).unwrap()),
                _ => None,
            }
        }))
    }

    fn children_iter<'a>(&'a self, x: &'a Self::Vertex) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a> {
        // Map vertex to matrix index.
        let x = *self._idxs.get_by_left(x).unwrap();

        Box::new(repeat(x).zip(0..self._data.shape()[0]).filter_map(|(x, y)| {
            // If i --> j then j is a child of i.
            match self._data[[x, y]] {
                Marker::TailHead => Some(self._idxs.get_by_right(&y).unwrap()),
                _ => None,
            }
        }))
    }

    fn add_directed_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> bool {
        // Map vertex to matrix index.
        let (&x, &y) = (self._idxs.get_by_left(x).unwrap(), self._idxs.get_by_left(y).unwrap());

        match self._data[[x, y]] {
            Marker::None => {
                // Insert the edge asymmetrically.
                self._data[[x, y]] = Marker::TailHead;

                self._size += 1;

                true
            }
            _ => false,
        }
    }
}

impl<V, A> Convert for PartiallyDirectedDenseAdjacencyMatrix<V, A>
where
    V: Vertex,
    A: WithAttributes<V>,
{
    fn dense_adjacency_matrix(&self) -> DenseAdjacencyMatrix {
        self._data.mapv(|x| match x {
            Marker::None => false,
            Marker::TailTail | Marker::TailHead => true,
            // Invalid markers have already been filtered out.
            _ => unreachable!(),
        })
    }

    fn sparse_adjacency_matrix(&self) -> SparseAdjacencyMatrix {
        self._data
            .indexed_iter()
            .map(|((i, j), x)| {
                let x = match x {
                    Marker::None => false,
                    Marker::TailTail | Marker::TailHead => true,
                    // Invalid markers have already been filtered out.
                    _ => unreachable!(),
                };
                (x, i, j)
            })
            .fold(
                {
                    let (n, m) = (self.order(), self.size());
                    SparseAdjacencyMatrix::with_capacity((n, n), m)
                },
                |mut acc, (x, i, j)| {
                    acc.add_triplet(i, j, x);

                    acc
                },
            )
    }

    fn dense_incidence_matrix(&self) -> Array2<i8> {
        // FIXME:
        todo!()
    }

    fn sparse_incidence_matrix(&self) -> TriMat<i8> {
        // FIXME:
        todo!()
    }

    fn dense_marker_matrix(&self) -> DenseMarkerMatrix {
        self._data.clone()
    }

    fn sparse_marker_matrix(&self) -> SparseMarkerMatrix {
        self._data.indexed_iter().fold(
            {
                let (n, m) = (self.order(), self.size());
                SparseMarkerMatrix::with_capacity((n, n), m)
            },
            |mut acc, ((i, j), m)| {
                acc.add_triplet(i, j, *m);

                acc
            },
        )
    }
}

impl<V, A> PartiallyDirected for PartiallyDirectedDenseAdjacencyMatrix<V, A>
where
    V: Vertex,
    A: WithAttributes<V>,
{
    fn from_dense_marker_matrix(data: DenseMarkerMatrix, variables: Vec<V>) -> Self {
        // Check if the marker pair is valid.
        assert!(
            data.iter()
                .all(|m| matches!(m, Marker::None | Marker::TailTail | Marker::TailHead)),
            "Invalid marker pair. Partially-directed graphs can accept only TailTail and TailHead."
        );
        // Add variables to the index.
        let idxs = variables.into_iter().enumerate().map(|(i, x)| (x, i)).collect();
        // Compute the final size.
        let size = data
            .indexed_iter()
            .map(|((i, j), &m)| {
                match m {
                    Marker::None => 0,
                    Marker::TailTail => {
                        // Return only first appearance of the edge.
                        if i > j {
                            return 0;
                        }

                        1
                    }
                    Marker::TailHead => 1,
                    // Invalid markers have already been filtered out.
                    _ => unreachable!(),
                }
            })
            .sum();

        Self {
            _data: data,
            _idxs: idxs,
            _size: size,
            ..Default::default()
        }
    }

    fn new_with_marker<I, J>(v_iter: I, e_iter: J) -> Self
    where
        I: IntoIterator<Item = Self::Vertex>,
        J: IntoIterator<Item = (Self::Vertex, Self::Vertex, Marker)>,
    {
        // Initialize the data storage using the vertex set.
        let idxs: BTreeSet<_> = FromIterator::from_iter(v_iter);
        let idxs: BiBTreeMap<_, _> = idxs.into_iter().enumerate().map(|(i, x)| (x, i)).collect();
        let data = DenseMarkerMatrix::from_elem((idxs.len(), idxs.len()), Marker::None);

        let mut g = Self {
            _data: data,
            _idxs: idxs,
            ..Default::default()
        };

        // Fill the data storage using the edge set.
        for (x, y, m) in e_iter {
            g.add_vertex(x.clone());
            g.add_vertex(y.clone());
            g.set_marker(&x, &y, m);
        }

        g
    }

    fn edges_with_marker_iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = (&'a Self::Vertex, &'a Self::Vertex, &'a Marker)> + 'a> {
        Box::new(ExactSizeIter::new(
            self._data.indexed_iter().filter_map(|((x, y), m)| {
                match m {
                    Marker::None => None,
                    Marker::TailTail => {
                        // Return only first appearance of the edge.
                        if x > y {
                            return None;
                        }
                        // Map matrix index to vertex.
                        let (x, y) = (
                            self._idxs.get_by_right(&x).unwrap(),
                            self._idxs.get_by_right(&y).unwrap(),
                        );

                        Some((x, y, m))
                    }
                    Marker::TailHead => {
                        // Map matrix index to vertex.
                        let (x, y) = (
                            self._idxs.get_by_right(&x).unwrap(),
                            self._idxs.get_by_right(&y).unwrap(),
                        );

                        Some((x, y, m))
                    }
                    // Invalid markers have already been filtered out.
                    _ => unreachable!(),
                }
            }),
            self._size,
        ))
    }

    fn has_marker(&self, x: &Self::Vertex, y: &Self::Vertex, m: Marker) -> bool {
        // Map vertex to matrix index.
        let (&x, &y) = (self._idxs.get_by_left(x).unwrap(), self._idxs.get_by_left(y).unwrap());

        self._data[[x, y]].eq(&m)
    }

    fn get_marker(&self, x: &Self::Vertex, y: &Self::Vertex) -> Option<Marker> {
        // Map vertex to matrix index.
        let (&x, &y) = (self._idxs.get_by_left(x).unwrap(), self._idxs.get_by_left(y).unwrap());

        match self._data[[x, y]] {
            Marker::None => None,
            m => Some(m),
        }
    }

    fn set_marker(&mut self, x: &Self::Vertex, y: &Self::Vertex, m: Marker) -> bool {
        // Check if the marker pair is valid.
        assert!(
            matches!(m, Marker::TailTail | Marker::TailHead),
            "Invalid marker pair. Partially-directed graphs can accept only TailTail and TailHead."
        );
        // Map vertex to matrix index.
        let (&x, &y) = (self._idxs.get_by_left(x).unwrap(), self._idxs.get_by_left(y).unwrap());
        // Get current marker pair.
        let n = self._data[[x, y]];
        // If the marker pair is already set ...
        if m.eq(&n) {
            // ... do not modify the matrix.
            return false;
        }
        // Increase the size only if None.
        if matches!(n, Marker::None) {
            self._size += 1;
        }
        // Set the marker pair.
        match (m, n) {
            // If the marker pair is symmetric ...
            (Marker::TailTail, _) => {
                // ... set the edge symmetrically.
                self._data[[y, x]] = m;
                self._data[[x, y]] = m;

                true
            }
            // Otherwise, the marker pair is asymmetric.
            (Marker::TailHead, Marker::TailTail) => {
                // ... set the edge asymmetrically ...
                self._data[[x, y]] = m;
                // ... and unset the symmetric edge.
                self._data[[y, x]] = Marker::None;

                true
            }
            // Otherwise, the marker pair is asymmetric.
            (Marker::TailHead, Marker::None | Marker::TailHead) => {
                // ... set the edge asymmetrically ...
                self._data[[x, y]] = m;

                true
            }
            // Invalid markers have already been filtered out.
            _ => unreachable!(),
        }
    }
}
