use pest_consume::{match_nodes, Error};
type Node<'i> = pest_consume::Node<'i, Rule, ()>;
type Result<T> = std::result::Result<T, Error<Rule>>;

/// DOT parser.
///
/// Implements a [DOT language](https://graphviz.org/doc/info/lang.html) parser.
///
#[derive(Parser)]
#[grammar = "io/dot.pest"]
pub struct DOTParser;

#[pest_consume::parser]
impl DOTParser {
    fn ID(input: Node) -> Result<String> {
        Ok(String::from(input.as_str()))
    }

    fn pair(input: Node) -> Result<Vec<String>> {
        Ok(match_nodes!(input.into_children();
            [ID(values)..] => values.collect(),
        ))
    }

    fn attributes(input: Node) -> Result<Vec<Vec<String>>> {
        Ok(match_nodes!(input.into_children();
            [pair(values)..] => values.collect(),
        ))
    }

    fn EOI(_input: Node) -> Result<()> {
        Ok(())
    }
}
