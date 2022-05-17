use std::{
    collections::{BTreeSet, HashSet, VecDeque},
    iter::repeat,
};

use crate::{
    traits::{PartiallyMixed, Storage},
    types::{directions, Mark as M},
    Adj,
};

/// Find all possible d-separation vertices.
///
/// Find all possible d-separation vertices [^1] given a source vertex and an (optional) target vertex.
///
/// [^1]: [Colombo, D., Maathuis, M. H., Kalisch, M., & Richardson, T. S. (2012). Learning high-dimensional directed acyclic graphs with latent and selection variables.](https://scholar.google.com/scholar?q=Learning+high-dimensional+directed+acyclic+graphs+with+latent+and+selection+variables)
///
pub struct PossibleDSep<'a, G, D>
where
    G: Storage<Direction = D>,
{
    /// Given graph reference.
    g: &'a G,
    /// Given source vertex.
    x: &'a G::Vertex,
    /// Optional target vertex.
    y: Option<&'a G::Vertex>,
    /// To-be-visited stack.
    queue: VecDeque<(&'a G::Vertex, &'a G::Vertex)>,
    /// Already visited set.
    visited: HashSet<(&'a G::Vertex, &'a G::Vertex)>,
    /// Vector of found possible-d-sep vertices.
    pub possible_d_sep: BTreeSet<&'a G::Vertex>,
}

impl<'a, G, D> PossibleDSep<'a, G, D>
where
    G: Storage<Direction = D>,
{
    /// Build a Possible-D-Sep search structure.
    pub fn new(g: &'a G, x: &'a G::Vertex, y: Option<&'a G::Vertex>) -> Self {
        // Assert source vertex is in graph.
        assert!(g.has_vertex(x) && (y.is_none() || g.has_vertex(y.clone().unwrap())));

        Self {
            // Set target graph.
            g,
            // Set source vertex.
            x,
            // Set target vertex.
            y,
            // Initialize the to-be-visited queue.
            queue: Default::default(),
            // Initialize the already visited set.
            visited: Default::default(),
            // Initialize vector of found circle paths.
            possible_d_sep: Default::default(),
        }
    }
}

impl<'a, G> PossibleDSep<'a, G, directions::PartiallyMixed>
where
    G: Storage<Direction = directions::PartiallyMixed> + PartiallyMixed,
{
    /// Definition 3.3 [^1].
    ///
    /// [^1]: [Colombo, D., Maathuis, M. H., Kalisch, M., & Richardson, T. S. (2012). Learning high-dimensional directed acyclic graphs with latent and selection variables.](https://scholar.google.com/scholar?q=Learning+high-dimensional+directed+acyclic+graphs+with+latent+and+selection+variables)
    ///
    fn visit(&mut self, x: &'a G::Vertex) {
        // Initialize the visiting queue.
        self.queue.extend(repeat(x).zip(Adj!(self.g, x)));
        // Get the X *-* Y edge.
        while let Some((x, y)) = self.queue.pop_front() {
            // For all Y *-* Z ...
            for z in Adj!(self.g, y) {
                // ... where X != Z, and ...
                if x != z &&
                    // ... if (Z, Y) has not already been visited, and ...
                    !self.visited.contains(&(y, z)) && (
                        // ... (X, Y, Z) forms a triangle, or ...
                        !(self.g.has_mark(x, z, M::None) && self.g.has_mark(z, x, M::None)) ||
                        // ... a collider on Y as (X *-> Y <-* Z), then ...
                        (
                            matches!(self.g.get_mark(x, y), Some(M::CircHead | M::HeadHead | M::TailHead)) &&
                            matches!(self.g.get_mark(z, y), Some(M::CircHead | M::HeadHead | M::TailHead))
                        )
                    )
                {
                    // ... add (Y, Z) to the queue ...
                    self.queue.push_back((y, z));
                    // ... set (Y, Z) as visited ...
                    self.visited.insert((y, z));
                    // ... add Z to the PDS set.
                    self.possible_d_sep.insert(z);
                }
            }
        }

        // Remove the source vertex from the PDS set, if any.
        self.possible_d_sep.remove(x);
    }

    /// Definition 3.4 [^1].
    ///  
    /// [^1]: [Colombo, D., Maathuis, M. H., Kalisch, M., & Richardson, T. S. (2012). Learning high-dimensional directed acyclic graphs with latent and selection variables.](https://scholar.google.com/scholar?q=Learning+high-dimensional+directed+acyclic+graphs+with+latent+and+selection+variables)
    ///
    fn visit_path(&mut self, _x: &'a G::Vertex, _y: &'a G::Vertex) {
        // FIXME:
        todo!()
    }

    /// Execute the procedure.
    ///
    /// Execute the procedure and store the results for later queries.
    ///
    /// TODO: Replace with FnMut once stabilized.
    ///
    pub fn call_mut(&mut self) -> &Self {
        // Get target vertex.
        match self.y {
            // Visit given graph with known target vertex.
            Some(y) => self.visit_path(self.x, y),
            // Visit given graph with unknown target vertex.
            None => self.visit(self.x),
        };

        self
    }
}

impl<'a, G, D> From<(&'a G, &'a G::Vertex)> for PossibleDSep<'a, G, D>
where
    G: Storage<Direction = D>,
{
    fn from((g, x): (&'a G, &'a G::Vertex)) -> Self {
        Self::new(g, x, None)
    }
}

impl<'a, G, D> From<(&'a G, &'a G::Vertex, &'a G::Vertex)> for PossibleDSep<'a, G, D>
where
    G: Storage<Direction = D>,
{
    fn from((g, x, y): (&'a G, &'a G::Vertex, &'a G::Vertex)) -> Self {
        Self::new(g, x, Some(y))
    }
}

impl<'a, G, D> Into<BTreeSet<&'a G::Vertex>> for PossibleDSep<'a, G, D>
where
    G: Storage<Direction = D>,
{
    fn into(self) -> BTreeSet<&'a G::Vertex> {
        self.possible_d_sep
    }
}
