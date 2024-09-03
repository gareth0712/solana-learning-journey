// Declaring the module ======================
mod module_here {
  struct Value(i32);
  pub fn get_string() -> String {
    String::from("Hello world!")
  }
  fn get_string_private() -> String {
    String::from("Hello world!")
  }
}

// Use of super keyword ======================
// this function is declared in the root file
pub fn get_string() -> String {
  String::from("Hello world!")
}
mod mod_inside_mod {
  pub fn get_string() -> String {
    // calling the function from the root file
    let mut a = super::get_string();
    a.insert_str(0, "mod1-");
    a
  }
  // we can even declare module inside another module
  pub mod mod2 {
    pub fn get_string() -> String {
      // calling the function from the mod1
      let mut a: String = super::get_string();
      a.insert_str(0, "mod2-");
      a
    }
  }
}

// Modules in another file ===================
mod module1;
mod module3;
mod module4;
// Modules in another directory ==============
// default entrypoint will be `mod.rs`
mod module2;

// Library crate vs Executable crate =========
// The crate with the main file `lib.rs` is a library or executable with a tree of modules.
// 😊 For example, the project structure with a "main.rs" is an executable crate. An executable and library crate can use any count of library crates, but a library crate cannot use an executable crate.
// The modules that we created, like module2, module3, and module4, are merely modules of this executable crate. Although they serve purposes like an library crate does, we don't consider them as library crate because they are not a separate crate.

// Crates vs Modules =========================
// A) Crate: A crate is a compilation unit in Rust. It can be either a library crate (which produces a .rlib file) or an executable crate (which produces a binary executable).

// B) Module: A module is a way to organize code within a crate. Modules help in structuring your code and separating functionality, but they are not standalone crates. They are part of a larger crate, either a library or an executable.

// Importing library crate ===================
// example of library crate that we import
// use std::thread;
// We can even rename the module we import
use std::thread as my_thread;

fn main() {
  println!("{}", module_here::get_string());

  // Non-runable example how of getting access to private field
  //println!("{}", module1::get_string_private());

  println!("{}", module1::get_string());
  println!("{}", module2::get_string());
  module2::sub_module::say_hello();
  module3::print_all_string();
  module4::print_all_string();

  // module inside another module
  println!("{}", mod_inside_mod::get_string());
  println!("{}", mod_inside_mod::mod2::get_string());

  // thread example from the standard library
  println!("The my_thread id is {:?}", my_thread::current().id());
}

// Using external crates =====================
// connect it to the "Cargo. toml" in the section "[dependencies]."
// Way 1
// [dependencies]
// time = "0.3"
// You have connected the crate "time" version 0.3. In this case, cargo may use a younger version but less than 1.0.0.

// Way 2
// The second way is using git:
// [dependencies]
// time = { git = "https://github.com/time-rs/time.git" }

// Way 3
// It is possible to connect the crate from the local path:
// [dependencies]
// time = { path = "../time" }

// ❗️ Take into account then the cargo will cache crates from crates.io and git. To check updates, use the command "cargo update."
// How does cargo search crate in path or git? It looks for the "Cargo. toml" with:
// [package]
// name = "name_of_crate"
