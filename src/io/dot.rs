use crate::errors::*;
use crate::graphs::GraphTrait;
use itertools::Itertools;
use pest::iterators::Pair;
use pest::Parser;
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
    T: GraphTrait,
{
    None,
    // Edge((T::Vertex, T::Vertex)),
    Vertex(T::Vertex),
}

/// From DOT string.
///
/// Parse DOT string into sequence of graphs.
///
pub fn from_dot<T>(string: &str) -> Result<Vec<T>, pest::error::Error<Rule>>
where
    T: GraphTrait,
{
    // Init result vector
    let mut graphs = vec![];
    // Parse the given dot file
    let pairs = DOTParser::parse(Rule::graphs, string.trim())?;
    // Match rules recursively
    fn match_rules<T>(
        graph: &mut T,
        pair: Pair<Rule>,
    ) -> Result<DOTValue<T>, pest::error::Error<Rule>>
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
            Rule::edge => {
                // Get iterator on parsed
                let mut i = pair.into_inner();
                // Parse edge identifier
                match_rules(graph, i.next().unwrap())?;
                // TODO: Add edge attributes
                if let Some(x) = i.next() {
                    warn!("Edge attributes '{}' ignored.", x.as_str());
                }
            }
            Rule::edge_id => {
                // Get iterator on parsed
                let i = pair
                    .into_inner()
                    // Discard direction placeholder
                    .step_by(2)
                    // Group by tuples of vertices
                    .tuple_windows();
                // Insert edge
                for (x, y) in i {
                    // Insert vertices if missing
                    let e = (match_rules(graph, x)?, match_rules(graph, y)?);
                    // Insert edge
                    match e {
                        (DOTValue::Vertex(x), DOTValue::Vertex(y)) => graph.add_edge(&(x, y)).ok(),
                        // FIXME: Handling subgraphs
                        _ => unreachable!(),
                    };
                }
            }
            Rule::vertex => {
                // Get iterator on parsed
                let mut i = pair.into_inner();
                // Parse vertex identifier
                match_rules(graph, i.next().unwrap())?;
                // TODO: Add vertex attributes
                if let Some(x) = i.next() {
                    warn!("Vertex attributes '{}' ignored.", x.as_str());
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
                    Rule::number => graph.add_vertex(&k.parse::<T::Vertex>().ok().unwrap()),
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

/// Read DOT file.
///
/// Read DOT file into sequence of graphs.
///
pub fn read_dot<T>(path: &Path) -> Result<Vec<T>, pest::error::Error<Rule>>
where
    T: GraphTrait,
{
    // Read DOT file
    let string = std::fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Failed to read file: {:?}", &path));
    // Parse dot string.
    from_dot(&string)
}
