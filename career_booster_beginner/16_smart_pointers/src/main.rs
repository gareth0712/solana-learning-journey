use std::cell::RefCell;
use std::any::type_name;

fn print_type_of<T>(_: &T) {
  println!("Its type is: {}", type_name::<T>());
}

fn main() {
  // TODO: Continue on learning
  // 1) Immutable vec
  let data = vec![1, 2, 3, 4, 5];
  println!("Vector Data is: {:?}", data);
  print_type_of(&data); // Vec<i32>

  {
    let slice: &[i32] = &data; // Slice of the whole vector without explicit [..]
    println!("Slice of the Vec without [..] {:?}", slice);
    print_type_of(&slice); // &[i32]
  }

  {
    let slice: &[i32] = &data[..]; // Slice of the whole vector
    println!("Slice of the whole vector with [..] {:?}", slice);
    print_type_of(&slice); // &[i32]
  }

  println!("\n====================================================\n");
  // 2) Mutable vec
  let mut_data = &mut vec![1, 2, 3, 4, 5];
  println!("Vector Data is: {:?}", mut_data);
  print_type_of(&mut_data); // &mut Vec<i32>

  {
    // Explicit type annotation is required here
    // Because without that, the type of mut_slice is inferred as &mut Vec<i32> (because mut_data is &mut Vec<i32>)
    // And Rust assumes that you want to preserve the vector type unless you explicitly state otherwise
    // When you explicitly annotate the type as &mut [i32], you're telling Rust that you want mut_slice to be a mutable slice of the vector. Rust then coerces &mut Vec<i32> into &mut [i32], meaning it views the entire vector as a slice of its elements
    // Vectors (Vec<T>) and Slices ([T]) are different types. A vector owns its data and manages the memory, while a slice is just a reference to a contiguous segment of memory (like part of an array or vector).
    // Rust allows implicit coercion from &Vec<T> or &mut Vec<T> to &[T] or &mut [T], but this coercion doesn't always happen automatically unless you explicitly tell Rust you want a slice by annotating the type.
    // This is not a good practice as using implicit coercion can lead to confusion and potential error-prone code
    let mut mut_slice: &mut [i32] = mut_data; // Mutable slice of the whole vector
    println!(
      "Mutable slice of the whole vector without [..], but with type annotation {:?}",
      mut_slice
    );
    print_type_of(&mut_slice);
  }

  // Explicit [..] is required here
  // Why is [..] making a difference?
  // it creates a slice from the vector, or breaking it down into an array-like view of its elements
  // so it gives you a mutable reference (&mut [i32]) from the entire vector
  {
    let mut mut_slice = &mut mut_data[..]; // Mutable slice of the whole vector
    println!("Mutable slice of the whole vector with [..] {:?}", mut_slice);
    print_type_of(&mut_slice); // &mut [i32]
  }

  println!("\n====================================================\n");

  // 3) RefCell + Vec
  let mut ref_data = RefCell::new(vec![1, 2, 3, 4, 5]);
  println!("RefCell Vector Data is: {:?}", ref_data);
  print_type_of(&ref_data); // &mut RefCell<Vec<i32>>

  // a) Immutable Reference (borrow)
  {
    // borrow returns a Ref<Vec<i32>>, which is a smart pointer that behaves like a reference but provides safety for interior mutability with RefCell.
    let borrowed_data = ref_data.borrow();
    println!("Borrowed immutable data without &: {:?}", borrowed_data);
    print_type_of(&borrowed_data); // Ref<Vec<i32>>
  }

  {
    // adding a & to Ref<Vec<i32> gives you a &Ref<Vec<i32>>, which is a reference to the Ref object, not a reference to the vector inside it.
    let borrowed_data = &ref_data.borrow();
    println!("Borrowed immutable data with &: {:?}", borrowed_data);
    print_type_of(&borrowed_data); // &Ref<Vec<i32>>
  }

  // Take an immutable slice of the entire vector
  // Ref<Vec<T>> is a smart pointer that wraps a reference to the Vec<T> inside a RefCell. It's used for managing the immutable borrow of the data in a RefCell.
  // When you use the [..] syntax, you coerce or convert the Vec<T> into a slice (&[T]). This means you no longer work with the Vec<T> (the growable collection), but instead with a view (slice) of the vector's elements.
  {
    let borrowed_data = &ref_data.borrow()[..]; // Slicing the Vec<i32> into &[i32]
    println!("Borrowed Immutable data with & + [..]: {:?}", borrowed_data);
    print_type_of(&borrowed_data); // &[i32]
  }

  println!("\n====================================================\n");

  // b) Mutable reference (borrow_mut)
  {
    // borrow_mut returns a RefMut<Vec<i32>>, which is a smart pointer that behaves like a mutable reference but provides safety for interior mutability with RefCell.
    let mut borrowed_data_mut = ref_data.borrow_mut();
    borrowed_data_mut[0] = 10; // Mutate the data
    println!("Mutated data: {:?}", borrowed_data_mut);
    print_type_of(&borrowed_data_mut); // RefMut<Vec<i32>>
  }

  {
    // adding a & to RefMut<Vec<i32> gives you a &RefMut<Vec<i32>>, which is a reference to the RefMut object, not a reference to the vector inside it.
    let mut borrowed_data_mut = &mut ref_data.borrow_mut();
    borrowed_data_mut[0] = 10; // Mutate the data
    println!("Mutated data: {:?}", borrowed_data_mut);
    print_type_of(&borrowed_data_mut); // RefMut<Vec<i32>>
  }

  {
    // Take an mutable slice of the entire vector
    // adding a & to RefMut<Vec<i32> gives you a &RefMut<Vec<i32>>, which is a reference to the RefMut object, not a reference to the vector inside it.
    // When you use the [..] syntax, you coerce or convert the Vec<T> into a slice (&[T]). This means you no longer work with the Vec<T> (the growable collection), but instead with a view (slice) of the vector's elements.
    let mut borrowed_data_mut = &mut ref_data.borrow_mut()[..];
    borrowed_data_mut[0] = 10; // Mutate the data
    println!("Mutated data: {:?}", borrowed_data_mut);
    print_type_of(&borrowed_data_mut); // &mut [i32]
  }
}
