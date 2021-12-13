#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

//! A Rust implementation of a GRAph THEory library.

/// Graph directions trait, such as UndirectedTrait.
pub mod directions;

/// Error enums declaration.
pub mod errors;

/// Graphs types, traits and structs.
// pub mod graphs;

/// I/O operations.
// pub mod io;

/// Frequently used items.
pub mod prelude;

/// Backend graph storage implementations.
pub mod storages;

/// Libray-wide types and type-traits.
pub mod types;

/// Tools, macros, etc.
pub mod utils;

mod tests;

extern crate nalgebra as na;
extern crate nalgebra_sparse as nasparse;
extern crate pest;
#[macro_use]
extern crate pest_derive;
