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
    fn vertices_iter<'a>(&'a self) -> Box<dyn VertexIterator<&'a Self::Vertex> + 'a> {
        Box::new(self.iter().map(|x| x.0))
    }

    #[inline(always)]
    fn edges_iter<'a>(
        &'a self,
    ) -> Box<dyn EdgeIterator<(&'a Self::Vertex, &'a Self::Vertex)> + 'a> {
        Box::new(self.iter().flat_map(|(x, ys)| std::iter::repeat(x).zip(ys)))
    }

    #[inline(always)]
    fn adjacents_iter<'a, U>(
        &'a self,
        x: &U,
    ) -> Result<Box<dyn VertexIterator<&'a Self::Vertex> + 'a>, Error<Self::Vertex>>
    where
        U: Eq + Clone + Into<Self::Vertex>,
    {
        // Get vertex identifier.
        let i = x.clone().into();
        // Get iterator over adjacent vertices.
        match self.get(&i) {
            Some(i) => Ok(Box::new(i.iter())),
            None => Err(Error::VertexNotDefined(i)),
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
    fn has_vertex<U>(&self, x: &U) -> bool
    where
        U: Eq + Clone + Into<Self::Vertex>,
    {
        // Get vertex identifier.
        let i = x.clone().into();
        // Check if map contains key.
        self.contains_key(&i)
    }

    #[inline(always)]
    fn add_vertex<'a, U>(&mut self, x: &'a U) -> Result<&'a U, Error<Self::Vertex>>
    where
        U: Eq + Clone + Into<Self::Vertex>,
    {
        // Get vertex identifier.
        let i = x.clone().into();
        // TODO: Update using insert once stable.
        if self.has_vertex(x) {
            return Err(Error::VertexAlreadyDefined(i));
        }
        self.insert(i, BTreeSet::new());
        Ok(x)
    }

    #[inline(always)]
    fn del_vertex<'a, U>(&mut self, x: &'a U) -> Result<&'a U, Error<Self::Vertex>>
    where
        U: Eq + Clone + Into<Self::Vertex>,
    {
        // Get vertex identifier.
        let i = x.clone().into();
        // Remove vertex from map.
        match self.remove(&i) {
            // If no vertex found return error.
            None => Err(Error::VertexNotDefined(i)),
            // Otherwise
            Some(_) => {
                // Remove remaining identifiers if any
                for (_, y) in self.iter_mut() {
                    y.remove(&i);
                }
                // Return success
                Ok(x)
            }
        }
    }

    #[inline(always)]
    fn has_edge<'a, U>(&self, (x, y): (&'a U, &'a U)) -> Result<bool, Error<Self::Vertex>>
    where
        U: Eq + Clone + Into<Self::Vertex>,
    {
        // Get vertex identifier.
        let (i, j) = (x.clone().into(), y.clone().into());
        // Get vertex adjacency list.
        match self.get(&i) {
            // If no vertex found return error.
            None => Err(Error::VertexNotDefined(i)),
            // Otherwise check second vertex.
            Some(adj) => match self.contains_key(&j) {
                // If no vertex found return error.
                false => Err(Error::VertexNotDefined(j)),
                // Otherwise check if it is in the adjacency list.
                true => Ok(adj.contains(&j)),
            },
        }
    }

    #[inline(always)]
    fn add_edge<'a, U>(
        &mut self,
        (x, y): (&'a U, &'a U),
    ) -> Result<(&'a U, &'a U), Error<Self::Vertex>>
    where
        U: Eq + Clone + Into<Self::Vertex>,
    {
        // Get vertex identifier.
        let (i, j) = (x.clone().into(), y.clone().into());
        // Check if second vertex exists. NOTE: Check second vertex before first
        // in order to avoid contemporaneous immutable and mutable refs to data.
        match self.contains_key(&j) {
            // If no vertex found return error.
            false => Err(Error::VertexNotDefined(j)),
            // Otherwise get mutable vertex adjacency list.
            true => match self.get_mut(&i) {
                // If no vertex exists return error.
                None => Err(Error::VertexNotDefined(i)),
                // Otherwise try to insert vertex into adjacency list.
                Some(adj) => match adj.insert(j.clone()) {
                    // If edge already defined return error.
                    false => Err(Error::EdgeAlreadyDefined((i, j))),
                    // Otherwise return successful.
                    true => Ok((x, y)),
                },
            },
        }
    }

    #[inline(always)]
    fn del_edge<'a, U>(
        &mut self,
        (x, y): (&'a U, &'a U),
    ) -> Result<(&'a U, &'a U), Error<Self::Vertex>>
    where
        U: Eq + Clone + Into<Self::Vertex>,
    {
        // Get vertex identifier.
        let (i, j) = (x.clone().into(), y.clone().into());
        // Check if second vertex exists.
        match self.contains_key(&j) {
            // If no vertex found return error.
            false => Err(Error::VertexNotDefined(j)),
            // Otherwise get mutable vertex adjacency list.
            true => match self.get_mut(&i) {
                // If no vertex exists return error.
                None => Err(Error::VertexNotDefined(i)),
                // Otherwise try to remove vertex from adjacency list.
                Some(adj) => match adj.remove(&j) {
                    // If no edge defined return error.
                    false => Err(Error::EdgeNotDefined((i, j))),
                    // Otherwise
                    true => {
                        // Return success
                        Ok((x, y))
                    }
                },
            },
        }
    }
}
