use crate::errors::Error;
use crate::traits::Base;
use itertools::Itertools;
use pest::error::Error as PestError;
use pest::iterators::Pair;
use pest::Parser;
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
    T: Base,
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
    T: Base,
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
        T: Base,
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
                if let DOTValue::Path(p) = match_rules(graph, i.next().unwrap())? {
                    // Add path attributes
                    if let Some(attributes) = i.next() {
                        // Iterate over attributes
                        for (k, v) in attributes
                            .into_inner()
                            // Map attributes into (key, value) pairs
                            .map(|x| x.into_inner().tuple_windows().next().unwrap())
                            // Get (key, value) pairs as string
                            .map(|(k, v)| (k.as_str(), v.as_str()))
                        {
                            // Iterate over each edge in the path.
                            for (x, y) in &p {
                                // Set the same attributes for each edge in the path.
                                // graph.set_edge_attr(x, y, k, v.to_string()).unwrap();
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
                        let e = (match_rules(graph, x).unwrap(), match_rules(graph, y).unwrap());
                        // Insert path
                        let e = match e {
                            (DOTValue::Vertex(x), DOTValue::Vertex(y)) => match graph.add_edge(&x, &y) {
                                Err(e) => Err(e),
                                Ok(_) => Ok((x, y)),
                            },
                            // FIXME: Handling subgraphs
                            _ => unreachable!(),
                        };
                        // Return path
                        match e {
                            Err(Error::EdgeAlreadyDefined(x, y)) | Ok((x, y)) => (x, y),
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
                        for (k, v) in attributes
                            .into_inner()
                            // Map attributes into (key, value) pairs
                            .map(|x| x.into_inner().tuple_windows().next().unwrap())
                            // Get (key, value) pairs as string
                            .map(|(k, v)| (k.as_str(), v.as_str()))
                        {
                            // graph.set_vertex_attr(&x, k, v.to_string()).unwrap();
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
