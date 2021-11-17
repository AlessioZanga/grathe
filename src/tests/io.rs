#[cfg(test)]
mod tests {
    use pest::Parser;
    use crate::io::*;

    #[test]
    fn dot_parser() {
        let paths = std::fs::read_dir("src/tests/data").expect("No such file or directory.");
        let paths = paths.map(|x| x.unwrap().path()).filter(|x| !x.extension().unwrap().eq("ignore"));
        for path in paths {
            let file = std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read file: {:?}", &path));
            let dot = DOTParser::parse(Rule::graphs, &file).unwrap_or_else(|_| panic!("Failed to parse file: {:?}", &path));
        }
    }
}
