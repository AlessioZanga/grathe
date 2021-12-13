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

    #[inline(always)]
    fn new() -> Self {
        Self::new()
    }

    #[inline(always)]
    fn clear(&mut self) {
        // Clear the data structures
        self.clear();
    }

    #[inline(always)]
    fn capacity(&self) -> usize {
        // INFO: BTreeMap as no `capacity` concept.
        0
    }

    #[inline(always)]
    fn with_capacity(_capacity: usize) -> Self {
        // INFO: BTreeMap as no `capacity` concept.
        Self::new()
    }

    #[inline(always)]
    fn reserve(&mut self, _additional: usize) {
        // INFO: BTreeMap as no `capacity` concept.
    }

    #[inline(always)]
    fn shrink_to(&mut self, _min_capacity: usize) {
        // INFO: BTreeMap as no `capacity` concept.
    }

    #[inline(always)]
    fn shrink_to_fit(&mut self) {
        // INFO: BTreeMap as no `capacity` concept.
    }

    #[inline(always)]
    fn vertices_iter<'a>(&'a self) -> Box<dyn VertexIterator<Self::Vertex> + 'a> {
        Box::new(self.iter().map(|x| x.0).copied())
    }

    #[inline(always)]
    fn edges_iter<'a>(&'a self) -> Box<dyn EdgeIterator<Self::Vertex> + 'a> {
        Box::new(self.iter().flat_map(|(x, ys)| ys.iter().map(|y| (*x, *y))))
    }

    #[inline(always)]
    fn adjacents_iter<'a>(
        &'a self,
        x: &Self::Vertex,
    ) -> Result<Box<dyn VertexIterator<Self::Vertex> + 'a>, Error<Self::Vertex>> {
        match self.get(x) {
            Some(i) => Ok(Box::new(i.iter().copied())),
            None => Err(Error::VertexNotDefined(*x)),
        }
    }

    #[inline(always)]
    fn order(&self) -> usize {
        // Get map size.
        self.len()
    }

    #[inline(always)]
    fn size(&self) -> usize {
        self.iter() // Iterate over the adjacency lists.
            .map(|(_, adj)| adj.len())
            .sum::<usize>() // Accumulate their sizes.
    }

    #[inline(always)]
    fn has_vertex(&self, x: &Self::Vertex) -> bool {
        // Check if map contains key.
        self.contains_key(x)
    }

    #[inline(always)]
    fn add_vertex(&mut self, x: &Self::Vertex) -> Result<Self::Vertex, Error<Self::Vertex>> {
        // TODO: Update using insert once stable.
        if self.has_vertex(x) {
            return Err(Error::VertexAlreadyDefined(*x));
        }
        self.insert(*x, BTreeSet::new());
        Ok(*x)
    }

    #[inline(always)]
    fn del_vertex(&mut self, x: &Self::Vertex) -> Result<Self::Vertex, Error<Self::Vertex>> {
        // Remove vertex from map.
        match self.remove(x) {
            // If no vertex found return error.
            None => Err(Error::VertexNotDefined(*x)),
            // Otherwise
            Some(_) => {
                // Remove remaining identifiers if any
                for (_, y) in self.iter_mut() {
                    y.remove(x);
                }
                // Return success
                Ok(*x)
            }
        }
    }

    #[inline(always)]
    fn has_edge(&self, e: &(Self::Vertex, Self::Vertex)) -> Result<bool, Error<Self::Vertex>> {
        // Get vertex adjacency list.
        match self.get(&e.0) {
            // If no vertex found return error.
            None => Err(Error::VertexNotDefined(e.0)),
            // Otherwise check second vertex.
            Some(adj) => match self.contains_key(&e.1) {
                // If no vertex found return error.
                false => Err(Error::VertexNotDefined(e.1)),
                // Otherwise check if it is in the adjacency list.
                true => Ok(adj.contains(&e.1)),
            },
        }
    }

    #[inline(always)]
    fn add_edge(
        &mut self,
        e: &(Self::Vertex, Self::Vertex),
    ) -> Result<(Self::Vertex, Self::Vertex), Error<Self::Vertex>> {
        // Check if second vertex exists. NOTE: Check second vertex before first
        // in order to avoid contemporaneous immutable and mutable refs to data.
        match self.contains_key(&e.1) {
            // If no vertex found return error.
            false => Err(Error::VertexNotDefined(e.1)),
            // Otherwise get mutable vertex adjacency list.
            true => match self.get_mut(&e.0) {
                // If no vertex exists return error.
                None => Err(Error::VertexNotDefined(e.0)),
                // Otherwise try to insert vertex into adjacency list.
                Some(adj) => match adj.insert(e.1) {
                    // If edge already defined return error.
                    false => Err(Error::EdgeAlreadyDefined(*e)),
                    // Otherwise return successful.
                    true => Ok(*e),
                },
            },
        }
    }

    #[inline(always)]
    fn del_edge(
        &mut self,
        e: &(Self::Vertex, Self::Vertex),
    ) -> Result<(Self::Vertex, Self::Vertex), Error<Self::Vertex>> {
        // Check if second vertex exists.
        match self.contains_key(&e.1) {
            // If no vertex found return error.
            false => Err(Error::VertexNotDefined(e.1)),
            // Otherwise get mutable vertex adjacency list.
            true => match self.get_mut(&e.0) {
                // If no vertex exists return error.
                None => Err(Error::VertexNotDefined(e.0)),
                // Otherwise try to remove vertex from adjacency list.
                Some(adj) => match adj.remove(&e.1) {
                    // If no edge defined return error.
                    false => Err(Error::EdgeNotDefined(*e)),
                    // Otherwise
                    true => {
                        // Return success
                        Ok(*e)
                    }
                },
            },
        }
    }
}
