use crate::errors::Error;
use crate::storages::StorageTrait;
use crate::types::*;
use std::collections::BTreeSet;

/// Graph structure based on adjacency list storage.
impl<T> StorageTrait for AdjacencyList<T>
where
    T: VertexTrait,
{
    type Vertex = T;

    type Storage = AdjacencyList<T>;

    fn new() -> Self {
        Self::new()
    }

    fn clear(&mut self) {
        // Clear the data structures
        self.clear();
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
        Box::new(self.iter().map(|x| x.0))
    }

    fn edges_iter<'a>(&'a self) -> Box<dyn EdgeIterator<'a, Self::Vertex> + 'a> {
        Box::new(ExactSizeIter::new(
            self.iter().flat_map(|(x, ys)| std::iter::repeat(x).zip(ys)),
            self.size(),
        ))
    }

    fn adjacents_iter<'a>(
        &'a self,
        x: &Self::Vertex,
    ) -> Result<Box<dyn VertexIterator<'a, Self::Vertex> + 'a>, Error<Self::Vertex>> {
        // Get iterator over adjacent vertices.
        match self.get(x) {
            Some(x) => Ok(Box::new(x.iter())),
            None => Err(Error::VertexNotDefined(x.clone())),
        }
    }

    fn order(&self) -> usize {
        // Get map size.
        self.len()
    }

    fn size(&self) -> usize {
        self.iter() // Iterate over the adjacency lists.
            .map(|(_, adj)| adj.len())
            .sum::<usize>() // Accumulate their sizes.
    }

    fn has_vertex(&self, x: &Self::Vertex) -> bool {
        // Check if map contains key.
        self.contains_key(x)
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
        self.insert(x.clone(), BTreeSet::new());
        Ok(x)
    }

    fn del_vertex(&mut self, x: &Self::Vertex) -> Result<(), Error<Self::Vertex>> {
        // Remove vertex from map.
        match self.remove(x) {
            // If no vertex found return error.
            None => Err(Error::VertexNotDefined(x.clone())),
            // Otherwise
            Some(_) => {
                // Remove remaining identifiers if any
                for (_, y) in self.iter_mut() {
                    y.remove(x);
                }
                // Return success
                Ok(())
            }
        }
    }

    fn has_edge(&self, x: &Self::Vertex, y: &Self::Vertex) -> Result<bool, Error<Self::Vertex>> {
        // Get vertex adjacency list.
        match self.get(x) {
            // If no vertex found return error.
            None => Err(Error::VertexNotDefined(x.clone())),
            // Otherwise check second vertex.
            Some(adj) => match self.contains_key(y) {
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
        match self.contains_key(y) {
            // If no vertex found return error.
            false => Err(Error::VertexNotDefined(y.clone())),
            // Otherwise get mutable vertex adjacency list.
            true => match self.get_mut(x) {
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
        match self.contains_key(y) {
            // If no vertex found return error.
            false => Err(Error::VertexNotDefined(y.clone())),
            // Otherwise get mutable vertex adjacency list.
            true => match self.get_mut(x) {
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
