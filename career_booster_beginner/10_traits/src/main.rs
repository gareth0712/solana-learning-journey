use std::fmt::Display;
use std::fmt;

fn main() {
    // A) Traits
    // When writing abstractions, you need some method of requiring types to expose some kind of interface or contract. In Rust, you do this through traits.

    // The standard library defines many useful traits, such as the Display trait.
    // Types that implement `Display` can be formatted. Thanks to formatting, we can print values in a number of ways.
    let value = "Hello";
    println!("regular: {}", value);
    println!("padded : {:_>8}", value);

    // More about `Display` traits
    // impl Display for str {
    //     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    //         f.pad(self)
    //     }
    // }
    // This implementation defers to the formatter's `pad` function, which handles the string padding and alignment and thus we can manipulate the string formatting with {:_>8} (as it makes use of pad function under the hood).

    // B) Bounds (Trait Bounds)
    struct MyStruct<A, B> {
        a: A,
        b: B,
    }
    // Earlier in this lesson, you’ve left off with an example that did not compile:
    // impl<A, B> MyStruct<A, B> {
    //     fn print(&self) {
    //         println!("a: {}, b: {}", self.a, self.b);
    //     }
    // }

    // The compiler throws two similar messages, the first of them being:
    // error[E0277]: `A` doesn't implement `std::fmt::Display`
    // println!("a: {}, b: {}", self.a, self.b);
    //                          ^^^^^^ `A` cannot be formatted
    // with the default formatter
    // = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead

    // The compiler message is very helpful as it recommends exactly what you should do to make it work. You should require the generic types `A` and `B` to implement the `Display` trait.
    impl<A: Display, B: Display> MyStruct<A, B> {
        fn print(&self) {
            println!("a: {}, b: {}", self.a, self.b);
        }
    }

    // If the bounds get too complicated, we can move the bounds to a `where` clause
    // impl<A, B> MyStruct<A, B>
    // where
    //     A: Display,
    //     B: Display {

    // C) Derive
    // Some trait implementations may be very predictable. They can be derived based on the type definition.
    // You can derive a Debug implementation with a procedural macro defined by the standard library as follows:
    #[derive(Debug)] // Automatic implementation of Debug trait
    struct Person {
        name: String,
        age: i32,
    }

    // Manually Implement the `Debug` trait for the `Person` struct
    // 1) Define the `fmt` method to specify how the `Person` should be formatted
    // impl fmt::Debug for Person {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // Customize the output format here
    //         write!(f, "Person {{ name: \"{}\", age: {} }}", self.name, self.age)
    //     }
    // }

    let mick = Person {
        name: "Mick".to_string(),
        age: 30,
    };

    // Without the derive line, we can't println the struct
    println!("{:?}", mick);

    // When you derive a trait, like std::fmt::Debug, what happens is that a procedural macro generates the trait implementation for you

    // Lesson Summary
    // You now know what traits are and how to declare, implement and use them as generic type parameter bounds. There is another use for traits, which is dynamic dispatch. You also know that some trait implementations do not have to be written manually and can be derived through procedural macros.
}
