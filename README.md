# GRATHE
![Crates.io](https://img.shields.io/crates/v/grathe)
[![build](https://github.com/AlessioZanga/grathe/actions/workflows/build.yml/badge.svg)](https://github.com/AlessioZanga/grathe/actions/workflows/build.yml)
[![codecov](https://codecov.io/gh/AlessioZanga/grathe/branch/main/graph/badge.svg?token=JTPni5P7Op)](https://codecov.io/gh/AlessioZanga/grathe)
![Crates.io](https://img.shields.io/crates/d/grathe)
![Crates.io](https://img.shields.io/crates/l/grathe)

A Rust implementation of a GRAph THEory library.

## Description

Grathe is a graph theory library that provides a coherent programming experience. Its main goal is to reduce the effort of translating theoretical aspect into practical computation.

## Installation

Include `grathe` into `Cargo.toml` as dependency.

## Usage

Here is a brief example:

```rust
use anyhow::Result; // Catch any error.
use grathe::prelude::*; // Frequently used items.

fn main() -> Result<()> {
    // Define an (undirected) graph given its edges.
    let G = Graph::from_edges([
        (0, 1), (1, 2), (3, 4)
    ]);
    
    // Iterate over the vertex set.
    for &x in V!(G) {
        assert!(G.has_vertex(&x));
    }

    // Iterate over the neighbors of `1`.
    for x in Ne!(G, &1) {
        assert!(G.has_edge(x, &1)?);
    }

    // Define a graph with labels, equivalent to Graph::<String>.
    let mut G = Graphl::new();

    // Add a vertex to the graph.
    let x = G.add_vertex("A")?;
    assert!(G.has_vertex(&x));

    // Handle errors in a Rust-compatible way.
    assert!(
        match G.add_vertex(x) {
            Err(_) => true, // Error! Vertex already defined!
            Ok(_) => false, // Ok! Vertex added successfully!
        }
    );

    // Exit with no error.
    Ok(())
}
```

## References

This repository is based on the following literature:

- [Erciyes, K. (2018). Guide to graph algorithms. Springer.](https://link.springer.com/book/10.1007/978-3-319-73235-0)

## License

Grathe is dual-licensed under the [APACHE LICENSE Version 2](https://choosealicense.com/licenses/apache-2.0/) or the [MIT LICENSE](https://choosealicense.com/licenses/mit/) to be compatible with the Rust project.
