#[cfg(test)]
mod tests {
    use crate::graphs::AdjacencyListGraph;
    use crate::io::*;
    use pest::Parser;
    use std::path::PathBuf;

    fn load_test_data() -> Vec<PathBuf> {
        std::fs::read_dir("src/tests/data")
            .expect("No such file or directory.")
            .map(|x| x.unwrap().path())
            .filter(|x| !x.extension().unwrap().eq("ignore"))
            .collect()
    }

    #[test]
    fn dot_parser() {
        for path in load_test_data() {
            let file = std::fs::read_to_string(&path)
                .unwrap_or_else(|_| panic!("Failed to read file: {:?}", &path));
            let parsed = DOTParser::parse(Rule::graphs, &file)
                .unwrap_or_else(|_| panic!("Failed to parse file: {:?}", &path));
            println!("{:?}", parsed);
        }
    }

    #[test]
    fn read_dot() {
        for path in load_test_data() {
            let parsed = crate::io::read_dot::<AdjacencyListGraph<usize>>(&path).unwrap();
            println!("{:?}", parsed);
        }
    }
}
