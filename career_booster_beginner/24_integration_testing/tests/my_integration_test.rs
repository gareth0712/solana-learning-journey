// By default, cargo test looks for integration tests in the `tests/` directory of your project. Every Rust source file directly under tests/ is compiled as a separate crate, and any function annotated with #[test] will be run as an integration test.
// Because the integration tests (files directly under tests/, not recursively) are compiled as separate crates, you can only access the public interface of the crate under the test.
// integration test can only access items declared `pub`
// integrations
mod common;

#[test]
fn test_add() {
  assert_eq!(integration_testing::add(3, 2), 5);
}

#[test]
fn test_add_with_common() {
  assert_eq!(common::CASES.len(), 2);
  for (left, right, expected) in common::CASES {
    assert_eq!(integration_testing::add(left, right), expected);
  }
}
