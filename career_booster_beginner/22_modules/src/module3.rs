// Can directly use everything inside module1 without the prefix `module1::`
use crate::module1::*;

pub fn print_all_string() {
  let mut a: String = get_string();
  a.insert_str(0, "Module3: "); // Inserts at the start of the string
  println!("{}", a);

  let mut b: String = get_different_string();
  b.insert_str(0, "Module3: "); // Inserts at the start of the string
  println!("{}", b);
}
