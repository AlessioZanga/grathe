#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

//! A Rust implementation of a GRAph THEory library.

/// Algorithms over graphs.
pub mod algorithms;

/// Error enums declaration.
pub mod errors;

/// Graphs types, traits and structs.
pub mod graphs;

/// I/O operations.
pub mod io;

/// Frequently used items.
pub mod prelude;

/// Backend graph storage implementations.
pub mod storages;

/// Libray-wide types and type-traits.
pub mod types;

pub mod traits;

/// Tools, macros, etc.
pub mod utils;

mod tests;

extern crate pest;
#[macro_use]
extern crate pest_derive;
