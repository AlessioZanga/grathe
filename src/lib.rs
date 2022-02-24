#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

//! A Rust implementation of a GRAph THEory library.

/// Algorithms over graphs.
pub mod algorithms;

/// Graphs types, traits and structs.
pub mod graphs;

/// I/O operations.
pub mod io;

/// Frequently used items.
pub mod prelude;

/// Crate-wide traits.
pub mod traits;

/// Crate-wide types.
pub mod types;

/// Tools, macros, etc.
mod utils;

mod tests;

extern crate pest;
#[macro_use]
extern crate pest_derive;
