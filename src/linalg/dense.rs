use crate::traits::Storage;
use ndarray::{Array, Array1, Array2, Axis};
use ndarray_linalg::lobpcg::{lobpcg, LobpcgResult, TruncatedOrder};
use ndarray_linalg::{flatten, into_col, EigVals, EigValshInto, UPLO};
use num_complex::Complex;
use std::collections::HashMap;

/// Adjacency matrix of a graph.
///
/// The adjacency matrix $\textbf{A}$ of a graph $G$ is defined as:
///
/// $$ \textbf{A}_{i,j} = \begin{cases} 1, & \text{if } (i, j) \in \textbf{E}, \newline 0, & \text{Otherwise.} \end{cases} $$
///
/// # Examples
///
/// ```
/// use approx::*;
/// use ndarray::arr2;
/// use grathe::prelude::*;
/// use grathe::linalg::dense as linalg;
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
/// assert_relative_eq!(
///     A,
///     arr2(&[
///         [0., 1., 0., 0., 1., 0.],
///         [1., 0., 1., 0., 1., 0.],
///         [0., 1., 0., 1., 0., 0.],
///         [0., 0., 1., 0., 1., 1.],
///         [1., 1., 0., 1., 0., 0.],
///         [0., 0., 0., 1., 0., 0.],
///     ])
/// );
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
        A[(idx[x], idx[y])] = 1.;
    }

    A
}

/// Spectral decomposition of the adjacency matrix.
pub fn adjacency_spectrum<T>(g: &T) -> Array1<Complex<f32>>
where
    T: Storage,
{
    adjacency_matrix(g).eigvals().unwrap()
}

/// Average adjacency matrix of a graph.
///
/// The average adjacency matrix $\bar{\textbf{A}}$ of a graph $G$ is defined as:
///
/// $$ \bar{\textbf{A}} = (\textbf{d} \cdot \textbf{d}^T) / \sum \textbf{d} $$
///
/// with $\textbf{d}$ the degree vector.
///
/// # Examples
///
/// ```
/// use approx::*;
/// use ndarray::arr2;
/// use grathe::prelude::*;
/// use grathe::linalg::dense as linalg;
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
/// assert_relative_eq!(
///     A_avg,
///     (arr2(&[
///         [4., 6., 4., 6., 6., 2.],
///         [6., 9., 6., 9., 9., 3.],
///         [4., 6., 4., 6., 6., 2.],
///         [6., 9., 6., 9., 9., 3.],
///         [6., 9., 6., 9., 9., 3.],
///         [2., 3., 2., 3., 3., 1.],
///     ]) / linalg::degree_vector(&g).sum())
/// );
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
/// The modularity matrix $\textbf{B}$ of a graph $G$ is defined as:
///
/// $$ \textbf{B} = \textbf{A} - \bar{\textbf{A}} $$
///
/// with $\textbf{A}$ the adjacency matrix and $\bar{\textbf{A}}$ the average adjacency matrix.
///
/// # Examples
///
/// ```
/// use approx::*;
/// use grathe::prelude::*;
/// use grathe::linalg::dense as linalg;
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
/// assert_relative_eq!(
///     B,
///     (
///         linalg::adjacency_matrix(&g) -
///         linalg::average_adjacency_matrix(&g)
///     )
/// );
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

/// Spectral decomposition of the modularity matrix.
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
/// The degree vector $\textbf{d}$ of a graph $G$ is the vector of the degrees of the vertices in $G$.
///
/// # Examples
///
/// ```
/// use approx::*;
/// use ndarray::arr1;
/// use grathe::prelude::*;
/// use grathe::linalg::dense as linalg;
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
/// assert_relative_eq!(d, arr1(&[2., 3., 2., 3., 3., 1.]));
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
/// The degree matrix $\textbf{D}$ of a graph $G$ is defined as:
///
/// $$ \textbf{D}_{i,j} = \begin{cases} \textbf{d}_i, & \text{if } i = j, \newline 0, & \text{Otherwise.} \end{cases} $$
///
/// with $\textbf{d}$ the degree vector.
///
/// # Examples
///
/// ```
/// use approx::*;
/// use ndarray::arr2;
/// use grathe::prelude::*;
/// use grathe::linalg::dense as linalg;
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
/// assert_relative_eq!(
///     D,
///     arr2(&[
///         [2., 0., 0., 0., 0., 0.],
///         [0., 3., 0., 0., 0., 0.],
///         [0., 0., 2., 0., 0., 0.],
///         [0., 0., 0., 3., 0., 0.],
///         [0., 0., 0., 0., 3., 0.],
///         [0., 0., 0., 0., 0., 1.],
///     ])
/// );
///
/// // Check degree matrix has the degree vector as diagonal using tolerance.
/// assert_relative_eq!(D.diag(), linalg::degree_vector(&g));
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
/// The Laplacian matrix $\textbf{L}$ of a graph $G$ is defined as:
///
/// $$ \textbf{L} = \textbf{D} - \textbf{A} $$
///
/// with $\textbf{D}$ the degree matrix and $\textbf{A}$ the adjacency matrix.
///
/// # Examples
///
/// ```
/// use approx::*;
/// use ndarray::arr2;
/// use grathe::prelude::*;
/// use grathe::linalg::dense as linalg;
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
/// assert_relative_eq!(
///     L,
///     arr2(&[
///         [ 2., -1.,  0.,  0., -1.,  0.],
///         [-1.,  3., -1.,  0., -1.,  0.],
///         [ 0., -1.,  2., -1.,  0.,  0.],
///         [ 0.,  0., -1.,  3., -1., -1.],
///         [-1., -1.,  0., -1.,  3.,  0.],
///         [ 0.,  0.,  0., -1.,  0.,  1.],
///     ])
/// );
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

