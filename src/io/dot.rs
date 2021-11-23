use crate::graphs::GraphTrait;
use pest::error::Error;
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

/// Read DOT file.
///
/// Read DOT file into sequence of graphs.
///
pub fn read_dot<T>(path: &Path) -> Result<Vec<T>, Error<Rule>>
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
    fn match_rules<T>(graph: &mut T, pair: Pair<Rule>) -> Result<(), Error<Rule>>
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
                // TODO: Add edge with attributes
            }
            Rule::edge_id => {
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
                let id = pair.into_inner().next().unwrap();
                // Get underlying text representation
                let txt = id.as_str();
                // Match ids types
                match id.as_rule() {
                    Rule::text => graph.add_vertex_label(txt).ok(),
                    Rule::quoted_text => graph.add_vertex_label(txt.trim_matches('"')).ok(),
                    // FIXME: Can Rule::number be a float? Use it as string?
                    Rule::number => match txt.parse::<T::Vertex>() {
                        Err(_) => graph.add_vertex_label(txt).ok(),
                        Ok(y) => graph.add_vertex(&y).ok(),
                    },
                    _ => unreachable!(),
                };
                // TODO: Add vertex port
            }
            _ => {
                // TODO: Handle missing rules
            }
        }
        Ok(())
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
