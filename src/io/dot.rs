use crate::errors::*;
use crate::storages::StorageTrait;
use crate::{E, V};
use itertools::Itertools;
use pest::error::Error as PestError;
use pest::iterators::Pair;
use pest::Parser;
use std::fmt::Write;
use std::fs::{read_to_string, write};
use std::path::Path;

// Workaround to print warning messages during tests
#[cfg(not(test))]
use log::warn;
#[cfg(test)]
use std::println as warn;

/// DOT parser.
///
/// Implements a [DOT language](https://graphviz.org/doc/info/lang.html) parser.
///
#[derive(Parser)]
#[grammar = "io/dot.pest"]
pub struct DOTParser;

/// Enumerator for DOT values.
enum DOTValue<T>
where
    T: StorageTrait,
{
    None,
    Path(Vec<(T::Vertex, T::Vertex)>),
    Vertex(T::Vertex),
}

/// From DOT string.
///
/// Parse DOT string into sequence of graphs.
///
pub fn from_dot<T>(string: &str) -> Result<Vec<T>, PestError<Rule>>
where
    T: StorageTrait,
{
    // Init result vector
    let mut graphs = vec![];
    // Parse the given dot file
    let pairs = DOTParser::parse(Rule::graphs, string.trim())?;
    // Match label
    fn to_label(pair: Pair<Rule>) -> &str {
        // Get label as string
        let y = pair.as_str();
        // Match label by type
        match pair.as_rule() {
            // Match text and number type
            Rule::text | Rule::number => y,
            // Match quoted text type by removing quoting
            Rule::quoted_text => y.trim_matches('"'),
            // Match everything else
            _ => unreachable!(),
        }
    }
    // Match rules recursively
    fn match_rules<T>(graph: &mut T, pair: Pair<Rule>) -> Result<DOTValue<T>, PestError<Rule>>
    where
        T: StorageTrait,
    {
        match pair.as_rule() {
            Rule::graph => {
                // Get iterator on parsed
                let mut i = pair.into_inner();
                // Parse strict attribute
                match_rules(graph, i.next().unwrap())?;
                // Parse graph type
                match_rules(graph, i.next().unwrap())?;
                // Parse graph identifier
                match_rules(graph, i.next().unwrap())?;
                // Parse graph statements
                match_rules(graph, i.next().unwrap())?;
            }
            Rule::strict_attribute => {
                // TODO: Set strict attribute
                warn!("Strict attribute '{}' ignored.", pair.as_str());
            }
            Rule::graph_type => {
                // TODO: Check if T is compatible with graph_type
                warn!("Graph type '{}' ignored.", pair.as_str());
            }
            Rule::graph_id => {
                // TODO: Set graph label
                warn!("Graph identifier '{}' ignored.", pair.as_str());
            }
            Rule::statements => {
                for statement in pair.into_inner() {
                    match_rules(graph, statement)?;
                }
            }
            Rule::path => {
                // Get iterator on parsed
                let mut i = pair.into_inner();
                // Parse path identifier
                if let DOTValue::Path(x) = match_rules(graph, i.next().unwrap())? {
                    // Add path attributes
                    if let Some(attributes) = i.next() {
                        // Iterate over attributes
                        for (key, value) in attributes
                            .into_inner()
                            // Map attributes into (key, value) pairs
                            .map(|x| x.into_inner().tuple_windows().next().unwrap())
                        {
                            // Handle special key
                            if key.as_str() == "label" {
                                // FIXME: How to handle duplicated labels? Now it is first come, first served.
                                let e = x.get(0).unwrap();
                                // FIXME: How to handle shared label across edges of path? Now it is first come, first served.
                                graph.set_edge_label(&e, to_label(value)).ok();
                            } else {
                                // TODO: Add path attributes
                                warn!(
                                    "path attribute '{}={}' ignored.",
                                    key.as_str(),
                                    value.as_str()
                                );
                            }
                        }
                    }
                }
            }
            Rule::path_id => {
                let i = pair
                    // Get iterator on parsed
                    .into_inner()
                    // Discard direction placeholder
                    .step_by(2)
                    // Group by tuples of vertices
                    .tuple_windows()
                    // Map tuples to path
                    .map(|(x, y)| {
                        // Insert vertices if missing
                        let e = (
                            match_rules(graph, x).unwrap(),
                            match_rules(graph, y).unwrap(),
                        );
                        // Insert path
                        let e = match e {
                            (DOTValue::Vertex(x), DOTValue::Vertex(y)) => graph.add_edge(&(x, y)),
                            // FIXME: Handling subgraphs
                            _ => unreachable!(),
                        };
                        // Return path
                        match e {
                            Err(Error::EdgeAlreadyDefined(x)) => x,
                            Err(_) => unreachable!(),
                            Ok(x) => x,
                        }
                    })
                    // Collect sequence of edges into path
                    .collect::<Vec<_>>();
                // Return result
                return Ok(DOTValue::Path(i));
            }
            Rule::vertex => {
                // Get iterator on parsed
                let mut i = pair.into_inner();
                // Parse vertex identifier
                if let DOTValue::Vertex(x) = match_rules(graph, i.next().unwrap())? {
                    // Add vertex attributes
                    if let Some(attributes) = i.next() {
                        // Iterate over attributes
                        for (key, value) in attributes
                            .into_inner()
                            // Map attributes into (key, value) pairs
                            .map(|x| x.into_inner().tuple_windows().next().unwrap())
                        {
                            // Handle special key
                            if key.as_str() == "label" {
                                // FIXME: How to handle duplicated labels? Here is first come, first served.
                                graph.set_vertex_label(&x, to_label(value)).ok();
                            } else {
                                // TODO: Add vertex attributes
                                warn!(
                                    "Vertex attribute '{}={}' ignored.",
                                    key.as_str(),
                                    value.as_str()
                                );
                            }
                        }
                    }
                }
            }
            Rule::vertex_id => {
                // Get iterator on parsed
                let mut i = pair.into_inner();
                // Get vertex identifier
                let j = i.next().unwrap();
                // Get underlying text representation
                let k = j.as_str();
                // Match identifier type
                let x = match j.as_rule() {
                    // Match text type
                    Rule::text => graph.add_vertex_label(k),
                    // Match quoted text type by removing quoting
                    Rule::quoted_text => graph.add_vertex_label(k.trim_matches('"')),
                    // Match number type
                    // FIXME: Can Rule::number be a float? Use it as string?
                    Rule::number => graph.reserve_vertex(&k.parse::<T::Vertex>().ok().unwrap()),
                    // Match everything else
                    _ => unreachable!(),
                };
                // Handle errors
                let x = match x {
                    // Match vertex label already defined error
                    Err(Error::VertexLabelAlreadyDefined(x)) => graph.get_vertex_id(&x).unwrap(),
                    // Match vertex identifier already defined error
                    Err(Error::VertexAlreadyDefined(x)) => x,
                    // Give up on other errors
                    Err(_) => unreachable!(),
                    // Match ok on success
                    Ok(x) => x,
                };
                // TODO: Add vertex port
                if let Some(x) = i.next() {
                    warn!("Vertex port '{}' ignored.", x.as_str());
                }
                // Return vertex identifier
                return Ok(DOTValue::Vertex(x));
            }
            _ => {
                // TODO: Handle missing rules
                warn!("Rule::{:?} '{}' ignored.", pair.as_rule(), pair.as_str());
            }
        }
        Ok(DOTValue::None)
    }
    // Match each graph in dot file
    for pair in pairs {
        // Init an empty graph
        let mut graph = T::default();
        // Match rules for this graph
        match_rules(&mut graph, pair)?;
        // Push graph to result vector
        graphs.push(graph);
    }
    // Return result vector
    Ok(graphs)
}

