#[cfg(tests)]
mod tests {
    #[allow(unused_imports)]
    use doc_comment::doctest;
    #[cfg(doctest)]
    doctest!("../../README.md"); // Test examples in README file.
}
