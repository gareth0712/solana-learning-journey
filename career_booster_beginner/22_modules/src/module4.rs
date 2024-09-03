// only use 1 function from module1
use crate::module1::{ get_string };
use crate::module1;

pub fn print_all_string() {
  let mut a: String = get_string();
  a.insert_str(0, "Module4: "); // Inserts at the start of the string
  println!("{}", a);

  let mut b: String = module1::get_different_string();
  b.insert_str(0, "Module4: "); // Inserts at the start of the string
  println!("{}", b);
}
