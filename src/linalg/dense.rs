use crate::traits::Storage;
use ndarray::{Array, Array1, Array2};
use std::collections::HashMap;

/// Degree vector of a graph.
///
/// The degree vector $d$ of a graph $G$ is the vector of the degrees of the vertices in $G$.
///
/// # Examples
///
/// ```
/// use grathe::prelude::*;
/// use grathe::linalg::dense as linalg;
/// use ndarray::arr1;
///
/// // Build an undirected graph.
/// let g = Graph::from_edges([
///     (0, 1), (0, 4), (1, 2), (1, 4), (2, 3), (3, 4), (3, 5)
/// ]);
///
/// // Get degree vector for given graph.
/// let d = linalg::degree_vector(&g);
///
/// // Check degree vector is [2, 3, 2, 3, 3, 1] using tolerance.
/// assert!(d.abs_diff_eq(&arr1(&[2.0, 3.0, 2.0, 3.0, 3.0, 1.0]), f32::EPSILON));
/// ```
///
pub fn degree_vector<T>(g: &T) -> Array1<f32>
where
    T: Storage,
{
    Array::from_iter(g.vertices_iter().map(|x| g.degree_of(x) as f32))
}

/// Degree matrix of a graph.
///
/// The degree matrix $D$ of a graph $G$ is the square matrix that has the degree vector $d$ as diagonal and zeros elsewhere:
///
/// $$ D_{i,j} = \\begin{cases} d\[i\], & \\text{if } i = j, \\\\ 0, & \\text{Otherwise.} \\end{cases} $$
///
/// # Examples
///
/// ```
/// use grathe::prelude::*;
/// use grathe::linalg::dense as linalg;
/// use ndarray::arr2;
///
/// // Build an undirected graph.
/// let g = Graph::from_edges([
///     (0, 1), (0, 4), (1, 2), (1, 4), (2, 3), (3, 4), (3, 5)
/// ]);
///
/// // Get degree matrix for given graph.
/// let D = linalg::degree_matrix(&g);
///
/// // Check degree matrix has [2, 3, 2, 3, 3, 1] as diagonal using tolerance.
/// assert!(D.abs_diff_eq(
///     &arr2(&[
///         [2.0, 0.0, 0.0, 0.0, 0.0, 0.0],
///         [0.0, 3.0, 0.0, 0.0, 0.0, 0.0],
///         [0.0, 0.0, 2.0, 0.0, 0.0, 0.0],
///         [0.0, 0.0, 0.0, 3.0, 0.0, 0.0],
///         [0.0, 0.0, 0.0, 0.0, 3.0, 0.0],
///         [0.0, 0.0, 0.0, 0.0, 0.0, 1.0],
///     ]),
///     f32::EPSILON,
/// ));
///
/// // Check degree matrix has the degree vector as diagonal using tolerance.
/// assert!(D.diag().abs_diff_eq(&linalg::degree_vector(&g), f32::EPSILON));
/// ```
///
pub fn degree_matrix<T>(g: &T) -> Array2<f32>
where
    T: Storage,
{
    Array::from_diag(&degree_vector(g))
}

/// Adjacency matrix of a graph.
///
/// The adjacency matrix $A$ of a graph $G$ is the square matrix defined as:
///
/// $$ A_{i,j} = \\begin{cases} 1, & \\text{if } i \in Adj(G, j), \\\\ 0, & \\text{Otherwise.} \\end{cases} $$
///
/// # Examples
///
/// ```
/// use grathe::prelude::*;
/// use grathe::linalg::dense as linalg;
/// use ndarray::arr2;
///
/// // Build an undirected graph.
/// let g = Graph::from_edges([
///     (0, 1), (0, 4), (1, 2), (1, 4), (2, 3), (3, 4), (3, 5)
/// ]);
///
/// // Get adjacency matrix for given graph.
/// let A = linalg::adjacency_matrix(&g);
///
/// // Check adjacency matrix using tolerance.
/// assert!(A.abs_diff_eq(
///     &arr2(&[
///         [0.0, 1.0, 0.0, 0.0, 1.0, 0.0],
///         [1.0, 0.0, 1.0, 0.0, 1.0, 0.0],
///         [0.0, 1.0, 0.0, 1.0, 0.0, 0.0],
///         [0.0, 0.0, 1.0, 0.0, 1.0, 1.0],
///         [1.0, 1.0, 0.0, 1.0, 0.0, 0.0],
///         [0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
///     ]),
///     f32::EPSILON,
/// ));
/// ```
///
pub fn adjacency_matrix<T>(g: &T) -> Array2<f32>
where
    T: Storage,
{
    let A = g.order();
    let mut A = Array::zeros((A, A));

    let idx: HashMap<_, _> = g.vertices_iter().enumerate().map(|(i, x)| (x, i)).collect();

    for (x, y) in g.edges_iter() {
        A[(idx[x], idx[y])] = 1.0;
    }

    A
}