/// Spectral decomposition of the Laplacian matrix.
pub fn laplacian_spectrum<T>(g: &T) -> Array1<f32>
where
    T: Storage,
{
    laplacian_matrix(g).eigvalsh_into(UPLO::Lower).unwrap()
}

/// Normalized adjacency matrix of a graph.
///
/// The (symmetrically) normalized adjacency matrix $\tilde{\textbf{A}}$ of a graph $G$ is defined as:
///
/// $$ \tilde{\textbf{A}} = \textbf{D}^{-\frac{1}{2}}\textbf{A}\textbf{D}^{-\frac{1}{2}} $$
///
/// with $\textbf{D}$ the degree matrix and $\textbf{A}$ the adjacency matrix.
///
/// # Examples
///
/// ```
/// use approx::*;
/// use ndarray::arr2;
/// use grathe::prelude::*;
/// use grathe::linalg::dense as linalg;
///
/// // Build an undirected graph.
/// let g = Graph::from_edges([(0, 1), (1, 2)]);
///
/// // Get normalized adjacency matrix for given graph.
/// let A_norm = linalg::normalized_adjacency_matrix(&g);
///
/// // Check normalized adjacency matrix using tolerance.
/// assert_relative_eq!(
///     A_norm,
///     arr2(&[
///         [                0., f32::sqrt(1. / 2.),                 0.],
///         [f32::sqrt(1. / 2.),                 0., f32::sqrt(1. / 2.)],
///         [                0., f32::sqrt(1. / 2.),                 0.],
///     ])
/// );
/// ```
///
pub fn normalized_adjacency_matrix<T>(g: &T) -> Array2<f32>
where
    T: Storage,
{
    let A = adjacency_matrix(g);
    let D = Array::from_diag(&degree_vector(g).mapv(|x| 1. / x.sqrt()));

    D.dot(&A).dot(&D)
}

/// Normalized Laplacian matrix of a graph.
///
/// The (symmetrically) normalized Laplacian matrix $\tilde{\textbf{L}}$ of a graph $G$ is defined as:
///
/// $$ \tilde{\textbf{L}}_{i,j} = \begin{cases} 1, & \text{if } i = j \wedge \textbf{d}_i \neq 0, \newline -\frac{1}{\sqrt{\textbf{d}_i \textbf{d}_j}}, & \text{if } i \neq j \wedge (i, j) \in \textbf{E}, \newline 0, & \text{Otherwise.} \end{cases} $$
///
/// and can be derived from the identity matrix $\textbf{I}$ and the normalized adjacency matrix $\tilde{\textbf{A}}$ as:
///
/// $$ \tilde{\textbf{L}} = \textbf{I} - \tilde{\textbf{A}} $$
///
/// # Examples
///
/// ```
/// use approx::*;
/// use ndarray::arr2;
/// use grathe::prelude::*;
/// use grathe::linalg::dense as linalg;
///
/// // Build an undirected graph.
/// let g = Graph::from_edges([(0, 1), (1, 2)]);
///
/// // Get normalized Laplacian matrix for given graph.
/// let L_norm = linalg::normalized_laplacian_matrix(&g);
///
/// // Check normalized Laplacian matrix using tolerance.
/// assert_relative_eq!(
///     L_norm,
///     arr2(&[
///         [                 1., -f32::sqrt(1. / 2.),                  0.],
///         [-f32::sqrt(1. / 2.),                  1., -f32::sqrt(1. / 2.)],
///         [                 0., -f32::sqrt(1. / 2.),                  1.],
///     ])
/// );
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

/// Spectral decomposition of the normalized Laplacian matrix.
pub fn normalized_laplacian_spectrum<T>(g: &T) -> Array1<f32>
where
    T: Storage,
{
    normalized_laplacian_matrix(g).eigvalsh_into(UPLO::Lower).unwrap()
}

