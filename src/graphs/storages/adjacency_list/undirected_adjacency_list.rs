use crate::graphs::attributes::AttributesMap;
use crate::traits::{Connectivity, Convert, Operators, Storage, Undirected, WithAttributes};
use crate::types::{directions, AdjacencyList, EdgeIterator, EdgeList, Error, ExactSizeIter, Vertex, VertexIterator};
use ndarray::Array2;
use sprs::TriMat;
use std::cmp::Ordering;
use std::collections::{btree_map::Entry, BTreeSet, HashMap};

#[derive(Debug, Default)]
pub struct UndirectedAdjacencyList<V, A = AttributesMap<V, (), (), ()>>
where
    V: Vertex,
    A: WithAttributes<V>,
{
    _data: AdjacencyList<V>,
    _size: usize,
    _attributes: A,
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
        let size = data
            .iter()
            .map(|(x, ys)| ys.iter().filter(|y| x <= y).count())
            .fold(0, |acc, x| acc + x);

        Self {
            _data: data,
            _size: size,
            ..Default::default()
        }
    }

    fn union(&self, other: &Self) -> Self {
        // Clone internal storage status.
        let mut union = self._data.clone();

        // For each other entry.
        for (x, ys) in other._data.iter() {
            // Get the correspondent entry on the union (or default it).
            let zs = union.entry(x.clone()).or_default();
            // Perform union of entries.
            *zs = zs.union(ys).cloned().collect();
        }

        // Return the union graph.
        Self {
            _data: union,
            ..Default::default()
        }
    }

    fn intersection(&self, other: &Self) -> Self {
        Self {
            _data: self
                ._data
                .iter()
                // For each entry in the current graph.
                .filter_map(|(x, ys)| {
                    other._data.get(x).map(|zs| {
                        // If there a common vertex, then intersect its adjacent vertices.
                        (x.clone(), ys.intersection(zs).cloned().collect())
                    })
                })
                .collect(),
            ..Default::default()
        }
    }

    fn symmetric_difference(&self, other: &Self) -> Self {
        // Clone internal storage status.
        let mut symmetric_difference = self._data.clone();

        // For each other entry.
        for (x, ys) in other._data.iter() {
            // Get the correspondent entry on the union (or default it).
            let zs = symmetric_difference.entry(x.clone()).or_default();
            // Perform symmetric difference of entries.
            *zs = zs.symmetric_difference(ys).cloned().collect();
        }

        // Return the symmetric difference graph.
        Self {
            _data: symmetric_difference,
            ..Default::default()
        }
    }

    fn difference(&self, other: &Self) -> Self {
        Self {
            _data: self
                ._data
                .iter()
                // For each entry in the current graph.
                .map(|(x, ys)| match other._data.get(x) {
                    // If there a common vertex, then subtract its adjacent vertices.
                    Some(zs) => (x.clone(), ys.difference(zs).cloned().collect()),
                    // Else return as it is.
                    None => (x.clone(), ys.clone()),
                })
                .collect(),
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
        let mut size = 0;
        let mut data: AdjacencyList<Self::Vertex> = v_iter.into_iter().map(|x| (x, Default::default())).collect();
        // Fill the data storage using the edge set.
        for (x, y) in e_iter.into_iter() {
            data.entry(x.clone()).or_default().insert(y.clone());
            data.entry(y).or_default().insert(x);
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
        // Compute the final size.
        let size = (data.len() * (data.len() + 1)) / 2;

        Self {
            _data: data
                .iter()
                .map(|x| (x.clone(), BTreeSet::from_iter(data.clone())))
                .collect(),
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
                        // Add reversed edge.
                        self._data.get_mut(y).unwrap().insert(x.clone());
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
                        // Delete reversed edge.
                        self._data.get_mut(y).unwrap().remove(x);
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

crate::traits::impl_capacity!(UndirectedAdjacencyList);

impl<V, A> Connectivity for UndirectedAdjacencyList<V, A>
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

impl<V, A> Convert for UndirectedAdjacencyList<V, A>
where
    V: Vertex,
    A: WithAttributes<V>,
{
    fn edge_list(&self) -> EdgeList<Self::Vertex> {
        let mut out = EdgeList::<Self::Vertex>::new();
        for (x, y) in self.edges_iter() {
            out.insert((x.clone(), y.clone()));
            out.insert((y.clone(), x.clone()));
        }

        out
    }

    fn adjacency_list(&self) -> AdjacencyList<Self::Vertex> {
        let mut out = AdjacencyList::<Self::Vertex>::new();
        for x in self.vertices_iter() {
            out.entry(x.clone()).or_default();
        }
        for (x, y) in self.edges_iter() {
            out.entry(x.clone()).or_default().insert(y.clone());
            out.entry(y.clone()).or_default().insert(x.clone());
        }

        out
    }

    fn dense_adjacency_matrix(&self) -> Array2<bool> {
        let n = self.order();
        let mut idx = HashMap::with_capacity(n);
        let mut out = Array2::from_elem((n, n), false);
        // Build vid-to-index mapping.
        idx.extend(self.vertices_iter().enumerate().map(|(i, x)| (x, i)));
        // Fill the output matrix.
        for (x, y) in self.edges_iter() {
            let (x, y) = (idx[&x], idx[&y]);
            out[(x, y)] = true;
            out[(y, x)] = true;
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
        idx.extend(self.vertices_iter().enumerate().map(|(i, x)| (x, i)));
        // Fill the output matrix.
        for (x, y) in self.edges_iter() {
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
        idx.extend(self.vertices_iter().enumerate().map(|(i, x)| (x, i)));
        // Fill the output matrix.
        for (i, (x, y)) in self.edges_iter().filter(|(x, y)| x <= y).enumerate() {
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
        idx.extend(self.vertices_iter().enumerate().map(|(i, x)| (x, i)));
        // Fill the output matrix.
        for (i, (x, y)) in self.edges_iter().filter(|(x, y)| x <= y).enumerate() {
            if x != y {
                out.add_triplet(idx[&x], i, 1);
                out.add_triplet(idx[&y], i, 1);
            } else {
                out.add_triplet(idx[&x], i, 2);
            }
        }

        out
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
