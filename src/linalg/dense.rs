use crate::traits::Storage;
use ndarray::{Array, Array1, Array2, Axis};
use ndarray_linalg::{into_col, EigVals, EigValshInto, UPLO};
use num_complex::Complex;
use std::collections::HashMap;

/// Adjacency matrix of a graph.
///
/// The adjacency matrix $A$ of a graph $G$ is defined as:
///
/// $$ A_{i,j} = \begin{cases} 1, & \text{if } i \in Adj(G, j), \newline 0, & \text{Otherwise.} \end{cases} $$
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

pub fn adjacency_spectrum<T>(g: &T) -> Array1<Complex<f32>>
where
    T: Storage,
{
    adjacency_matrix(g).eigvals().unwrap()
}

/// Average adjacency matrix of a graph.
///
/// The average adjacency matrix $\bar{A}$ of a graph $G$ is defined as:
///
/// $$ \bar{A}_{i,j} = (d \cdot d^T) / \sum d $$
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
/// // Get average adjacency matrix for given graph.
/// let A_avg = linalg::average_adjacency_matrix(&g);
///
/// // Check average adjacency matrix using tolerance.
/// assert!(A_avg.abs_diff_eq(
///     &(arr2(&[
///         [4.0, 6.0, 4.0, 6.0, 6.0, 2.0],
///         [6.0, 9.0, 6.0, 9.0, 9.0, 3.0],
///         [4.0, 6.0, 4.0, 6.0, 6.0, 2.0],
///         [6.0, 9.0, 6.0, 9.0, 9.0, 3.0],
///         [6.0, 9.0, 6.0, 9.0, 9.0, 3.0],
///         [2.0, 3.0, 2.0, 3.0, 3.0, 1.0],
///     ]) / linalg::degree_vector(&g).sum()),
///     f32::EPSILON,
/// ));
/// ```
///
pub fn average_adjacency_matrix<T>(g: &T) -> Array2<f32>
where
    T: Storage,
{
    let d = into_col(degree_vector(g));

    // m := sum(d) / 2;
    // d := (d * d^t) / (2 * m)
    // d := (d * d^t) / (2 * (sum(d) / 2))
    // d := (d * d^t) / sum(d)

    d.dot(&d.t()) / d.sum()
}

/// Modularity matrix of a graph.
///
/// The modularity matrix $B$ of a graph $G$ is defined as
/// the difference between the adjacency matrix $A$ and
/// the average adjacency matrix $\bar{A}$:
///
/// $$ B = A - \bar{A} $$
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
/// // Get modularity matrix for given graph.
/// let B = linalg::modularity_matrix(&g);
///
/// // Check modularity matrix using tolerance.
/// assert!(B.abs_diff_eq(
///     &(
///         linalg::adjacency_matrix(&g) -
///         linalg::average_adjacency_matrix(&g)
///     ),
///     f32::EPSILON,
/// ));
/// ```
///
pub fn modularity_matrix<T>(g: &T) -> Array2<f32>
where
    T: Storage,
{
    let A = adjacency_matrix(g);
    let A_avg = average_adjacency_matrix(g);

    A - A_avg
}

pub fn modularity_spectrum<T>(g: &T) -> Array1<Complex<f32>>
where
    T: Storage,
{
    modularity_matrix(g).eigvals().unwrap()
}

pub fn incidence_matrix<T>(g: &T) -> Array2<f32>
where
    T: Storage,
{
    todo!()
}

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
    adjacency_matrix(g).sum_axis(Axis(1))
}

/// Degree matrix of a graph.
///
/// The degree matrix $D$ of a graph $G$ is defined as
/// the matrix with the degree vector $d$ as diagonal and zeros elsewhere:
///
/// $$ D_{i,j} = \begin{cases} d_i, & \text{if } i = j, \newline 0, & \text{Otherwise.} \end{cases} $$
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

/// Laplacian matrix of a graph.
///
/// The Laplacian matrix $L$ of a graph $G$ is defined as
/// the difference between the degree matrix $D$ and the adjacency matrix $A$:
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

pub fn laplacian_spectrum<T>(g: &T) -> Array1<f32>
where
    T: Storage,
{
    laplacian_matrix(g).eigvalsh_into(UPLO::Lower).unwrap()
}

