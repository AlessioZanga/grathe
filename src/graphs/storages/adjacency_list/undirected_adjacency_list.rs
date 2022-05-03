use std::{
    cmp::Ordering,
    collections::{btree_map::Entry, BTreeSet, HashMap},
};

use itertools::Itertools;
use ndarray::prelude::*;
use sprs::TriMat;

use crate::{
    graphs::attributes::AttributesMap,
    prelude::{DFSEdge, DFSEdges, Traversal, BFS},
    traits::{Connectivity, Convert, Operators, Storage, Undirected, WithAttributes},
    types::{
        directions, AdjacencyList, DenseAdjacencyMatrix, DenseMarkMatrix, EdgeIterator, Error, ExactSizeIter,
        Mark as M, SparseAdjacencyMatrix, SparseMarkMatrix, Vertex, VertexIterator,
    },
    E, V,
};

/// Undirected graph based on adjacency list storage layout.
#[derive(Clone, Debug, Default)]
pub struct UndirectedAdjacencyList<V, A = AttributesMap<V, (), (), ()>>
where
    V: Vertex,
    A: WithAttributes<V>,
{
    _attributes: A,
    _data: AdjacencyList<V>,
    _size: usize,
}

impl<V, A> PartialEq for UndirectedAdjacencyList<V, A>
where
    V: Vertex,
    A: WithAttributes<V>,
{
    fn eq(&self, other: &Self) -> bool {
        self._data.eq(&other._data)
    }
}

impl<V, A> Eq for UndirectedAdjacencyList<V, A>
where
    V: Vertex,
    A: WithAttributes<V>,
{
}

crate::traits::impl_partial_ord!(UndirectedAdjacencyList);

crate::traits::impl_capacity!(UndirectedAdjacencyList);

impl<V, A> Operators for UndirectedAdjacencyList<V, A>
where
    V: Vertex,
    A: WithAttributes<V>,
{
    fn complement(&self) -> Self {
        // Copy the vertex set.
        let vertices: BTreeSet<_> = self._data.keys().cloned().collect();
        // Iterate over every permutation of V^2.
        let data: AdjacencyList<_> = vertices
            .iter()
            .map(|x| (x.clone(), vertices.difference(&self._data[x]).cloned().collect()))
            .collect();
        // Compute complement size.
        let size = data.iter().map(|(x, ys)| ys.iter().filter(|y| x <= y).count()).sum();

        Self {
            _data: data,
            _size: size,
            ..Default::default()
        }
    }

    fn union(&self, other: &Self) -> Self {
        // Clone internal storage status.
        let mut data = self._data.clone();
        // For each other entry.
        for (x, ys) in other._data.iter() {
            // Get the correspondent entry on the union (or default it).
            let zs = data.entry(x.clone()).or_default();
            // Perform union of entries.
            *zs = zs.union(ys).cloned().collect();
        }
        // Compute union size.
        let size = data.iter().map(|(x, ys)| ys.iter().filter(|y| x <= y).count()).sum();

        // Return the union graph.
        Self {
            _data: data,
            _size: size,
            ..Default::default()
        }
    }

    fn intersection(&self, other: &Self) -> Self {
        let data: AdjacencyList<_> = self
            ._data
            .iter()
            // For each entry in the current graph.
            .filter_map(|(x, ys)| {
                other._data.get(x).map(|zs| {
                    // If there a common vertex, then intersect its adjacent vertices.
                    (x.clone(), ys.intersection(zs).cloned().collect())
                })
            })
            .collect();
        // Compute intersection size.
        let size = data.iter().map(|(x, ys)| ys.iter().filter(|y| x <= y).count()).sum();

        Self {
            _data: data,
            _size: size,
            ..Default::default()
        }
    }

    fn symmetric_difference(&self, other: &Self) -> Self {
        // Clone internal storage status.
        let mut data = self._data.clone();
        // For each other entry.
        for (x, ys) in other._data.iter() {
            // Get the correspondent entry on the union (or default it).
            let zs = data.entry(x.clone()).or_default();
            // Perform symmetric difference of entries.
            *zs = zs.symmetric_difference(ys).cloned().collect();
        }
        // Compute symmetric difference size.
        let size = data.iter().map(|(x, ys)| ys.iter().filter(|y| x <= y).count()).sum();

        // Return the symmetric difference graph.
        Self {
            _data: data,
            _size: size,
            ..Default::default()
        }
    }

    fn difference(&self, other: &Self) -> Self {
        let data: AdjacencyList<_> = self
            ._data
            .iter()
            // For each entry in the current graph.
            .map(|(x, ys)| match other._data.get(x) {
                // If there a common vertex, then subtract its adjacent vertices.
                Some(zs) => (x.clone(), ys.difference(zs).cloned().collect()),
                // Else return as it is.
                None => (x.clone(), ys.clone()),
            })
            .collect();
        // Compute symmetric difference size.
        let size = data.iter().map(|(x, ys)| ys.iter().filter(|y| x <= y).count()).sum();

        Self {
            _data: data,
            _size: size,
            ..Default::default()
        }
    }
}

