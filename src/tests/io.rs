#[cfg(test)]
mod tests {
    mod dot {
        use crate::io::{DOT, IO};
        use tempfile::NamedTempFile;

        const DATA: &str = "graph {\n\t\"A\";\n\t\"B\";\n\t\"A\" -- \"B\";\n}\n";

        #[test]
        fn try_from() {
            assert!(DOT::try_from(DATA.to_string()).is_ok());
        }

        #[test]
        fn try_into() {
            let dot = DOT::try_from(DATA.to_string()).unwrap();
            let dot = TryInto::<String>::try_into(dot);
            assert!(dot.is_ok());
            assert_eq!(dot.unwrap(), DATA);
        }

        #[test]
        fn read() {
            std::fs::read_dir("src/tests/data")
                .expect("No such file or directory.")
                .map(|x| x.unwrap().path())
                .filter(|x| !x.extension().unwrap().eq("ignore"))
                .for_each(|x| {
                    assert!(DOT::read(&x).is_ok());
                });
        }

        #[test]
        fn write() {
            let path = NamedTempFile::new().unwrap().into_temp_path();
            let dot = DOT::try_from(DATA.to_string()).unwrap();
            assert!(dot.write(&path).is_ok());
            let dot = DOT::read(&path).unwrap();
            let dot = TryInto::<String>::try_into(dot);
            assert!(dot.is_ok());
            assert_eq!(dot.unwrap(), DATA);
        }
    }
}
