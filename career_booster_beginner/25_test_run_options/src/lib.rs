// The default behavior of `cargo test` compiles your code in test mode and runs the resulting test binary. The default behavior of the binary produced by cargo test is to run all the tests in parallel and capture output generated during test runs, preventing the output from being displayed and making it easier to read the output related to the test results. You can, however, specify command line options to change this default behavior.

// A) Showing `stdout` Output
// By default, cargo test hides any output written to stdout by tests so that the test rapport looks clean. For example, running `cargo test` with the following test will not show HELLOOOO? in the output 👉
pub fn try_me() {
  println!("HELLOOOO?");
}
// To reveal the output, run `cargo test -- --show-output`

// B) Filtering Tests By Name
// ☝️ It is possible to only run a subset of all tests by passing a string to `cargo test <TESTNAME>`
// Only tests that contain the `<TESTNAME>` string in their fully qualified path will be running.
// All tests that do not contain the string will not be run, but their count will be displayed as filtered out in the test output.
// e.g. try cargo test it_also_works

// C) Ignoring Tests
// The default testing framework will run every test function unless annotated with #[ignore]
// If you have to disable a test for some reason, using the #[ignore] attribute is often better than uncommenting the test function entirely. Unlike uncommented code, ignored tests appear in the test output and can be checked for correctness by the compiler, linters, formatters, and other tools.

// In contrast, 📌 You can run only the ignored tests with `cargo test -- --ignored`
// Also, you can run all tests, both ignored or not, with `cargo test -- --include-ignored`

// D) Running Tests Sequentially
// By default, cargo test runs all tests in parallel. This works when each test is entirely independent. Sometimes, tests must share some resources that cannot be duplicated.
// This is best achieved through the `lazy_static` crate and a `std::sync::Mutex` although work is being done to bring the functionality of `lazy_static` to the standard library.

// To demonstrate, consider this library src/lib.rs exposing access to some external resource 👇
pub fn use_external_resources() {
  pub struct ExternalResource;

  impl ExternalResource {
    pub fn modify(&mut self) {
      unimplemented!();
    }
    pub fn check(&mut self) -> bool {
      unimplemented!();
    }
  }
}
// There is only one instance of this external resource, but there are multiple integration tests that you would like to execute. These tests need to run sequentially. To realize this, we created a module tests/common/mod.rs and continue there

pub fn add(left: u64, right: u64) -> u64 {
  left + right
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }

  #[test]
  pub fn it_also_works() {
    try_me();
  }

  #[test]
  #[ignore] // ignore this test.
  fn ignore_me() {
    assert_eq!(add(2, 3), 5);
  }
}
