#[allow(unused_imports)]
use doc_comment::doctest;

// Test examples in README file.
#[cfg(doctest)]
doctest!("../../README.md");
