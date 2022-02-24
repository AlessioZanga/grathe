/// The graph operators trait.
pub trait Operators {
    /// Complement of a graph.
    ///
    /// Let $G$ be a graph, then the complement graph $\overline{G}$ is defined as:
    ///
    /// $$ \overline{G} \thinspace \equiv \thinspace \lbrace (x, y) \thinspace | \thinspace (x, y) \in (V(G) \times V(G)) \wedge (x, y) \not \in E(G) \rbrace $$
    ///
    /// Ignores additional attributes (for now).
    ///
    fn complement(&self) -> Self;

    /// Union of two graphs.
    ///
    /// Let $G$ and $H$ be two graphs, then the union graph $G \cup H$ is defined as:
    ///
    /// $$ G \cup H \thinspace \equiv \thinspace V(G) \cup V(H) \wedge E(G) \cup E(H) $$
    ///
    /// Ignores additional attributes (for now).
    ///
    fn union(&self, other: &Self) -> Self;

    /// Intersection of two graphs.
    ///
    /// Let $G$ and $H$ be two graphs, then the intersection graph $G \cap H$ is defined as:
    ///
    /// $$ G \cap H \thinspace \equiv \thinspace V(G) \cap V(H) \wedge E(G) \cap E(H) $$
    ///
    /// Ignores additional attributes (for now).
    ///
    fn intersection(&self, other: &Self) -> Self;

    /// Symmetric difference of two graphs.
    ///
    /// Let $G$ and $H$ be two graphs, then the symmetric difference graph $G \thinspace \Delta \thinspace H$ is defined as:
    ///
    /// $$ G \thinspace \Delta \thinspace H \thinspace \equiv \thinspace E(G) \thinspace \Delta \thinspace E(H) $$
    ///
    /// It can also be expressed as:
    ///
    /// $$ G \thinspace \Delta \thinspace H \thinspace \equiv \thinspace (G - H) \cup (H - G) $$
    ///
    /// Ignores additional attributes (for now).
    ///
    fn symmetric_difference(&self, other: &Self) -> Self;

    /// Difference of two graphs.
    ///
    /// Let $G$ and $H$ be two graphs, then the difference graph $G - H$ is defined as:
    ///
    /// $$ G - H \thinspace \equiv \thinspace E(G) - E(H) $$
    ///
    /// Ignores additional attributes (for now).
    ///
    fn difference(&self, other: &Self) -> Self;
}

