// We proved some additional functionality on the `ExternalResource` type through the `ExternalResourceExt` trait and create an instance of `ExternalResource` within a globally accessible `Mutex` through `lazy_static`

#![allow(unused)]
fn main() {
  use std::sync::Mutex;
  use test_run_options::ExternalResource;

  // Functionality that is not part of ExternalResource's public API but is useful for testing.
  pub trait ExternalResourceExt {
    fn reset(&mut self);
  }
  impl ExternalResourceExt for ExternalResource {
    fn reset(&mut self) {
      unimplemented!();
    }
  }

  lazy_static::lazy_static! {
    // You abstract mutually exclusive access to an external resource through this global variable.
    pub static ref EXTERNAL_RESOURCE: Mutex<ExternalResource> = Mutex::new(ExternalResource);
  }
}
