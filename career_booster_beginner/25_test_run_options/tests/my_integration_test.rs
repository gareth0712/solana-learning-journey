mod common;

use common::{ ExternalResourceExt, EXTERNAL_RESOURCE };

#[test]
fn check_fails_initially() {
  let mut db = EXTERNAL_RESOURCE.lock().unwrap();

  db.reset();
  assert!(!db.check());
}

#[test]
fn check_passes_after_modify() {
  let mut db = EXTERNAL_RESOURCE.lock().unwrap();

  db.reset();
  db.modify();
  assert!(!db.check());
}

// 📌 The tests will fail, of course, since the implementation is missing, but the concept of putting a shared resource behind a globally accessible Mutex should be clear.
