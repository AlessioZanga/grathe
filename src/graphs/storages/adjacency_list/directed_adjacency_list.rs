use crate::graphs::attributes::AttributesMap;
use crate::traits::{Connectivity, Convert, Directed, Operators, Storage, WithAttributes};
use crate::types::{directions, AdjacencyList, EdgeIterator, EdgeList, Error, ExactSizeIter, Vertex, VertexIterator};
use crate::{E, V};
use ndarray::Array2;
use sprs::TriMat;
use std::cmp::Ordering;
use std::collections::{btree_map::Entry, BTreeSet, HashMap};

#[derive(Debug, Default)]
pub struct DirectedAdjacencyList<V, A = AttributesMap<V, (), (), ()>>
where
    V: Vertex,
    A: WithAttributes<V>,
{
    _data: AdjacencyList<V>,
    _size: usize,
    _attributes: A,
}

impl<V, A> PartialEq for DirectedAdjacencyList<V, A>
where
    V: Vertex,
    A: WithAttributes<V>,
{
    fn eq(&self, other: &Self) -> bool {
        self._data.eq(&other._data)
    }
}

impl<V, A> Eq for DirectedAdjacencyList<V, A>
where
    V: Vertex,
    A: WithAttributes<V>,
{
}

crate::traits::impl_partial_ord!(DirectedAdjacencyList);

impl<V, A> Operators for DirectedAdjacencyList<V, A>
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
        let size = data.iter().map(|(_, ys)| ys.len()).fold(0, |acc, x| acc + x);

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
        let size = data.iter().map(|(_, ys)| ys.len()).fold(0, |acc, x| acc + x);

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
        let size = data.iter().map(|(_, ys)| ys.len()).fold(0, |acc, x| acc + x);

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
        let size = data.iter().map(|(_, ys)| ys.len()).fold(0, |acc, x| acc + x);

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
        // Compute difference size.
        let size = data.iter().map(|(_, ys)| ys.len()).fold(0, |acc, x| acc + x);

        Self {
            _data: data,
            _size: size,
            ..Default::default()
        }
    }
}

crate::traits::impl_operators_extension!(DirectedAdjacencyList);

impl<V, A> Storage for DirectedAdjacencyList<V, A>
where
    V: Vertex,
    A: WithAttributes<V>,
{
    type Vertex = V;

    type Direction = directions::Directed;

    fn new<I, J>(v_iter: I, e_iter: J) -> Self
    where
        I: IntoIterator<Item = Self::Vertex>,
        J: IntoIterator<Item = (Self::Vertex, Self::Vertex)>,
    {
        // Initialize the data storage using the vertex set.
        let mut size = 0;
        let mut data: AdjacencyList<Self::Vertex> = v_iter.into_iter().map(|x| (x, Default::default())).collect();
        // Fill the data storage using the edge set.
        for (x, y) in e_iter.into_iter() {
            data.entry(y.clone()).or_default();
            data.entry(x).or_default().insert(y);
            size += 1;
        }

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
        // Fill the data storage by copying the vertex set.
        let data: AdjacencyList<Self::Vertex> = data
            .iter()
            .map(|x| (x.clone(), BTreeSet::from_iter(data.clone())))
            .collect();
        let size = data.len() * data.len();

        Self {
            _data: data,
            _size: size,
            ..Default::default()
        }
    }

    fn clear(&mut self) {
        // Clear the data structure.
        self._data.clear();
        // Clear size.
        self._size = 0;
    }

    fn vertices_iter<'a>(&'a self) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a> {
        Box::new(self._data.keys())
    }

    fn edges_iter<'a>(&'a self) -> Box<dyn EdgeIterator<'a, Self::Vertex> + 'a> {
        Box::new(ExactSizeIter::new(
            self._data.iter().flat_map(|(x, ys)| std::iter::repeat(x).zip(ys)),
            self.size(),
        ))
    }

    fn adjacents_iter<'a>(&'a self, x: &'a Self::Vertex) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a> {
        let mut adjacents: BTreeSet<&'a Self::Vertex> = self._data[x].iter().collect();
        for (y, zs) in &self._data {
            if zs.contains(x) {
                adjacents.insert(y);
            }
        }
        Box::new(adjacents.into_iter())
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
        // Check if first vertex exists.
        match self._data.get(x) {
            // No vertex defined.
            None => panic!(),
            // Check if second vertex exists.
            Some(adjacent) => match self._data.contains_key(y) {
                // No vertex defined.
                false => panic!(),
                // Check if it is in the adjacency set.
                true => adjacent.contains(y),
            },
        }
    }

    fn add_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> bool {
        // Check if second vertex exists.
        match self._data.contains_key(y) {
            // No vertex defined.
            false => panic!(),
            // Get mutable vertex adjacency set.
            true => match self._data.get_mut(x) {
                // No vertex defined.
                None => panic!(),
                // Try to insert vertex into adjacency set.
                Some(adjacent) => {
                    if adjacent.insert(y.clone()) {
                        // Update size.
                        self._size += 1;

                        return true;
                    }
                }
            },
        }

        false
    }

    fn del_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> bool {
        // Check if second vertex exists.
        match self._data.contains_key(y) {
            // No vertex defined.
            false => panic!(),
            // Get mutable vertex adjacency set.
            true => match self._data.get_mut(x) {
                // No vertex defined.
                None => panic!(),
                // Try to remove vertex from adjacency set.
                Some(adjacent) => {
                    if adjacent.remove(y) {
                        // Update size.
                        self._size -= 1;

                        return true;
                    }
                }
            },
        }

        false
    }
}

