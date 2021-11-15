#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

//! A Rust implementation of a GRAph THEory library.

pub mod errors;
pub mod graphs;
pub mod types;

mod tests;

extern crate nalgebra as na;
extern crate nalgebra_sparse as nasparse;
