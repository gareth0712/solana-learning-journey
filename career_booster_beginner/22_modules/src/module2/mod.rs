// The "sub_module.rs" is inside the directory "module2".
// To use that, declare this module in the "mod.rs": `mod submodule`; Submodule can be used in the "mod.rs". but declaring via this way it will only be available inside the "mod.rs."
// It is suitable for the open-closed principle of SOLID, but to open it for users of this module, try to make it pub: `Pub mod submodule;` so that other modules can use it.
pub mod sub_module; // Declare the `sub_module` module, which is in `sub_module.rs`

pub fn get_string() -> String {
  String::from("Hello Module2!")
}

fn get_string_private() -> String {
  String::from("Hello Module2!")
}
