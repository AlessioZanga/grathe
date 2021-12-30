use crate::errors::Error;
use crate::partial_cmp_sets;
use crate::storages::StorageTrait;
use crate::types::*;
use std::cmp::Ordering;
use std::collections::HashSet;

/// Graph structure based on adjacency list storage.
#[derive(PartialEq, Eq, Default, Debug)]
pub struct AdjacencyListStorage<T>
where
    T: VertexTrait,
{
    data: AdjacencyList<T>,
}

impl<T> PartialOrd for AdjacencyListStorage<T>
where
    T: VertexTrait,
{
    // TODO: That's a generic implementation of PartialOrd,
    // which is fine, but here we would like to have an
    // optimized implementation to boost performances...
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Collect vertex sets for comparison.
        let a: HashSet<_> = self.vertices_iter().collect();
        let b: HashSet<_> = other.vertices_iter().collect();

        // Partial ordering of vertex sets.
        let vertices: Option<Ordering> = partial_cmp_sets!(a, b);

        // If vertices are comparable.
        if let Some(vertices) = vertices {
            // Collect edge sets for comparison.
            let a: HashSet<_> = self.edges_iter().collect();
            let b: HashSet<_> = other.edges_iter().collect();

            // Partial ordering of edge sets.
            let edges: Option<Ordering> = partial_cmp_sets!(a, b);

            // If edges are comparable.
            if let Some(edges) = edges {
                // If vertices are equal,
                // then order is determined by edges.
                if matches!(vertices, Ordering::Equal) {
                    return Some(edges);
                }
                // If vertices are different but edges are equal,
                // then order is determined by vertices.
                if matches!(edges, Ordering::Equal) {
                    return Some(vertices);
                }
                // If orders are coherent, then return the order.
                if vertices == edges {
                    return Some(vertices);
                }
            }
        }

        // Otherwise, self and other are not comparable.
        None
    }
}

impl<T> StorageTrait for AdjacencyListStorage<T>
where
    T: VertexTrait,
{
    type Vertex = T;

    type Storage = AdjacencyList<T>;

    fn new() -> Self {
        Self {
            data: Self::Storage::new(),
        }
    }

    fn storage(&self) -> &Self::Storage {
        &self.data
    }

    fn clear(&mut self) {
        // Clear the data structures
        self.data.clear();
    }

    fn capacity(&self) -> usize {
        // INFO: BTreeMap as no `capacity` concept.
        0
    }

    fn with_capacity(_capacity: usize) -> Self {
        // INFO: BTreeMap as no `capacity` concept.
        Self::new()
    }

    fn reserve(&mut self, _additional: usize) {
        // INFO: BTreeMap as no `capacity` concept.
    }

    fn shrink_to(&mut self, _min_capacity: usize) {
        // INFO: BTreeMap as no `capacity` concept.
    }

    fn shrink_to_fit(&mut self) {
        // INFO: BTreeMap as no `capacity` concept.
    }

    fn vertices_iter<'a>(&'a self) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a> {
        Box::new(self.data.iter().map(|x| x.0))
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
        // Get map size.
        self.data.len()
    }

    fn size(&self) -> usize {
        self.data
            .iter() // Iterate over the adjacency lists.
            .map(|(_, adj)| adj.len())
            .sum::<usize>() // Accumulate their sizes.
    }

    fn has_vertex(&self, x: &Self::Vertex) -> bool {
        // Check if map contains key.
        self.data.contains_key(x)
    }

    fn add_vertex<U>(&mut self, x: &U) -> Result<Self::Vertex, Error<Self::Vertex>>
    where
        U: Eq + Clone + Into<Self::Vertex>,
    {
        // Get vertex identifier.
        let x = x.clone().into();
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
