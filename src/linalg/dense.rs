use crate::traits::Storage;
use ndarray::{Array, Array1, Array2};
use std::collections::HashMap;

pub fn degree_vector<T>(g: &T) -> Array1<f32>
where
    T: Storage,
{
    Array::from_iter(g.vertices_iter().map(|x| g.degree_of(x) as f32))
}

pub fn degree_matrix<T>(g: &T) -> Array2<f32>
where
    T: Storage,
{
    Array::from_diag(&degree_vector(g))
}

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

pub fn laplacian_matrix<T>(g: &T) -> Array2<f32>
where
    T: Storage,
{
    let D = degree_matrix(g);
    let A = adjacency_matrix(g);

    D - A
}

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
