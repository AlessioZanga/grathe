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
    let file = std::fs::read_to_string(path.to_str().unwrap())
        .unwrap_or_else(|_| panic!("Failed to read file: {:?}", &path));
    // Parse the given dot file
    let dot = DOTParser::parse(Rule::graphs, &file)?.next().unwrap();
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
                //TODO: Add edge with attributes
            }
            Rule::edge_id => {
                //TODO: Add edge or multiple edges
            }
            Rule::vertex => {
                // Get iterator on parsed
                let mut i = pair.into_inner();
                // Parse node identifier
                match_rules(graph, i.next().unwrap())?;
                //TODO: Add node attributes
            }
            Rule::vertex_id => {
                // Get matched string
                let x = pair.as_str();
                // Check if token can be parsed into identifier
                match x.parse::<T::Vertex>() {
                    Err(_) => graph.add_vertex_label(x).ok(),
                    Ok(y) => graph.add_vertex(&y).ok(),
                };
            }
            _ => {
                //TODO: Handle missing rules
            }
        }
        Ok(())
    }
    // Match each graph in dot file
    for pair in dot.into_inner() {
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
