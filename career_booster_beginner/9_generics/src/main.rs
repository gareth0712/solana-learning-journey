use std::cmp::PartialEq;
use std::ops::Add;

fn main() {
    // Generics
    // The term generics is used as a method to facilitate generic programming. Generic programming usually means implementing functionality that can be used in many situations. Generics is one solution to generic programming. It allows writing an implementation using types that only know how they behave, but not exactly what type it is.

    // For example, we have two Sequence3, one is of types i32 while another is of types string
    struct Sequence3_I32 {
        first: i32,
        second: i32,
        third: i32,
    }

    struct Sequence3_String {
        first: String,
        second: String,
        third: String,
    }

    impl Sequence3_I32 {
        pub fn new(first: i32, second: i32, third: i32) -> Self {
            Self { first, second, third }
        }
    }

    impl Sequence3_String {
        pub fn new(first: String, second: String, third: String) -> Self {
            Self { first, second, third }
        }
    }

    // We see that both the struct and impl are basically the same and here's when we can apply generics
    struct Sequence3<T> {
        first: T,
        second: T,
        third: T,
    }

    // Read this as: for any type `T`, implement for `Sequence<T>` ...
    impl<T> Sequence3<T> {
        pub fn new(first: T, second: T, third: T) -> Self {
            Self { first, second, third }
        }

        // Now we want to add another method for `Sequence3` that checks if all the elements are the same; but the following code will fail because we haven't determine whether the type that will be filled in for <T> supports checking for equality
        // pub fn all_same(&self) -> bool {
        //     self.first == self.second && self.second == self.third
        // }
    }

    // Thus, we implement this all_same function just for `Sequence3<i32>`
    // impl Sequence3<i32> {
    //     pub fn all_same(&self) -> bool {
    //         self.first == self.second && self.second == self.third
    //     }
    // }

    // fn callSequence3() {
    //     let s = Sequence3 { first: 1, second: 2, third: 3 };
    //     println!("{}", s.all_same()); // false
    // }
    // callSequence3();

    // but the above seems making us back to where we started.
    // A better solution is to tell the compiler to restrict the generic type <T> to only those that support equality checking.
    // Rust allows you to do so with type parameter bounds. 👉 This case would look like <T: PartialEq>
    // Meaning <T> should implement the PartialEq trait
    // For example, types that implement the PartialEq trait can be compared with the binary `==` operator.

    // Traits can be used in generic type parameter bounds written as T: Trait1 + Trait2 + ... These type parameter bounds can be written at the declaration site of the generic type parameter.
    // For all types T implementing PartialEq, implement for Sequence3<T> ...
    // impl<T: PartialEq> Sequence3<T> {
    //     pub fn all_same(&self) -> bool {
    //         self.first == self.second && self.second == self.third
    //     }
    // }

    // You can also write the exact same type bounds in a `where` clause. When type bounds get large, this can be much more readable.

    // We can also move the type bound to a `where` clause.
    impl<T> Sequence3<T> where T: PartialEq {
        pub fn _all_same(&self) -> bool {
            self.first == self.second && self.second == self.third
        }
    }

    impl<T> Sequence3<T> where T: Copy + Add<Output = T> {
        pub fn sum(&self) -> T {
            self.first + self.second + self.third
        }
    }

    let s = Sequence3 { first: 1, second: 2, third: 3 };
    println!("sum of sequence3 is {}", s.sum()); // 6

    // Using Multiple Generic Type Parameters
    struct MyStruct<A, B> {
        a: A,
        b: B,
    }

    // This line will be explained in next section 10_traits
    #[derive(Debug)]
    enum MyEnum<A, B> {
        _A(A),
        B(B),
    }

    let s = MyStruct { a: 10, b: "Hello" };
    println!("s a is: {:?}", s.a);
    println!("s b is: {:?}", s.b);

    // We have to specify the type of the `MyEnum::A` variant here because Rust does not have information to infer it.
    // While using `_` tells Rust to infer the type
    let e = MyEnum::<i32, _>::B("Hello");
    // Given the above #[derive(Debug)] line, we can print the enum as if it has been implemented with the Debug trait
    println!("e is: {:?}", e);

    // Generic Functions
    // It is possible to write free functions that accept generic types too:
    // Accept any type `T` implements `Display` meaning that they can be formatted as text.
    fn say_hello<T: std::fmt::Display>(value: &T) {
        println!("Hello, {value}!");
    }

    say_hello(&true); // Hello, true!
    say_hello(&String::from("World")); // Hello, World!
    say_hello(&1337); // Hello, 1337!
}