crate::traits::impl_capacity!(DirectedAdjacencyList);

impl<V, A> Connectivity for DirectedAdjacencyList<V, A>
where
    V: Vertex,
    A: WithAttributes<V>,
{
    fn has_path(&self, x: &Self::Vertex, y: &Self::Vertex) -> bool {
        // FIXME:
        todo!()
    }

    fn is_connected(&self) -> bool {
        // FIXME:
        todo!()
    }

    fn is_acyclic(&self) -> bool {
        // FIXME:
        todo!()
    }
}

impl<V, A> Convert for DirectedAdjacencyList<V, A>
where
    V: Vertex,
    A: WithAttributes<V>,
{
    fn edge_list(&self) -> EdgeList<Self::Vertex> {
        let mut out = EdgeList::<Self::Vertex>::new();
        for (x, y) in E!(self) {
            out.insert((x.clone(), y.clone()));
        }

        out
    }

    fn adjacency_list(&self) -> AdjacencyList<Self::Vertex> {
        let mut out = AdjacencyList::<Self::Vertex>::new();
        for x in V!(self) {
            out.entry(x.clone()).or_default();
        }
        for (x, y) in E!(self) {
            out.entry(x.clone()).or_default().insert(y.clone());
        }

        out
    }

    fn dense_adjacency_matrix(&self) -> Array2<bool> {
        let n = self.order();
        let mut idx = HashMap::with_capacity(n);
        let mut out = Array2::from_elem((n, n), false);
        // Build vid-to-index mapping.
        idx.extend(V!(self).enumerate().map(|(i, x)| (x, i)));
        // Fill the output matrix.
        for (x, y) in E!(self) {
            out[(idx[&x], idx[&y])] = true;
        }

        out
    }

    fn sparse_adjacency_matrix(&self) -> TriMat<bool> {
        let n = self.order();
        let mut idx = HashMap::with_capacity(n);
        let mut out = TriMat::new((n, n));
        // Reserve capacity for sparse matrix.
        out.reserve(self.size());
        // Build vid-to-index mapping.
        idx.extend(V!(self).enumerate().map(|(i, x)| (x, i)));
        // Fill the output matrix.
        for (x, y) in E!(self) {
            out.add_triplet(idx[&x], idx[&y], true);
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
            out[(idx[&x], i)] -= 1;
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
                out.add_triplet(idx[&x], i, -1);
                out.add_triplet(idx[&y], i, 1);
            } else {
                out.add_triplet(idx[&x], i, 0);
            }
        }

        out
    }
}

crate::traits::impl_extend!(DirectedAdjacencyList);

crate::traits::impl_from!(DirectedAdjacencyList);

crate::traits::impl_subgraph!(DirectedAdjacencyList);

crate::traits::impl_with_attributes!(DirectedAdjacencyList);

impl<V, A> Directed for DirectedAdjacencyList<V, A>
where
    V: Vertex,
    A: WithAttributes<V>,
{
    fn parents_iter<'a>(&'a self, x: &'a Self::Vertex) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a> {
        assert!(self.has_vertex(x));
        Box::new(self._data.iter().filter_map(|(y, z)| match z.contains(x) {
            false => None,
            true => Some(y),
        }))
    }

    fn children_iter<'a>(&'a self, x: &'a Self::Vertex) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a> {
        Box::new(self._data[x].iter())
    }

    fn add_directed_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> bool {
        self.add_edge(x, y)
    }
}
