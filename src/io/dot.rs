use super::IO;
use itertools::Itertools;
use pest::error::Error as ParserError;
use pest::iterators::Pair;
use pest::Parser;
use std::collections::HashMap;
use std::io::{Error as IOError, ErrorKind as IOErrorKind};
use std::path::Path;
use std::vec::Vec;

// Workaround for logging during tests
#[cfg(not(test))]
use log::debug;
#[cfg(test)]
use std::println as debug;

// Enumerator for parsed values.
enum Parsed {
    Graph(Vec<Parsed>, Vec<Parsed>, HashMap<String, String>),
    Vertex(String, HashMap<String, String>),
    Edge(String, String, String, HashMap<String, String>),
}

/// DOT parser.
///
/// Implements a [DOT language](https://graphviz.org/doc/info/lang.html) parser.
///
#[derive(Parser)]
#[grammar = "io/dot.pest"]
pub struct DOT {
    data: Vec<Parsed>,
}

impl DOT {
    fn parse_graph(pair: Pair<Rule>) -> Parsed {
        // Check rule.
        assert!(matches!(pair.as_rule(), Rule::graph));
        // Initialize graph attributes.
        let mut attributes: HashMap<String, String> = Default::default();
        // Get iterator on parsed.
        let mut pair = pair.into_inner();
        // Parse strict attribute.
        let strict = pair.next().unwrap();
        if matches!(strict.as_rule(), Rule::strict) {
            let strict = strict.as_str();
            if !strict.is_empty() {
                attributes.insert("strict".into(), strict.into());
            }
        }
        // Parse graph type.
        let graph_type = pair.next().unwrap();
        if matches!(graph_type.as_rule(), Rule::graph_type) {
            let graph_type = graph_type.as_str();
            attributes.insert("graph_type".into(), graph_type.into());
        }
        // Parse graph identifier.
        let graph_id = pair.next().unwrap();
        if matches!(graph_id.as_rule(), Rule::graph_id) {
            let graph_id = graph_id.as_str();
            if !graph_id.is_empty() {
                attributes.insert("graph_id".into(), graph_id.into());
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
                    rule => debug!("Ignore 'Rule::{:?}'", rule),
                };
            }
        }
        // Return parsed values.
        Parsed::Graph(vertices, edges, attributes)
    }

    fn parse_attributes(pair: Pair<Rule>) -> HashMap<String, String> {
        // Get iterator on parsed.
        pair.into_inner()
            // Map attributes into (key, value) pairs
            .map(|x| x.into_inner().tuple_windows().next().unwrap())
            // Get (key, value) pairs as string
            .map(|(k, v)| (k.as_str().into(), v.as_str().into()))
            // Collect into a map of attributes
            .collect()
    }

    fn parse_path(pair: Pair<Rule>) -> Vec<Parsed> {
        // Check rule.
        assert!(matches!(pair.as_rule(), Rule::path));
        // Get iterator on parsed.
        let mut pair = pair.into_inner();
        // Parse the path into a vector of edges.
        let edges = DOT::parse_path_edges(pair.next().unwrap());
        // Parse the attributes of the paths, if any.
        let attributes = match pair.next() {
            Some(attributes) => DOT::parse_attributes(attributes),
            None => Default::default(),
        };
        // Combine edges and attributes
        edges
            .into_iter()
            .map(|(x, direction, y)| Parsed::Edge(x, direction, y, attributes.clone()))
            .collect()
    }

    fn parse_path_edges(pair: Pair<Rule>) -> Vec<(String, String, String)> {
        // Check rule.
        assert!(matches!(pair.as_rule(), Rule::path_id));
        // Get iterator on parsed.
        pair.into_inner()
            // Group by tuples of vertex
            .tuple_windows()
            // Skip useless (path_direction, vertex_id, path_direction) intermediate results.
            .step_by(2)
            // Map tuples to path
            .filter_map(|(x, direction, y)| {
                match (x.as_rule(), direction.as_rule(), y.as_rule()) {
                    (Rule::vertex_id, Rule::path_direction, Rule::vertex_id) => Some((
                        DOT::parse_vertex_id(x),
                        direction.as_str().into(),
                        DOT::parse_vertex_id(y),
                    )),
                    // TODO: Handle subgraphs
                    _ => todo!(),
                }
            })
            // Collect sequence of edges into path
            .collect()
    }

    fn parse_vertex(pair: Pair<Rule>) -> Parsed {
        // Check rule.
        assert!(matches!(pair.as_rule(), Rule::vertex));
        // Get iterator on parsed.
        let mut pair = pair.into_inner();
        // Parse the vertex id.
        let vertex = DOT::parse_vertex_id(pair.next().unwrap());
        // Parse the attributes of the vertex, if any.
        match pair.next() {
            Some(attributes) => Parsed::Vertex(vertex, DOT::parse_attributes(attributes)),
            None => Parsed::Vertex(vertex, Default::default()),
        }
    }

    fn parse_vertex_id(pair: Pair<Rule>) -> String {
        // Check rule.
        assert!(matches!(pair.as_rule(), Rule::vertex_id));
        // Get iterator on parsed.
        let mut pair = pair.into_inner();
        // Get vertex identifier
        let vertex_id = pair.next().unwrap();
        // Get underlying text representation
        let vertex_id = match vertex_id.as_rule() {
            // Match text and number type
            Rule::text | Rule::number => vertex_id.as_str().into(),
            // Match quoted text type by removing quoting
            Rule::quoted_text => vertex_id.as_str().trim_matches('"').into(),
            // Match everything else
            _ => unreachable!(),
        };
        // TODO: Handle vertex port
        if let Some(vertex_port) = pair.next() {
            debug!("Vertex port '{}' ignored.", vertex_port.as_str());
        }
        vertex_id
    }
}