/// Normalized adjacency matrix of a graph.
///
/// The (symmetrically) normalized adjacency matrix $\tilde{A}$ of a graph $G$ is defined as:
///
/// $$ \tilde{A}_{i,j} = \begin{cases} 0, & \text{if } i = j \wedge d_i \neq 0, \newline \frac{1}{\sqrt{d_id\[j\]}}, & \text{if } i \neq j \wedge i \in Adj(G, j), \newline 0, & \text{Otherwise.} \end{cases} $$
///
/// and can be derived from the degree matrix $D$ and the adjacency matrix $A$ as:
///
/// $$ \tilde{A} = D^{-\frac{1}{2}}AD^{-\frac{1}{2}} $$
///
/// # Examples
///
/// ```
/// use grathe::prelude::*;
/// use grathe::linalg::dense as linalg;
/// use ndarray::arr2;
///
/// # fn main() {
/// // Build an undirected graph.
/// let g = Graph::from_edges([(0, 1), (1, 2)]);
///
/// // Get normalized adjacency matrix for given graph.
/// let A_norm = linalg::normalized_adjacency_matrix(&g);
///
/// // Check normalized adjacency matrix using tolerance.
/// assert!(A_norm.abs_diff_eq(
///     &arr2(&[
///         [                 0.0, f32::sqrt(1.0 / 2.0),                  0.0],
///         [f32::sqrt(1.0 / 2.0),                  0.0, f32::sqrt(1.0 / 2.0)],
///         [                 0.0, f32::sqrt(1.0 / 2.0),                  0.0],
///     ]),
///     f32::EPSILON,
/// ));
/// # }
/// ```
///
pub fn normalized_adjacency_matrix<T>(g: &T) -> Array2<f32>
where
    T: Storage,
{
    let A = adjacency_matrix(g);
    let D = Array::from_diag(&degree_vector(g).mapv(|x| 1.0 / x.sqrt()));

    D.dot(&A).dot(&D)
}

/// Normalized Laplacian matrix of a graph.
///
/// The (symmetrically) normalized Laplacian matrix $\tilde{L}$ of a graph $G$ is defined as:
///
/// $$ \tilde{L}_{i,j} = \begin{cases} 1, & \text{if } i = j \wedge d_i \neq 0, \newline -\frac{1}{\sqrt{d_id\[j\]}}, & \text{if } i \neq j \wedge i \in Adj(G, j), \newline 0, & \text{Otherwise.} \end{cases} $$
///
/// and can be derived from the identity matrix $I$ and the normalized adjacency matrix $\tilde{A}$ as:
///
/// $$ \tilde{L} = I - \tilde{A} $$
///
/// # Examples
///
/// ```
/// use grathe::prelude::*;
/// use grathe::linalg::dense as linalg;
/// use ndarray::arr2;
///
/// # fn main() {
/// // Build an undirected graph.
/// let g = Graph::from_edges([(0, 1), (1, 2)]);
///
/// // Get normalized Laplacian matrix for given graph.
/// let L_norm = linalg::normalized_laplacian_matrix(&g);
///
/// // Check normalized Laplacian matrix using tolerance.
/// assert!(L_norm.abs_diff_eq(
///     &arr2(&[
///         [                  1.0, -f32::sqrt(1.0 / 2.0),                   0.0],
///         [-f32::sqrt(1.0 / 2.0),                   1.0, -f32::sqrt(1.0 / 2.0)],
///         [                  0.0, -f32::sqrt(1.0 / 2.0),                   1.0],
///     ]),
///     f32::EPSILON,
/// ));
/// # }
/// ```
///
pub fn normalized_laplacian_matrix<T>(g: &T) -> Array2<f32>
where
    T: Storage,
{
    let A = normalized_adjacency_matrix(g);
    let I = Array::eye(A.raw_dim()[0]);

    I - A
}

pub fn normalized_laplacian_spectrum<T>(g: &T) -> Array1<f32>
where
    T: Storage,
{
    normalized_laplacian_matrix(g).eigvalsh_into(UPLO::Lower).unwrap()
}

pub fn deformed_laplacian_matrix<T>(g: &T, r: f32) -> Array2<f32>
where
    T: Storage,
{
    let A = adjacency_matrix(g);
    let D = degree_matrix(g);
    let I = Array::eye(A.raw_dim()[0]);

    (r.powf(2.0) - 1.0) * I - r * A + D
}

pub fn deformed_laplacian_spectrum<T>(g: &T, r: f32) -> Array1<f32>
where
    T: Storage,
{
    deformed_laplacian_matrix(g, r).eigvalsh_into(UPLO::Lower).unwrap()
}
