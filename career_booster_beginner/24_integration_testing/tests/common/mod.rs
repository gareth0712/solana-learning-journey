// Sharing Testing Code
// Sometimes it can be helpful to split your integration tests over multiple files. When doing so, some common test setup code might need to be shared. If you want to share the testing code, one option is to add a module in the tests/ directory.
// 📌 To avoid having this shared module interpreted as an integration test source file, you can create this module as a directory containing a mod.rs.
// like this file: tests/common/mod.rs
#![allow(unused)]
pub const CASES: [(u64, u64, u64); 2] = [
  (2, 2, 4),
  (3, 2, 5),
];
