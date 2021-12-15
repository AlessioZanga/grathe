use crate::errors::Error;
use crate::graphs::GraphTrait;
use crate::{E, V};
use itertools::Itertools;
use pest::error::Error as PestError;
use pest::iterators::Pair;
use pest::Parser;
use std::fmt::Write;
use std::fs::{read_to_string, write};
use std::path::Path;
use std::str::FromStr;

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
    T: GraphTrait,
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
    T: GraphTrait,
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
        T: GraphTrait,
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
                                // FIXME: graph.set_edge_label(e, to_label(value)).ok();
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
                    // Group by tuples of vertex
                    .tuple_windows()
                    // Map tuples to path
                    .map(|(x, y)| {
                        // Insert vertex if missing
                        let e = (
                            match_rules(graph, x).unwrap(),
                            match_rules(graph, y).unwrap(),
                        );
                        // Insert path
                        let e = match e {
                            (DOTValue::Vertex(x), DOTValue::Vertex(y)) => {
                                match graph.add_edge(&x, &y) {
                                    Err(e) => Err(e),
                                    Ok(_) => Ok((x, y)),
                                }
                            }
                            // FIXME: Handling subgraphs
                            _ => unreachable!(),
                        };
                        // Return path
                        match e {
                            Err(Error::EdgeAlreadyDefined(e)) | Ok(e) => e,
                            Err(_) => unreachable!(),
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
                                // FIXME: graph.set_vertex_label(&x, to_label(value)).ok();
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
                let k = to_label(j);
                // Map text representation to vertex
                let k = T::Vertex::from_str(k);
                // Assert conversion succeeded
                let k = k.ok().unwrap();
                // Handle errors
                let k = match graph.add_vertex(&k) {
                    // Match vertex identifier already defined error
                    Err(Error::VertexAlreadyDefined(k)) | Ok(k) => k,
                    // Give up on other errors
                    Err(_) => unreachable!(),
                };
                // TODO: Add vertex port
                if let Some(k) = i.next() {
                    warn!("Vertex port '{}' ignored.", k.as_str());
                }
                // Return vertex identifier
                return Ok(DOTValue::Vertex(k));
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
        let mut graph = T::new();
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
    T: GraphTrait,
{
    // Initialize empty string
    let mut dot = String::new();
    // Set the graph_type
    let mut graph_type = "graph";
    if graph.is_directed() {
        graph_type = "digraph";
    }
    // Set the edge_type
    let mut edge_type = "--";
    if graph.is_directed() {
        edge_type = "->";
    }
    // De-symmetrize edge set for undirected graphs
    // TODO: Avoid 'collect' workaround to deal with
    //       different iterator resulting types
    let mut edges = E!(graph).collect::<Vec<_>>();
    if !graph.is_directed() {
        edges = E!(graph).filter(|(x, y)| x <= y).collect();
    }
    // Open DOT string by escaping "{"
    writeln!(dot, "{} {{", graph_type)?;
    // Write vertex with label
    for x in V!(graph) {
        writeln!(dot, "\t{:?};", x);
        // FIXME: writeln!(dot, "\t{:?} [label=\"{:?}\"];", x, y);
    }
    // Write edges with label
    for (x, y) in edges {
        writeln!(dot, "\t{:?} {} {:?};", x, edge_type, y);
        // FIXME: writeln!(dot, "\t{:?} {} {:?} [label=\"{}\"];", x, edge_type, y, z);
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
    T: GraphTrait,
{
    // Read DOT file
    let string =
        read_to_string(path).unwrap_or_else(|_| panic!("Failed to read file \"{:?}\"", &path));
    // Parse dot string.
    from_dot(&string)
}

/// Write DOT file.
///
/// Write sequence of graphs to DOT file.
///
pub fn write_dot<T>(path: &Path, graph: &T) -> Result<(), std::fmt::Error>
where
    T: GraphTrait,
{
    // Export a sequence of graph into DOT string.
    let dot = to_dot(graph)?;
    // Write string to file
    write(path, dot).unwrap_or_else(|_| panic!("Failed to write file \"{:?}\"", &path));

    Ok(())
}
