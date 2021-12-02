use crate::errors::Error;
use crate::storages::StorageTrait;
use crate::types::*;
use std::cmp::Ordering;
use std::collections::BTreeSet;

/// Graph structure based on adjacency list storage.
#[derive(Debug)]
pub struct AdjacencyListStorage<T>
where
    T: VertexTrait,
{
    data: AdjacencyList<T>,
    v_labels: LabelMap<T>,
    e_labels: LabelMap<(T, T)>,
}

impl<T> PartialEq for AdjacencyListStorage<T>
where
    T: VertexTrait,
{
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        // Compare maps.
        self.data == other.data
    }
}

impl<T> Eq for AdjacencyListStorage<T> where T: VertexTrait {}

impl<T> PartialOrd for AdjacencyListStorage<T>
where
    T: VertexTrait,
{
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Compare maps.
        self.data.partial_cmp(&other.data)
    }
}

impl<T> Default for AdjacencyListStorage<T>
where
    T: VertexTrait,
{
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> StorageTrait for AdjacencyListStorage<T>
where
    T: VertexTrait,
{
    type Vertex = T;

    #[inline(always)]
    fn new() -> Self {
        AdjacencyListStorage {
            data: AdjacencyList::<T>::new(),
            v_labels: LabelMap::new(),
            e_labels: LabelMap::new(),
        }
    }

    #[inline(always)]
    fn vertices_iter<'a>(&'a self) -> Box<dyn VertexIterator<Self::Vertex> + 'a> {
        Box::new(self.data.iter().map(|x| x.0).copied())
    }

    #[inline(always)]
    fn edges_iter<'a>(&'a self) -> Box<dyn EdgeIterator<Self::Vertex> + 'a> {
        Box::new(
            self.data
                .iter()
                .flat_map(|(x, ys)| std::iter::repeat(x).copied().zip(ys.iter().copied())),
        )
    }

    #[inline(always)]
    fn adjacents_iter<'a>(
        &'a self,
        x: &Self::Vertex,
    ) -> Result<Box<dyn VertexIterator<Self::Vertex> + 'a>, Error<Self::Vertex>> {
        match self.data.get(x) {
            Some(i) => Ok(Box::new(i.iter().copied())),
            None => Err(Error::VertexNotDefined(*x)),
        }
    }

    #[inline(always)]
    fn as_vertices_labels(&self) -> &LabelMap<Self::Vertex> {
        &self.v_labels
    }

    #[inline(always)]
    fn as_mut_vertices_labels(&mut self) -> &mut LabelMap<Self::Vertex> {
        &mut self.v_labels
    }

    #[inline(always)]
    fn as_edges_labels(&self) -> &LabelMap<(Self::Vertex, Self::Vertex)> {
        &self.e_labels
    }

    #[inline(always)]
    fn as_mut_edges_labels(&mut self) -> &mut LabelMap<(Self::Vertex, Self::Vertex)> {
        &mut self.e_labels
    }

    #[inline(always)]
    fn order(&self) -> usize {
        // Get map size.
        self.data.len()
    }

    #[inline(always)]
    fn size(&self) -> usize {
        self.data
            .iter() // Iterate over the adjacency lists.
            .map(|(_, adj)| adj.len())
            .sum::<usize>() // Accumulate their sizes.
    }

    #[inline(always)]
    fn has_vertex(&self, x: &Self::Vertex) -> bool {
        // Check if map contains key.
        self.data.contains_key(x)
    }

    #[inline(always)]
    fn add_vertex(&mut self) -> Result<Self::Vertex, Error<Self::Vertex>> {
        // Increase last identifier or get default.
        let x = self
            .data
            .iter()
            .rev()
            .next()
            .map(|x| *x.0 + Self::Vertex::one())
            .unwrap_or_else(Self::Vertex::zero);
        // Reserve new identifier
        self.reserve_vertex(&x)
    }

    #[inline(always)]
    fn reserve_vertex(&mut self, x: &Self::Vertex) -> Result<Self::Vertex, Error<Self::Vertex>> {
        // TODO: Update using insert once stable.
        if self.has_vertex(x) {
            return Err(Error::VertexAlreadyDefined(*x));
        }
        self.data.insert(*x, BTreeSet::new());
        Ok(*x)
    }

    #[inline(always)]
    fn del_vertex(&mut self, x: &Self::Vertex) -> Result<Self::Vertex, Error<Self::Vertex>> {
        // Remove vertex from map.
        match self.data.remove(x) {
            // If no vertex found return error.
            None => Err(Error::VertexNotDefined(*x)),
            // Otherwise
            Some(_) => {
                // Remove remaining identifiers if any
                for (_, y) in self.data.iter_mut() {
                    y.remove(x);
                }
                // Remove associated label if any
                self.as_mut_vertices_labels().remove_by_left(x);
                // Return success
                Ok(*x)
            }
        }
    }

    #[inline(always)]
    fn has_edge(&self, e: &(Self::Vertex, Self::Vertex)) -> Result<bool, Error<Self::Vertex>> {
        // Get vertex adjacency list.
        match self.data.get(&e.0) {
            // If no vertex found return error.
            None => Err(Error::VertexNotDefined(e.0)),
            // Otherwise check second vertex.
            Some(adj) => match self.data.contains_key(&e.1) {
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
        match self.data.contains_key(&e.1) {
            // If no vertex found return error.
            false => Err(Error::VertexNotDefined(e.1)),
            // Otherwise get mutable vertex adjacency list.
            true => match self.data.get_mut(&e.0) {
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
        match self.data.contains_key(&e.1) {
            // If no vertex found return error.
            false => Err(Error::VertexNotDefined(e.1)),
            // Otherwise get mutable vertex adjacency list.
            true => match self.data.get_mut(&e.0) {
                // If no vertex exists return error.
                None => Err(Error::VertexNotDefined(e.0)),
                // Otherwise try to remove vertex from adjacency list.
                Some(adj) => match adj.remove(&e.1) {
                    // If no edge defined return error.
                    false => Err(Error::EdgeNotDefined(*e)),
                    // Otherwise
                    true => {
                        // Remove associated label if any
                        self.as_mut_edges_labels().remove_by_left(e);
                        // Return success
                        Ok(*e)
                    }
                },
            },
        }
    }

    fn clear(&mut self) {
        // Clear the data structures
        self.data.clear();
        // Clear the vertices/edges labels
        self.v_labels.clear();
        self.e_labels.clear();
    }
}