/// To DOT string.
///
/// Export a sequence of graphs into DOT string.
///
pub fn to_dot<T>(graph: &T) -> Result<String, std::fmt::Error>
where
    T: StorageTrait,
{
    // Initialize empty string
    let mut dot = String::new();
    // Open DOT string by escaping "{"
    // FIXME: Handle directionality
    writeln!(dot, "graph {{")?;
    // Write vertices with label
    for x in V!(graph) {
        match graph.get_vertex_label(&x) {
            Err(_) => writeln!(dot, "\t{:?};", x),
            Ok(y) => writeln!(dot, "\t{:?} [label=\"{}\"];", x, y),
        }?
    }
    // Write edges with label
    for (x, y) in E!(graph) {
        match graph.get_edge_label(&(x, y)) {
            Err(_) => writeln!(dot, "\t{:?} -- {:?};", x, y),
            Ok(z) => writeln!(dot, "\t{:?} -- {:?} [label=\"{}\"];", x, y, z),
        }?
    }
    // Close DOT string by escaping "}"
    writeln!(dot, "}}")?;
    // Return resulting DOT string
    Ok(dot)
}

/// Read DOT file.
///
/// Read DOT file into sequence of graphs.
///
pub fn read_dot<T>(path: &Path) -> Result<Vec<T>, PestError<Rule>>
where
    T: StorageTrait,
{
    // Read DOT file
    let string = read_to_string(path).expect(&format!("Failed to read file \"{:?}\"", &path));
    // Parse dot string.
    from_dot(&string)
}

/// Write DOT file.
///
/// Write sequence of graphs to DOT file.
///
pub fn write_dot<T>(path: &Path, graph: &T) -> Result<(), std::fmt::Error>
where
    T: StorageTrait,
{
    // Export a sequence of graph into DOT string.
    let dot = to_dot(graph)?;
    // Write string to file
    write(path, dot).expect(&format!("Failed to write file \"{:?}\"", &path));

    Ok(())
}