impl TryFrom<String> for DOT {
    type Error = ParserError<Rule>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // Parse the given dot file
        let pairs = DOT::parse(Rule::graphs, value.trim())?;
        // Match each graph in dot file
        let pairs = pairs.map(|pair| DOT::parse_graph(pair)).collect();

        Ok(Self { data: pairs })
    }
}

impl TryInto<String> for DOT {
    type Error = std::fmt::Error;

    fn try_into(self) -> Result<String, Self::Error> {
        // Use Write trait.
        use std::fmt::Write;
        // Define macro to write attributes.
        macro_rules! write_attributes {
            ($string:ident, $attributes:ident) => {
                if !$attributes.is_empty() {
                    // Write attributes.
                    write!(
                        $string,
                        " [{}]",
                        $attributes
                            // Format attributes as key-value pairs.
                            .into_iter()
                            .map(|(k, v)| format!("{:?}={:?}", k, v))
                            .join(", ")
                    )?;
                }
            };
        }
        // Initialize output result.
        let mut string: String = Default::default();
        // Iterate over graphs.
        for graph in self.data {
            // Bind enum variant type.
            match graph {
                Parsed::Graph(vertices, edges, attributes) => {
                    // Write graph strict attribute, if any.
                    if attributes.contains_key("strict") {
                        write!(string, "strict")?;
                    }
                    // Write graph type.
                    write!(string, "{}", attributes["graph_type"])?;
                    // Write graph identifier, if any.
                    if attributes.contains_key("graph_id") {
                        write!(string, " {:?}", attributes["graph_id"])?;
                    }
                    // Begin graph statement.
                    writeln!(string, " {{")?;
                    // Iterate over vertices.
                    for vertex in vertices {
                        // Bind enum variant type.
                        match vertex {
                            Parsed::Vertex(x, attributes) => {
                                // Begin vertex statement.
                                write!(string, "\t{:?}", x)?;
                                // If there are attributes to write.
                                write_attributes!(string, attributes);
                                // End vertex statement.
                                writeln!(string, ";")?;
                            }
                            _ => unreachable!(),
                        };
                    }
                    // Iterate over edges.
                    for edge in edges {
                        // Bind enum variant type.
                        match edge {
                            Parsed::Edge(x, direction, y, attributes) => {
                                // Begin edge statement.
                                write!(string, "\t{:?} {} {:?}", x, direction, y)?;
                                // If there are attributes to write.
                                write_attributes!(string, attributes);
                                // End edge statement.
                                writeln!(string, ";")?;
                            }
                            _ => unreachable!(),
                        }
                    }
                    // End graph statement.
                    writeln!(string, "}}")?;
                }
                _ => unreachable!(),
            };
        }

        Ok(string)
    }
}

impl IO for DOT {
    fn read(path: &Path) -> Result<Self, IOError> {
        let string = std::fs::read_to_string(path)?;
        Self::try_from(string).map_err(|_| IOError::new(IOErrorKind::InvalidData, "invalid data"))
    }

    fn write(self, path: &Path) -> Result<(), IOError> {
        match TryInto::<String>::try_into(self) {
            Ok(string) => std::fs::write(path, string),
            Err(_) => Err(IOError::new(IOErrorKind::InvalidData, "invalid data")),
        }
    }
}
