fn main() {
    // A reference to a variable is a pointer that leads to that variable. Rust uses the concept of ownership, which is associated with how references are used. In this lesson, you will briefly show reference syntax, so you can use them throughout this course.
    // It is often useful to be able to share data within a program. In many programming languages, this can be achieved through pointers or references. Rust has both of them.
    // Note, that pointers are not often used in safe Rust.

    // 1) Immutable References (borrowing)
    // 👉 Sometimes you need to share data that is read-only. For example, let's say you have some application configuration that can be passed around the program. You want to share this configuration without having to create copies of it. This can be achieved with immutable references.
    struct Config {
        port: u16,
    }
    let config: Config = Config {
        port: 8080
    };
    // Create a reference.
    let config_reference_im: &Config = &config;
    // In some other part of the program, use the reference.
    println!("Using port {}.", config_reference_im.port);

    // The `&T` represents a reference to the type `T`
    // 👉 You can have multiple immutable references to a value at the same time.
    let val = 10;
    let r1 = &val;
    let r2 = &val;
    println!("{r1} should be the same as {r2}.");
}