pub fn incidence_matrix<T>(g: &T) -> Array2<f32>
where
    T: Storage,
{
    todo!()
}

/// Laplacian matrix of a graph.
///
/// The Laplacian matrix $L$ of a graph $G$ is defined as the difference between the degree matrix $D$ and the adjacency matrix $A$:
///
/// $$ L = D - A $$
///
/// # Examples
///
/// ```
/// use grathe::prelude::*;
/// use grathe::linalg::dense as linalg;
/// use ndarray::arr2;
///
/// // Build an undirected graph.
/// let g = Graph::from_edges([
///     (0, 1), (0, 4), (1, 2), (1, 4), (2, 3), (3, 4), (3, 5)
/// ]);
///
/// // Get Laplacian matrix for given graph.
/// let L = linalg::laplacian_matrix(&g);
///
/// // Check Laplacian matrix using tolerance.
/// assert!(L.abs_diff_eq(
///     &arr2(&[
///         [ 2.0, -1.0,  0.0,  0.0, -1.0,  0.0],
///         [-1.0,  3.0, -1.0,  0.0, -1.0,  0.0],
///         [ 0.0, -1.0,  2.0, -1.0,  0.0,  0.0],
///         [ 0.0,  0.0, -1.0,  3.0, -1.0, -1.0],
///         [-1.0, -1.0,  0.0, -1.0,  3.0,  0.0],
///         [ 0.0,  0.0,  0.0, -1.0,  0.0,  1.0],
///     ]),
///     f32::EPSILON,
/// ));
/// ```
///
pub fn laplacian_matrix<T>(g: &T) -> Array2<f32>
where
    T: Storage,
{
    let D = degree_matrix(g);
    let A = adjacency_matrix(g);

    D - A
}

/// Normalized Laplacian matrix of a graph.
///
/// The (symmetrically) normalized Laplacian matrix $\tilde{L}$ of a graph $G$ is defined as:
///
/// $$ \\tilde{L}_{i,j} = \\begin{cases} 1, & \\text{if } i = j \wedge d\[i\] \neq 0, \\\\ -\frac{1}{\sqrt{d\[i\]d\[j\]}}, & \\text{if } i \neq j \wedge i \in Adj(G, j), \\\\ 0, & \\text{Otherwise.} \\end{cases} $$
///
/// and can be derived from the degree matrix $D$ and the laplacian matrix $L$ as:
///
/// $$ \\tilde{L} = D^{-\frac{1}{2}}LD^{-\frac{1}{2}} $$
///
/// # Examples
///
/// ```
/// use grathe::prelude::*;
/// use grathe::linalg::dense as linalg;
/// use ndarray::arr2;
///
/// // Build an undirected graph.
/// let g = Graph::from_edges([
///     (0, 1), (0, 4), (1, 2), (1, 4), (2, 3), (3, 4), (3, 5)
/// ]);
///
/// // Get normalized Laplacian matrix for given graph.
/// let L_norm = linalg::normalized_laplacian_matrix(&g);
///
/// // Check normalized Laplacian matrix using tolerance.
/// assert!(L_norm.abs_diff_eq(
///     &arr2(&[
///         [ 2.0, -1.0,  0.0,  0.0, -1.0,  0.0],
///         [-1.0,  3.0, -1.0,  0.0, -1.0,  0.0],
///         [ 0.0, -1.0,  2.0, -1.0,  0.0,  0.0],
///         [ 0.0,  0.0, -1.0,  3.0, -1.0, -1.0],
///         [-1.0, -1.0,  0.0, -1.0,  3.0,  0.0],
///         [ 0.0,  0.0,  0.0, -1.0,  0.0,  1.0],
///     ]),
///     f32::EPSILON,
/// ));
/// ```
///
pub fn normalized_laplacian_matrix<T>(g: &T) -> Array2<f32>
where
    T: Storage,
{
    let L = laplacian_matrix(g);
    let D = degree_matrix(g).mapv(|x| x.powf(-1.0 / 2.0));

    D.dot(&L).dot(&D)
}

pub fn deformed_laplacian_matrix<T>(g: &T) -> Array2<f32>
where
    T: Storage,
{
    todo!()
}