macro_rules! impl_operators {
    ($graph:ident) => {
        impl<T, X, Y, Z> $crate::traits::Operators for $graph<T, X, Y, Z>
        where
            T: $crate::types::VertexTrait,
            X: Default + std::fmt::Debug,
            Y: Default + std::fmt::Debug,
            Z: Default + std::fmt::Debug,
        {
            fn complement(&self) -> Self {
                Self {
                    data: self.data.complement(),
                    gattrs: Default::default(),
                    vattrs: Default::default(),
                    eattrs: Default::default(),
                }
            }

            fn union(&self, other: &Self) -> Self {
                Self {
                    data: self.data.union(&other.data),
                    gattrs: Default::default(),
                    vattrs: Default::default(),
                    eattrs: Default::default(),
                }
            }

            fn intersection(&self, other: &Self) -> Self {
                Self {
                    data: self.data.intersection(&other.data),
                    gattrs: Default::default(),
                    vattrs: Default::default(),
                    eattrs: Default::default(),
                }
            }

            fn symmetric_difference(&self, other: &Self) -> Self {
                Self {
                    data: self.data.symmetric_difference(&other.data),
                    gattrs: Default::default(),
                    vattrs: Default::default(),
                    eattrs: Default::default(),
                }
            }

            fn difference(&self, other: &Self) -> Self {
                Self {
                    data: self.data.difference(&other.data),
                    gattrs: Default::default(),
                    vattrs: Default::default(),
                    eattrs: Default::default(),
                }
            }
        }

        impl<T, X, Y, Z> std::ops::Not for &$graph<T, X, Y, Z>
        where
            T: $crate::types::VertexTrait,
            X: Default + std::fmt::Debug,
            Y: Default + std::fmt::Debug,
            Z: Default + std::fmt::Debug,
        {
            type Output = $graph<T, X, Y, Z>;

            /// Complement of a graph.
            ///
            /// Let $G$ be a graph, then the complement graph $\overline{G}$ is defined as:
            ///
            /// $$ \overline{G} \thinspace \equiv \thinspace \lbrace (x, y) \thinspace | \thinspace (x, y) \in (V(G) \times V(G)) \wedge (x, y) \not \in E(G) \rbrace $$
            ///
            /// Ignores additional attributes (for now).
            ///
            fn not(self) -> Self::Output {
                self.complement()
            }
        }

        impl<T, X, Y, Z> std::ops::BitAnd for &$graph<T, X, Y, Z>
        where
            T: $crate::types::VertexTrait,
            X: Default + std::fmt::Debug,
            Y: Default + std::fmt::Debug,
            Z: Default + std::fmt::Debug,
        {
            type Output = $graph<T, X, Y, Z>;

            /// Intersection of two graphs.
            ///
            /// Let $G$ and $H$ be two graphs, then the intersection graph $G \cap H$ is defined as:
            ///
            /// $$ G \cap H \thinspace \equiv \thinspace V(G) \cap V(H) \wedge E(G) \cap E(H) $$
            ///
            /// Ignores additional attributes (for now).
            ///
            fn bitand(self, rhs: Self) -> Self::Output {
                self.intersection(rhs)
            }
        }

        impl<T, X, Y, Z> std::ops::BitOr for &$graph<T, X, Y, Z>
        where
            T: $crate::types::VertexTrait,
            X: Default + std::fmt::Debug,
            Y: Default + std::fmt::Debug,
            Z: Default + std::fmt::Debug,
        {
            type Output = $graph<T, X, Y, Z>;

            /// Union of two graphs.
            ///
            /// Let $G$ and $H$ be two graphs, then the union graph $G \cup H$ is defined as:
            ///
            /// $$ G \cup H \thinspace \equiv \thinspace V(G) \cup V(H) \wedge E(G) \cup E(H) $$
            ///
            /// Ignores additional attributes (for now).
            ///
            fn bitor(self, rhs: Self) -> Self::Output {
                self.union(rhs)
            }
        }

        impl<T, X, Y, Z> std::ops::BitXor for &$graph<T, X, Y, Z>
        where
            T: $crate::types::VertexTrait,
            X: Default + std::fmt::Debug,
            Y: Default + std::fmt::Debug,
            Z: Default + std::fmt::Debug,
        {
            type Output = $graph<T, X, Y, Z>;

            /// Symmetric difference of two graphs.
            ///
            /// Let $G$ and $H$ be two graphs, then the symmetric difference graph $G \thinspace \Delta \thinspace H$ is defined as:
            ///
            /// $$ G \thinspace \Delta \thinspace H \thinspace \equiv \thinspace E(G) \thinspace \Delta \thinspace E(H) $$
            ///
            /// It can also be expressed as:
            ///
            /// $$ G \thinspace \Delta \thinspace H \thinspace \equiv \thinspace (G - H) \cup (H - G) $$
            ///
            /// Ignores additional attributes (for now).
            ///
            fn bitxor(self, rhs: Self) -> Self::Output {
                self.symmetric_difference(rhs)
            }
        }

        impl<T, X, Y, Z> std::ops::Sub for &$graph<T, X, Y, Z>
        where
            T: $crate::types::VertexTrait,
            X: Default + std::fmt::Debug,
            Y: Default + std::fmt::Debug,
            Z: Default + std::fmt::Debug,
        {
            type Output = $graph<T, X, Y, Z>;

            /// Difference of two graphs.
            ///
            /// Let $G$ and $H$ be two graphs, then the difference graph $G - H$ is defined as:
            ///
            /// $$ G - H \thinspace \equiv \thinspace E(G) - E(H) $$
            ///
            /// Ignores additional attributes (for now).
            ///
            fn sub(self, rhs: Self) -> Self::Output {
                self.difference(rhs)
            }
        }
    };
}

pub(crate) use impl_operators;
