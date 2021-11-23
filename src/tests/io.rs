#[cfg(test)]
mod tests {
    use crate::graphs::AdjacencyListGraph;
    use crate::io::*;
    use pest::Parser;

    #[test]
    fn dot_parser() {
        let paths = std::fs::read_dir("src/tests/data")
            .expect("No such file or directory.")
            .map(|x| x.unwrap().path())
            .filter(|x| !x.extension().unwrap().eq("ignore"));
        for path in paths {
            let file = std::fs::read_to_string(&path)
                .unwrap_or_else(|_| panic!("Failed to read file: {:?}", &path));
            DOTParser::parse(Rule::graphs, &file)
                .unwrap_or_else(|_| panic!("Failed to parse file: {:?}", &path));
        }
    }

    #[test]
    fn read_dot() {
        let paths = std::fs::read_dir("src/tests/data")
            .expect("No such file or directory.")
            .map(|x| x.unwrap().path())
            .filter(|x| !x.extension().unwrap().eq("ignore"));
        for path in paths {
            crate::io::read_dot::<AdjacencyListGraph<usize>>(&path).unwrap();
        }
    }
}