/// Deformed Laplacian matrix of a graph.
///
/// The deformed Laplacian matrix, also called Bethe-Hessian matrix, $\textbf{H}(r)$ of a graph $G$ is defined as:
///
/// $$ \textbf{H}(r) = (r^2 - 1) \textbf{I} - r \textbf{A} + \textbf{D} $$
///
/// with $r$ the regularization factor, $\textbf{I}$ the identity matrix, $\textbf{A}$ the adjacency matrix and $\textbf{D}$ the degree matrix.
///
/// If $r$ is equal to one, then $\textbf{H}(1)$ is equal to the Laplacian matrix $\textbf{L}$.
///
/// If $r$ is not provided, then it is defined as:
///
/// $$ r = \bigg( \sum \textbf{d} \bigg)^{-1} \bigg( \sum \textbf{d}^2 \bigg) - 1 $$
///
/// # Examples
///
/// ```
/// use approx::*;
/// use ndarray::arr2;
/// use grathe::prelude::*;
/// use grathe::linalg::dense as linalg;
///
/// // Build an undirected graph.
/// let g = Graph::from_edges([(0, 1), (1, 2)]);
///
/// // Get deformed Laplacian matrix for given graph.
/// let H = linalg::deformed_laplacian_matrix(&g, Some(1.));
///
/// // Check deformed Laplacian matrix using tolerance.
/// assert_relative_eq!(H, linalg::laplacian_matrix(&g));
/// ```
///
pub fn deformed_laplacian_matrix<T>(g: &T, r: Option<f32>) -> Array2<f32>
where
    T: Storage,
{
    let A = adjacency_matrix(g);
    let D = degree_matrix(g);
    let I = Array::eye(A.raw_dim()[0]);

    // Check if r is None.
    let r = match r {
        // If r is None, then compute the optimal regularization factor:
        // r := sum(d)^-1 * sum(d^2) - 1
        // r := sum(d^2) / sum(d) - 1
        // r := sum(D[i,i]^2) / sum(D[i,i]) - 1
        None => D.diag().mapv(|x| x.powf(2.)).sum() / D.diag().sum() - 1.,
        // Otherwise, return r.
        Some(r) => r,
    };

    (r.powf(2.) - 1.) * I - r * A + D
}

/// Spectral decomposition of the deformed Laplacian matrix.
pub fn deformed_laplacian_spectrum<T>(g: &T, r: Option<f32>) -> Array1<f32>
where
    T: Storage,
{
    deformed_laplacian_matrix(g, r).eigvalsh_into(UPLO::Lower).unwrap()
}

/// Fiedler value and vector of a graph.
///
/// Let $\textbf{L}$ be the Laplacian matrix of $G$. Then, the eigenvalue $\lambda$ and
/// eigenvector $\textbf{x}$ of $\textbf{L}$ is defined as:
///
/// $$ \textbf{L}\textbf{x} = \lambda\textbf{x} $$
///
/// The Fiedler value, also called algebraic connectivity $\alpha$, is the second smallest eigenvalue
/// $\lambda_{n-1}$ where the associated eigenvector $\textbf{x}$ is known as the Fiedler vector.
///
/// # Examples
///
/// ```
/// use approx::*;
/// use ndarray::arr1;
/// use grathe::prelude::*;
/// use grathe::linalg::dense as linalg;
///
/// // Build an undirected graph.
/// let g = Graph::from_edges([
///     (0, 1), (0, 4), (1, 2), (1, 4), (2, 3), (3, 4), (3, 5)
/// ]);
///
/// // Get Fiedler value and vector with relative tolerance of 1e-8.
/// let (a, A) = linalg::fiedler(&g, 1e-8);
///
/// // Check Fiedler value ...
/// assert_relative_eq!(a, 0.7215863905035553);
/// // ... and Fiedler vector using tolerance.
/// assert_relative_eq!(
///     A,
///     arr1(&[0.41486979, 0.30944167, 0.0692328, -0.22093352, 0.22093352, -0.79354426])
/// );
/// ```
///
pub fn fiedler<T>(g: &T, rtol: f32) -> (f32, Array1<f32>)
where
    T: Storage,
{
    // Get Laplacian matrix L.
    let L = laplacian_matrix(g);
    // Initialize n.
    let n = L.raw_dim()[0];
    // Initialize m.
    let m = n as f32;
    // Initialize X.
    let X = into_col(Array::linspace(0., m, n) - ((m - 1.) / 2.));
    // Initialize Y.
    let Y = into_col(Array::ones((n,)));
    // Initialize D.
    let D = Array::from_diag(&L.diag());
    // Compute eigendecomposition A.
    let A = lobpcg(
        |x| L.dot(&x),
        X,
        |mut x| x.assign(&D.dot(&x)),
        Some(Y),
        rtol,
        2 * n,
        TruncatedOrder::Smallest,
    );

    match A {
        LobpcgResult::Ok(a, A, _) => (a[0], flatten(A)),
        _ => unreachable!(),
    }
}
