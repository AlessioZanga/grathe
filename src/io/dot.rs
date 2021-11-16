/// DOT parser.
/// 
/// Implements a [DOT language](https://graphviz.org/doc/info/lang.html) parser.
/// 
#[derive(Parser)]
#[grammar = "io/dot.pest"]
pub struct DOTParser;
