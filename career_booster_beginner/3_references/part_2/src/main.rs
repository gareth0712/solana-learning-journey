fn main() {
    // 2) Mutable References (borrowing)
    // 👉 If you need to mutate data without copying and replacing it, then mutable references can help. Any changes to the mutable references is also reflected on its original data
    // *** You can only have one mutable reference to a value simultaneously. ***
    // The `&mut T` represents a mutable reference to the type `T`

    struct Config {
        port: u16,
    }
    let mut config: Config = Config {
        port: 8080
    };
    // Create a mutable reference.
    let config_reference: &mut Config = &mut config;
    // In some other part of the program, use the reference.
    config_reference.port = 4000;
    // Observe the original having been modified.
    println!("The original config port has changed to {}.", config.port);
    // println!("Observe config_reference port {}.", config_reference.port);

    // Dereferencing / Copy
    // To get to the value behind a reference, you can use the 
    // dereference operator `*`.
    let val: i32 = 10;
    let r1: &i32 = &val;
    // This creates a copy of the value 10.
    let val2: i32 = *r1;
    // It works because i32 is copyable. A copyable type is one where we can create a new value simply by copying all the bits. Not all types are copyable.
    // For example, strings are not copyable because it contains a pointer to some memory in a heap.

    // Lifetimes
    // 👉 A lifetime describes in what part of the code a reference can be safely used. While most of the time, the compiler can infer lifetimes for all references automatically, sometimes it is useful to describe your intentions explicitly. The actual complete types of an immutable and mutable reference are 
}
