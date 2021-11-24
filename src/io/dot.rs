use crate::errors::*;
use crate::graphs::GraphTrait;
use itertools::Itertools;
use pest::iterators::Pair;
use pest::Parser;
use std::path::Path;

/// DOT parser.
///
/// Implements a [DOT language](https://graphviz.org/doc/info/lang.html) parser.
///
#[derive(Parser)]
#[grammar = "io/dot.pest"]
pub struct DOTParser;

/// Enumerator for DOT values.
enum DOTValue<T> {
    None,
    Vertex(T),
    Edge((T, T)),
}

/// Read DOT file.
///
/// Read DOT file into sequence of graphs.
///
pub fn read_dot<T>(path: &Path) -> Result<Vec<T>, pest::error::Error<Rule>>
where
    T: GraphTrait,
{
    // Init result vector
    let mut graphs = vec![];
    // Read DOT file
    let file = std::fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Failed to read file: {:?}", &path));
    // Parse the given dot file
    let pairs = DOTParser::parse(Rule::graphs, &file.trim())?;
    // Match rules recursively
    fn match_rules<T>(
        graph: &mut T,
        pair: Pair<Rule>,
    ) -> Result<DOTValue<T::Vertex>, pest::error::Error<Rule>>
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
                // Parse graph id
                match_rules(graph, i.next().unwrap())?;
                // Parse graph statements
                match_rules(graph, i.next().unwrap())?;
            }
            Rule::strict_attribute => {
                // TODO: Set strict attribute
            }
            Rule::graph_type => {
                // TODO: Check if T is compatible with graph_type
            }
            Rule::graph_id => {
                // TODO: Set graph label
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
                // TODO: Add edge or multiple edges
            }
            Rule::vertex => {
                // Get iterator on parsed
                let mut i = pair.into_inner();
                // Parse vertex identifier
                match_rules(graph, i.next().unwrap())?;
                // TODO: Add vertex attributes
            }
            Rule::vertex_id => {
                // Get vertex id
                let i = pair.into_inner().next().unwrap();
                // Get underlying text representation
                let j = i.as_str();
                // Match ids types
                let x = match i.as_rule() {
                    Rule::text => graph.add_vertex_label(j),
                    Rule::quoted_text => graph.add_vertex_label(j.trim_matches('"')),
                    // FIXME: Can Rule::number be a float? Use it as string?
                    Rule::number => match j.parse::<T::Vertex>() {
                        Err(_) => graph.add_vertex_label(j),
                        Ok(j) => graph.add_vertex(&j),
                    },
                    _ => unreachable!(),
                };
                // Handle errors
                let x = match x {
                    // Catch vertex label already defined error
                    Err(Error::VertexLabelAlreadyDefined(x)) => graph.get_vertex_id(&x).unwrap(),
                    // Catch vertex identifier already defined error
                    Err(Error::VertexAlreadyDefined(x)) => x,
                    // Give up on other errors
                    Err(_) => unreachable!(),
                    // Return on success
                    Ok(j) => j,
                };
                // TODO: Add vertex port
                return Ok(DOTValue::Vertex(x));
            }
            _ => {
                // TODO: Handle missing rules
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