crate::traits::impl_operators_extension!(UndirectedAdjacencyList);

impl<V, A> Storage for UndirectedAdjacencyList<V, A>
where
    V: Vertex,
    A: WithAttributes<V>,
{
    type Vertex = V;

    type Direction = directions::Undirected;

    fn new<I, J>(v_iter: I, e_iter: J) -> Self
    where
        I: IntoIterator<Item = Self::Vertex>,
        J: IntoIterator<Item = (Self::Vertex, Self::Vertex)>,
    {
        // Initialize the data storage using the vertex set.
        let mut data: AdjacencyList<Self::Vertex> = v_iter.into_iter().map(|x| (x, Default::default())).collect();
        // Fill the data storage using the edge set.
        let size = e_iter.into_iter().fold(0, |acc, (x, y)| {
            data.entry(x.clone()).or_default().insert(y.clone());
            data.entry(y).or_default().insert(x);

            acc + 1
        });

        Self {
            _data: data,
            _size: size,
            ..Default::default()
        }
    }

    fn null() -> Self {
        Default::default()
    }

    fn empty<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self::Vertex>,
    {
        let data = iter.into_iter().map(|x| (x, Default::default())).collect();

        Self {
            _data: data,
            ..Default::default()
        }
    }

    fn complete<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self::Vertex>,
    {
        // Initialize the data storage using the vertex set.
        let data: Vec<_> = iter.into_iter().collect();
        let data: AdjacencyList<_> = data
            .iter()
            .map(|x| (x.clone(), BTreeSet::from_iter(data.clone())))
            .collect();
        // Compute the final size.
        let size = (data.len() * (data.len() + 1)) / 2;

        Self {
            _data: data,
            _size: size,
            ..Default::default()
        }
    }

    fn clear(&mut self) {
        // Clear the data structures
        self._data.clear();
        // Clear size.
        self._size = 0;
    }

    fn vertices_iter<'a>(&'a self) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a> {
        Box::new(self._data.keys())
    }

    fn edges_iter<'a>(&'a self) -> Box<dyn EdgeIterator<'a, Self::Vertex> + 'a> {
        Box::new(ExactSizeIter::new(
            self._data
                .iter()
                .flat_map(|(x, ys)| std::iter::repeat(x).zip(ys).filter(|(x, y)| x <= y)),
            self.size(),
        ))
    }

    fn adjacents_iter<'a>(&'a self, x: &'a Self::Vertex) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a> {
        Box::new(self._data[x].iter())
    }

    fn order(&self) -> usize {
        self._data.len()
    }

    fn size(&self) -> usize {
        self._size
    }

    fn has_vertex(&self, x: &Self::Vertex) -> bool {
        // Check if map contains key.
        self._data.contains_key(x)
    }

    fn add_vertex(&mut self, x: Self::Vertex) -> bool {
        // If the entry is vacant ...
        if let Entry::Vacant(entry) = self._data.entry(x) {
            // ... insert the default value.
            entry.insert(Default::default());

            return true;
        }

        false
    }

    fn del_vertex(&mut self, x: &Self::Vertex) -> bool {
        // Remove vertex from map.
        if self._data.remove(x).is_some() {
            // Remove remaining identifiers, if any.
            self._data.iter_mut().for_each(|(_, y)| {
                y.remove(x);
            });

            return true;
        }

        false
    }

    fn has_edge(&self, x: &Self::Vertex, y: &Self::Vertex) -> bool {
        // Check vertices indentifiers.
        assert!(
            self._data.contains_key(x),
            "Vertex identifiers does not exist in the graph"
        );
        assert!(
            self._data.contains_key(y),
            "Vertex identifiers does not exist in the graph"
        );

        self._data.get(x).unwrap().contains(y)
    }

    fn add_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> bool {
        // Check vertices indentifiers.
        assert!(
            self._data.contains_key(x),
            "Vertex identifiers does not exist in the graph"
        );
        assert!(
            self._data.contains_key(y),
            "Vertex identifiers does not exist in the graph"
        );

        // Get mutable reference to adjacency set.
        if self._data.get_mut(x).unwrap().insert(y.clone()) {
            // Update reverse reference.
            self._data.get_mut(y).unwrap().insert(x.clone());
            // Update size.
            self._size += 1;

            return true;
        }

        false
    }

    fn del_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> bool {
        // Check vertices indentifiers.
        assert!(
            self._data.contains_key(x),
            "Vertex identifiers does not exist in the graph"
        );
        assert!(
            self._data.contains_key(y),
            "Vertex identifiers does not exist in the graph"
        );

        // Get mutable reference to adjacency set.
        if self._data.get_mut(x).unwrap().remove(y) {
            // Update reverse reference.
            self._data.get_mut(y).unwrap().remove(x);
            // Update size.
            self._size -= 1;

            return true;
        }

        false
    }
}

