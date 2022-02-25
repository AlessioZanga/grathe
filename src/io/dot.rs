use itertools::Itertools;
use pest::error::Error as PestError;
use pest::iterators::Pair;
use pest::Parser;
use std::collections::HashMap;
use std::vec::Vec;

// Workaround for logging during tests
#[cfg(not(test))]
use log::debug;
#[cfg(test)]
use std::println as debug;

// Enumerator for parsed values.
enum Parsed<'a> {
    Graph(Vec<Parsed<'a>>, Vec<Parsed<'a>>, HashMap<&'a str, &'a str>),
    Vertex(&'a str, HashMap<&'a str, &'a str>),
    Edge(&'a str, &'a str, HashMap<&'a str, &'a str>),
}

/// DOT parser.
///
/// Implements a [DOT language](https://graphviz.org/doc/info/lang.html) parser.
///
#[derive(Parser)]
#[grammar = "io/dot.pest"]
pub struct DOT<'a> {
    data: Vec<Parsed<'a>>,
}

impl<'a> DOT<'a> {
    fn parse_graph(pair: Pair<'a, Rule>) -> Parsed {
        let mut attributes: HashMap<&'a str, &'a str> = Default::default();
        // Get iterator on parsed.
        let mut pair = pair.into_inner();
        // Parse strict attribute.
        let strict = pair.next().unwrap();
        if matches!(strict.as_rule(), Rule::strict) {
            let strict = strict.as_str();
            if !strict.is_empty() {
                attributes.insert(strict, "true");
                debug!("Strict attribute '{}' set.", strict);
            }
        }
        // Parse graph type.
        let graph_type = pair.next().unwrap();
        if matches!(graph_type.as_rule(), Rule::graph_type) {
            // TODO: Check if T is compatible with graph_type.
            debug!("Graph type check '{}' ignored.", graph_type.as_str());
        }
        // Parse graph identifier.
        let graph_id = pair.next().unwrap();
        if matches!(graph_id.as_rule(), Rule::graph_id) {
            let graph_id = graph_id.as_str();
            if !graph_id.is_empty() {
                attributes.insert("label", graph_id);
                debug!("Graph identifier '{}' set.", graph_id);
            }
        }
        //
        let mut vertices: Vec<Parsed> = Default::default();
        let mut edges: Vec<Parsed> = Default::default();
        // Parse graph statements.
        let statements = pair.next().unwrap();
        if matches!(statements.as_rule(), Rule::statements) {
            for statement in statements.into_inner() {
                match statement.as_rule() {
                    Rule::path => {
                        edges.extend(DOT::parse_path(statement));
                    }
                    Rule::vertex => {
                        vertices.push(DOT::parse_vertex(statement));
                    }
                    _ => unreachable!(),
                };
            }
        }
        // Return parsed values.
        Parsed::Graph(vertices, edges, attributes)
    }

    fn parse_path(pair: Pair<'a, Rule>) -> Vec<Parsed> {
        // Get iterator on parsed.
        let mut pair = pair.into_inner();
        // Parse the path into a vector of edges.
        let edges = DOT::parse_path_edges(pair.next().unwrap());
        // Parse the attributes of the paths, if any.
        let attributes = match pair.next() {
            Some(attributes) => DOT::parse_path_attributes(attributes),
            None => Default::default(),
        };
        // Combine edges and attributes
        edges
            .into_iter()
            .map(|(x, y)| Parsed::Edge(x, y, attributes.clone()))
            .collect()
    }

    fn parse_path_edges(pair: Pair<'a, Rule>) -> Vec<(&'a str, &'a str)> {
        pair
            // Get iterator on parsed
            .into_inner()
            // Discard direction placeholder
            .step_by(2)
            // Group by tuples of vertex
            .tuple_windows()
            // Map tuples to path
            .map(|(x, y)| {
                match (x.as_rule(), y.as_rule()) {
                    (Rule::vertex, Rule::vertex) => (x.as_str(), y.as_str()),
                    // TODO: Handle subgraphs
                    _ => unreachable!(),
                }
            })
            // Collect sequence of edges into path
            .collect()
    }

    fn parse_path_attributes(pair: Pair<'a, Rule>) -> HashMap<&'a str, &'a str> {
        pair
            // Get iterator on parsed
            .into_inner()
            // Map attributes into (key, value) pairs
            .map(|x| x.into_inner().tuple_windows().next().unwrap())
            // Get (key, value) pairs as string
            .map(|(k, v)| (k.as_str().into(), v.as_str().into()))
            // Collect into a map of attributes
            .collect()
    }

    fn parse_vertex(pair: Pair<'a, Rule>) -> Parsed {
        // Get iterator on parsed.
        let mut pair = pair.into_inner();
        // Parse the vertex id.
        let vertex = DOT::parse_vertex_id(pair.next().unwrap());
        // Parse the attributes of the vertex, if any.
        match pair.next() {
            Some(attributes) => Parsed::Vertex(vertex, DOT::parse_vertex_attributes(attributes)),
            None => Parsed::Vertex(vertex, Default::default()),
        }
    }

    fn parse_vertex_id(pair: Pair<'a, Rule>) -> &'a str {
        // Get iterator on parsed
        let mut pair = pair.into_inner();
        // Get vertex identifier
        let vertex_id = pair.next().unwrap();
        // Get underlying text representation
        let vertex_id = match vertex_id.as_rule() {
            // Match text and number type
            Rule::text | Rule::number => vertex_id.as_str(),
            // Match quoted text type by removing quoting
            Rule::quoted_text => vertex_id.as_str().trim_matches('"'),
            // Match everything else
            _ => unreachable!(),
        };
        // TODO: Handle vertex port
        if let Some(vertex_port) = pair.next() {
            debug!("Vertex port '{}' ignored.", vertex_port.as_str());
        }
        vertex_id
    }

    fn parse_vertex_attributes(pair: Pair<'a, Rule>) -> HashMap<&'a str, &'a str> {
        pair
            // Iterate over parsed
            .into_inner()
            // Map attributes into (key, value) pairs
            .map(|x| x.into_inner().tuple_windows().next().unwrap())
            // Get (key, value) pairs as string
            .map(|(k, v)| (k.as_str().into(), v.as_str().into()))
            // Collect into a map of attributes
            .collect()
    }
}

impl<'a> TryFrom<&'a str> for DOT<'a> {
    type Error = PestError<Rule>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        // Parse the given dot file
        let pairs = DOT::parse(Rule::graphs, value.trim())?;
        // Match each graph in dot file
        let pairs = pairs.map(|pair| DOT::parse_graph(pair)).collect();

        Ok(Self { data: pairs })
    }
}
