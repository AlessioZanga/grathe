use crate::graphs::attributes::AttributesMap;
use crate::traits::{Connectivity, Convert, Operators, Storage, Undirected, WithAttributes};
use crate::types::{directions, EdgeIterator, Error, ExactSizeIter, Vertex, VertexIterator};
use ndarray::Array2;
use sprs::TriMat;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap};

#[derive(Debug, Default)]
pub struct UndirectedAdjacencyList<V, A = AttributesMap<V, (), (), ()>>
where
    V: Vertex,
    A: WithAttributes<V>,
{
    data: BTreeMap<V, BTreeSet<V>>,
    attributes: A,
}

impl<V, A> PartialEq for UndirectedAdjacencyList<V, A>
where
    V: Vertex,
    A: WithAttributes<V>,
{
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data)
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
        let vertices: BTreeSet<_> = self.data.keys().cloned().collect();

        // Iterate over every permutation of V^2.
        Self {
            data: vertices
                .iter()
                .map(|x| (x.clone(), vertices.difference(&self.data[x]).cloned().collect()))
                .collect(),
            attributes: Default::default(),
        }
    }

    fn union(&self, other: &Self) -> Self {
        // Clone internal storage status.
        let mut union = self.data.clone();

        // For each other entry.
        for (x, ys) in other.data.iter() {
            // Get the correspondent entry on the union (or default it).
            let zs = union.entry(x.clone()).or_default();
            // Perform union of entries.
            *zs = zs.union(ys).cloned().collect();
        }

        // Return the union graph.
        Self {
            data: union,
            attributes: Default::default(),
        }
    }

    fn intersection(&self, other: &Self) -> Self {
        Self {
            data: self
                .data
                .iter()
                // For each entry in the current graph.
                .filter_map(|(x, ys)| {
                    other.data.get(x).map(|zs| {
                        // If there a common vertex, then intersect its adjacent vertices.
                        (x.clone(), ys.intersection(zs).cloned().collect())
                    })
                })
                .collect(),
            attributes: Default::default(),
        }
    }

    fn symmetric_difference(&self, other: &Self) -> Self {
        // Clone internal storage status.
        let mut symmetric_difference = self.data.clone();

        // For each other entry.
        for (x, ys) in other.data.iter() {
            // Get the correspondent entry on the union (or default it).
            let zs = symmetric_difference.entry(x.clone()).or_default();
            // Perform symmetric difference of entries.
            *zs = zs.symmetric_difference(ys).cloned().collect();
        }

        // Return the symmetric difference graph.
        Self {
            data: symmetric_difference,
            attributes: Default::default(),
        }
    }

    fn difference(&self, other: &Self) -> Self {
        Self {
            data: self
                .data
                .iter()
                // For each entry in the current graph.
                .map(|(x, ys)| match other.data.get(x) {
                    // If there a common vertex, then subtract its adjacent vertices.
                    Some(zs) => (x.clone(), ys.difference(zs).cloned().collect()),
                    // Else return as it is.
                    None => (x.clone(), ys.clone()),
                })
                .collect(),
            attributes: Default::default(),
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

    fn new<I, J, K>(v_iter: I, e_iter: J) -> Self
    where
        I: IntoIterator<Item = K>,
        J: IntoIterator<Item = (K, K)>,
        K: Into<Self::Vertex>,
    {
        // Initialize the data storage using the vertex set.
        let mut data: BTreeMap<V, BTreeSet<V>> = v_iter.into_iter().map(|x| (x.into(), Default::default())).collect();
        // Fill the data storage using the edge set.
        for (x, y) in e_iter.into_iter().map(|(x, y)| (x.into(), y.into())) {
            data.entry(x.clone()).or_default().insert(y.clone());
            data.entry(y).or_default().insert(x);
        }

        Self {
            data: data,
            attributes: Default::default(),
        }
    }

    fn null() -> Self {
        Default::default()
    }

    fn empty<I, K>(iter: I) -> Self
    where
        I: IntoIterator<Item = K>,
        K: Into<Self::Vertex>,
    {
        Self {
            data: iter.into_iter().map(|x| (x.into(), Default::default())).collect(),
            attributes: Default::default(),
        }
    }

    fn complete<I, J>(iter: I) -> Self
    where
        I: IntoIterator<Item = J>,
        J: Into<Self::Vertex>,
    {
        // Initialize the data storage using the vertex set.
        let data: Vec<Self::Vertex> = iter.into_iter().map(Into::into).collect();
        // Fill the data storage by copying the vertex set.
        Self {
            data: data
                .iter()
                .map(|x| (x.clone(), BTreeSet::from_iter(data.clone())))
                .collect(),
            attributes: Default::default(),
        }
    }

    fn clear(&mut self) {
        // Clear the data structures
        self.data.clear();
    }

    fn vertices_iter<'a>(&'a self) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a> {
        Box::new(self.data.keys())
    }

    fn edges_iter<'a>(&'a self) -> Box<dyn EdgeIterator<'a, Self::Vertex> + 'a> {
        Box::new(ExactSizeIter::new(
            self.data.iter().flat_map(|(x, ys)| std::iter::repeat(x).zip(ys)),
            self.size(),
        ))
    }

    fn adjacents_iter<'a>(&'a self, x: &'a Self::Vertex) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a> {
        assert!(self.has_vertex(x));
        // Get iterator over adjacent vertices.
        Box::new(self.data[x].iter())
    }

    fn order(&self) -> usize {
        // FIXME: Use constant.
        // Get map size.
        self.data.len()
    }

    fn size(&self) -> usize {
        // FIXME: Use constant.
        self.data
            .iter() // Iterate over the adjacency lists.
            .map(|(_, adj)| adj.len())
            .sum::<usize>() // Accumulate their sizes.
    }

    fn has_vertex(&self, x: &Self::Vertex) -> bool {
        // Check if map contains key.
        self.data.contains_key(x)
    }

    fn add_vertex<K>(&mut self, x: K) -> Result<Self::Vertex, Error<Self::Vertex>>
    where
        K: Into<Self::Vertex>,
    {
        // Get vertex identifier.
        let x = x.into();
        // TODO: Update using insert once stable.
        if self.has_vertex(&x) {
            return Err(Error::VertexAlreadyDefined(x));
        }
        self.data.insert(x.clone(), Default::default());
        Ok(x)
    }

    fn del_vertex(&mut self, x: &Self::Vertex) -> Result<(), Error<Self::Vertex>> {
        // Remove vertex from map.
        match self.data.remove(x) {
            // If no vertex found return error.
            None => Err(Error::VertexNotDefined(x.clone())),
            // Otherwise
            Some(_) => {
                // Remove remaining identifiers if any
                for (_, y) in self.data.iter_mut() {
                    y.remove(x);
                }
                // Return success
                Ok(())
            }
        }
    }

    fn has_edge(&self, x: &Self::Vertex, y: &Self::Vertex) -> Result<bool, Error<Self::Vertex>> {
        // Get vertex adjacency list.
        match self.data.get(x) {
            // If no vertex found return error.
            None => Err(Error::VertexNotDefined(x.clone())),
            // Otherwise check second vertex.
            Some(adj) => match self.data.contains_key(y) {
                // If no vertex found return error.
                false => Err(Error::VertexNotDefined(y.clone())),
                // Otherwise check if it is in the adjacency list.
                true => Ok(adj.contains(y)),
            },
        }
    }

    fn add_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Result<(), Error<Self::Vertex>> {
        // FIXME:
        // Check if second vertex exists. NOTE: Check second vertex before first
        // in order to avoid contemporaneous immutable and mutable refs to data.
        match self.data.contains_key(y) {
            // If no vertex found return error.
            false => Err(Error::VertexNotDefined(y.clone())),
            // Otherwise get mutable vertex adjacency list.
            true => match self.data.get_mut(x) {
                // If no vertex exists return error.
                None => Err(Error::VertexNotDefined(x.clone())),
                // Otherwise try to insert vertex into adjacency list.
                Some(adj) => match adj.insert(y.clone()) {
                    // If edge already defined return error.
                    false => Err(Error::EdgeAlreadyDefined(x.clone(), y.clone())),
                    // Otherwise return successful.
                    true => Ok(()),
                },
            },
        }
    }

    fn del_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Result<(), Error<Self::Vertex>> {
        // FIXME:
        // Check if second vertex exists.
        match self.data.contains_key(y) {
            // If no vertex found return error.
            false => Err(Error::VertexNotDefined(y.clone())),
            // Otherwise get mutable vertex adjacency list.
            true => match self.data.get_mut(x) {
                // If no vertex exists return error.
                None => Err(Error::VertexNotDefined(x.clone())),
                // Otherwise try to remove vertex from adjacency list.
                Some(adj) => match adj.remove(y) {
                    // If no edge defined return error.
                    false => Err(Error::EdgeNotDefined(x.clone(), y.clone())),
                    // Otherwise return success
                    true => Ok(()),
                },
            },
        }
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
    fn edge_list(&self) -> BTreeSet<(Self::Vertex, Self::Vertex)> {
        let mut out = BTreeSet::<(Self::Vertex, Self::Vertex)>::new();
        for (x, y) in self.edges_iter() {
            out.insert((x.clone(), y.clone()));
        }

        out
    }

    fn adjacency_list(&self) -> BTreeMap<Self::Vertex, BTreeSet<Self::Vertex>> {
        let mut out = BTreeMap::<Self::Vertex, BTreeSet<Self::Vertex>>::new();
        for (x, y) in self.edges_iter() {
            out.entry(x.clone()).or_default().insert(y.clone());
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
        idx.extend(self.vertices_iter().enumerate().map(|(i, x)| (x, i)));
        // Fill the output matrix.
        for (x, y) in self.edges_iter() {
            out.add_triplet(idx[&x], idx[&y], true);
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
            out[(idx[&x], i)] = 1;
            out[(idx[&y], i)] = -1;
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
            out.add_triplet(idx[&x], i, 1);
            out.add_triplet(idx[&y], i, -1);
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

    fn add_undirected_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Result<(), Error<Self::Vertex>> {
        self.add_edge(x, y)
    }
}