impl<V, A> Connectivity for UndirectedAdjacencyList<V, A>
where
    V: Vertex,
    A: WithAttributes<V>,
{
    fn has_path(&self, x: &Self::Vertex, y: &Self::Vertex) -> bool {
        // Account for self loops and then execute BFS.
        // NOTE: The difference between performing self.has_edge(z, y),
        // instead of z == y, is due to the possible presence of cycles.
        // Check using tuple windows for incoming edge filtering.
        self.has_edge(x, y)
            || BFS::from((self, x))
                .tuple_windows()
                .any(|(w, z)| self.has_edge(z, y) && w != y)
    }

    fn is_connected(&self) -> bool {
        // A graph is connected if a tree-traversal discover every vertex in the graph.
        BFS::from(self).count() == self.order()
    }

    fn is_acyclic(&self) -> bool {
        // A graph is acyclic if it does not contain any back edge.
        !DFSEdges::new(self, None, Traversal::Forest).any(|e| matches!(e, DFSEdge::Back(_, _)))
    }
}

impl<V, A> Convert for UndirectedAdjacencyList<V, A>
where
    V: Vertex,
    A: WithAttributes<V>,
{
    fn dense_adjacency_matrix(&self) -> DenseAdjacencyMatrix {
        let n = self.order();
        let mut idx = HashMap::with_capacity(n);
        let mut out = DenseAdjacencyMatrix::from_elem((n, n), false);
        // Build vid-to-index mapping.
        idx.extend(V!(self).enumerate().map(|(i, x)| (x, i)));
        // Fill the output matrix.
        for (x, y) in E!(self) {
            let (x, y) = (idx[&x], idx[&y]);
            out[(x, y)] = true;
            out[(y, x)] = true;
        }

        out
    }

    fn sparse_adjacency_matrix(&self) -> SparseAdjacencyMatrix {
        let n = self.order();
        let mut idx = HashMap::with_capacity(n);
        let mut out = SparseAdjacencyMatrix::new((n, n));
        // Reserve capacity for sparse matrix.
        out.reserve(self.size());
        // Build vid-to-index mapping.
        idx.extend(V!(self).enumerate().map(|(i, x)| (x, i)));
        // Fill the output matrix.
        for (x, y) in E!(self) {
            let (x, y) = (idx[&x], idx[&y]);
            out.add_triplet(x, y, true);
            if x != y {
                out.add_triplet(y, x, true);
            }
        }

        out
    }

    fn dense_incidence_matrix(&self) -> Array2<i8> {
        let (n, m) = (self.order(), self.size());
        let mut idx = HashMap::with_capacity(n);
        let mut out = Array2::from_elem((n, m), 0);
        // Build vid-to-index mapping.
        idx.extend(V!(self).enumerate().map(|(i, x)| (x, i)));
        // Fill the output matrix.
        for (i, (x, y)) in E!(self).enumerate() {
            out[(idx[&x], i)] += 1;
            out[(idx[&y], i)] += 1;
        }

        out
    }

    fn sparse_incidence_matrix(&self) -> TriMat<i8> {
        let (n, m) = (self.order(), self.size());
        let mut idx = HashMap::with_capacity(n);
        let mut out = TriMat::new((n, m));
        // Reserve capacity for sparse matrix.
        out.reserve(2 * m);
        // Build vid-to-index mapping.
        idx.extend(V!(self).enumerate().map(|(i, x)| (x, i)));
        // Fill the output matrix.
        for (i, (x, y)) in E!(self).enumerate() {
            if x != y {
                out.add_triplet(idx[&x], i, 1);
                out.add_triplet(idx[&y], i, 1);
            } else {
                out.add_triplet(idx[&x], i, 2);
            }
        }

        out
    }

    fn dense_mark_matrix(&self) -> DenseMarkMatrix {
        self.dense_adjacency_matrix().mapv(|x| match x {
            false => M::None,
            true => M::TailTail,
        })
    }

    fn sparse_mark_matrix(&self) -> SparseMarkMatrix {
        self.sparse_adjacency_matrix()
            .into_iter()
            .map(|(&x, (i, j))| {
                let x = match x {
                    false => M::None,
                    true => M::TailTail,
                };
                (x, i, j)
            })
            .fold(
                {
                    let (n, m) = (self.order(), self.size());
                    SparseMarkMatrix::with_capacity((n, n), m)
                },
                |mut acc, (x, i, j)| {
                    acc.add_triplet(i, j, x);

                    acc
                },
            )
    }
}

crate::traits::impl_extend!(UndirectedAdjacencyList);

crate::traits::impl_from!(UndirectedAdjacencyList);

crate::traits::impl_subgraph!(UndirectedAdjacencyList);

crate::traits::impl_with_attributes!(UndirectedAdjacencyList);

impl<V, A> Undirected for UndirectedAdjacencyList<V, A>
where
    V: Vertex,
    A: WithAttributes<V>,
{
    fn neighbors_iter<'a>(&'a self, x: &'a Self::Vertex) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a> {
        self.adjacents_iter(x)
    }

    fn add_undirected_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> bool {
        self.add_edge(x, y)
    }
}
