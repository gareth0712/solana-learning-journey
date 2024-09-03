// Rust allows you to write tests both very close to your code and from the perspective of someone consuming your code as a library. Although the definitions not always clear, the first form is often referred to as unit tests, and the second as integration tests.
pub fn add(left: u64, right: u64) -> u64 {
  left + right
}

// The `cargo init --lib abc` contains some code that is not strictly necessary. Setting up a unit test in Rust is very simple: just add a function annotated with `#[test]` and running `cargo test` will execute the test and provide the test result
// So why we still need the #[cfg(test)] annotation and the `mod tests {}`?
// 1) #[cfg(test)] annotation tells the compiler only to include the code in the following item (which is a module named tests) when the feature flag test is enabled
// 2️) Cargo allows you to define your feature flags in the `Cargo.toml` if you want to
// 3) The `test` feature flag automatically gets enabled when `cargo test` runs, making the code compile the contents of the `test` module - a normal `cargo build` will not include the code in the tests module
// More discussion is at the end of this file
#[cfg(test)]
mod tests {
  use super::*;

  // Functions annotated by `#[test]` can be considered program entry points just like `fn main`
  // You can add tests by writing more functions annotated with `#[test]`
  #[test]
  fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }

  #[test]
  fn adding_unequal_numbers_works() {
    assert_eq!(add(2, 3), 5);
  }
}

// It is good to realize that it is possible to omit the `#[cfg(test)]` annotation entirely if desired. However, that would make the binary include the testing code, which is probably unnecessary.
// It is also good to realize that the module's name can be anything you wish, but it is usually named `tests` by convention. If you need functionality in your tests but do not need it as part of the public API of your module, then you can implement the testing functionality within the test's module.
// This is done for the function `Database::at_10` in the following example 👇
#[derive(Default)]
pub struct Database {
  count: u32,
}

#[allow(unused)]
impl Database {
  // Implement this function for testing purposes within the current crate.
  // If you need this function outside of the current crate (like in for example integration tests), use `pub` instead of `pub(crate)`.
  #[cfg(test)]
  pub(crate) fn at_10() -> Database {
    Self { count: 10 }
  }
  pub fn operate(&mut self) {
    self.count += 1;
  }
}

#[cfg(test)]
mod tests_database {
  use super::*;

  // Implement a functions only required for testing.
  impl Database {
    fn test_at_100() -> Database {
      Self { count: 100 }
    }
  }

  #[test]
  fn operate_once() {
    let mut database = Database::at_10();
    database.operate();
    assert_eq!(database.count, 11);
  }

  #[test]
  fn operating_twice() {
    let mut database = Database::at_10();
    database.operate();
    database.operate();
    assert_eq!(database.count, 12);
  }
}
