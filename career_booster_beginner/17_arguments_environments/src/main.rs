fn main() {
  // Print all arguments given to the program
  // try `$ cargo run hi` => hi is printed
  for argument in std::env::args() {
    println!("{argument}");
  }
  // Creating a proper command line application with subcommands, positional and optional parameters is probably best done by utilizing a library from the Rust ecosystem like clap.

  // The environment variables set for the program upon invocation can also be accessed through the std::env module.
  // The following prints all environment variables
  for (key, value) in std::env::vars() {
    println!("{key}={value}");
  }
  // current directory
  let path = std::env::current_dir().unwrap();
  println!("The current directory is {}", path.display());
}
