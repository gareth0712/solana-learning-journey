use std::{ fs, io };
use std::fs::File;
use std::io::prelude::*;
// The Rust standard library provides types and functions to interact with the file system under std::fs. This lesson introduces you to the terms of files and directories.

// The `File` type is an abstraction around files.
// The `File::create` function creates a new file if it doesn't exist, or truncates it if it does.
fn createFile() -> std::io::Result<()> {
  let mut file = File::create("foo.txt")?;
  file.write_all(b"Hello, world!")?;
  Ok(())
}

// The file::open is used to open a file in read-only mode.
fn readFile() -> std::io::Result<()> {
  let mut file = File::open("foo.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  assert_eq!(contents, "Hello, world!");
  Ok(())
}

// The above can also be realized more easily through `fs::write` and `fs::read`

// Directories
// 😀 You can list the entries in a directory using
fn readDirectory() -> io::Result<()> {
  for entry in fs::read_dir(".")? {
    let entry = entry?;
    let file_type = entry.file_type()?;
    let prefix = match file_type {
      t if t.is_dir() => "D",
      t if t.is_file() => "F",
      t if t.is_symlink() => "L",
      _ => "?",
    };
    println!("{prefix} {}", entry.path().display());
  }

  Ok(())
}

// Recursively walking directories is not directly supported by the standard library, but some libraries can help, like ignore.

// Module that is useful to import when working with reading and writing -> std::io::prelude
// Functionality to interact with the filesystem -> std::fs

fn main() {
  createFile();
  readFile();
}
