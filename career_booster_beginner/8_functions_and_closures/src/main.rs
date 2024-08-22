fn main() {
    // Functions are the building blocks of readable, maintainable, and reusable code. In this lesson, you’ll discover different functions and how to declare them. Via examples, you’ll look through function implementation and its most common usage.

    // While implementing a program, you often think of different layers of abstraction. When writing data into a database, you are not concerned about how to write bytes to the network interface and ensure you don’t lose any information in transit. Use functions, closures, and methods to facilitate abstracting and reusing pieces of code.

    // A) Functions
    // fn fn_name(input1: InputType1, input2, InputType2) -> OutputType {
    //   <body>
    // }

    fn _u32_add(a: u32, b: u32) -> u32 {
        return a + b;
    }

    // even without return statement
    fn _u32_add2(a: u32, b: u32) -> u32 {
        a + b
    }

    // defining another function inside a function
    // But fn g cannot be accessed outside of fn f
    // this fn can be defined outside of main fn and can be accessed inside main fn
    fn f(n: u32) -> u32 {
        fn g(n: u32) -> u32 {
            n + 1
        }

        g(n * 2)
    }
    println!("{}", f(3));

    // 1) main function
    // A Rust executable actually needs a function that defines the entry point of the program. This function is called `main`. Just like the main function in this program

    // 2) Associated Functions and Methods
    // We can attach functions to created by ourselves, e.g., struct and enum. If those functions take a value of, or a reference to, a value of the type, they are also referred to as methods. 👉 A method of a type <T> is a function where the first argument is a reference to or value of <T>

    struct X(&'static str);
    // An implementation block for the type `X`.
    impl X {
        // An associated function.
        fn associated_fn() -> &'static str {
            "I am always the same!"
        }
        // A method.
        fn method(self: &Self) -> &'static str {
            self.0
        }
    }
    // Every associated function is a method
    // Call a function associated to the type `X`.
    println!("{}", X::associated_fn());
    // Create an instance of X and call a method on the instance.
    let instance = X("My value depends on an instance of `X`!");
    println!("{}", instance.method());

    // Notice the ways on invoking associated functions and methods are different. We don't need to create an instance of the struct to call an associated function. But we need an instance of the struct to call a method.

    // The type of `self` for a method will often be either `Self`, `&Self`, or `&mut Self`. The `Self` type refers to the type itself, `&Self` refers to a reference to the type, and `&mut Self` refers to a mutable reference to the type.

    // These commonly used types have a shorthand:
    // Shorthand	Equivalent
    // self         self: Self
    // &self        self: &Self
    // &mut self    self: &mut Self

    // So for the above X struct, the method can be written as:
    // fn method(&Self) -> &'static str

    // There are other allowed types such as Box<Self>, Rc<Self>, Arc<Self>, Pin<Self>

    // B) Closures
    // Closures are very similar to functions, except they have the ability to "capture their environment". This can be useful, for example when working with iterators.

    let _c = |x: i32| { x * 2 };
    // or without the curly braces since its only a single expression
    let c = |x: i32| x * 2;
    println!("{}", c(6));
    // The above example could've been written with a fn instead because it does not use its environment.
    // fn c(x: i32) -> i32 {
    //     x * 2
    // }

    // 😀 Closures are most commonly used when iterating over collections of values.
    let mut n = 0;
    let mut c = |x| {
        n += 1;
        x + n - 1
    };
    println!("{}", c(2));
    println!("{}", c(2));
    println!("{}", c(2));

    fn perform_operation(should_add: bool, amount_to_add: i32, value: i32) -> i32 {
        // Approach 1: 1 closure
        let operation = |value: i32| if should_add { value + amount_to_add } else { value };

        // Approach 2: 2 closures
        // This apporach is incorrect although both the branches return i32. In Rust, closures are distinct types even if they have the same signature. So, even though both closures in your if and else branches have the same input type (i32) and return type (i32), they are different types internally. This causes a problem because Rust expects the if and else branches to return the same type.
        // let operation = if should_add {
        //     // A closure that adds `amount_to_add` to `value`.
        //     |value: i32| value + amount_to_add
        // } else {
        //     // A closure that returns `value` without modification.
        //     |value: i32| value
        // };
        operation(value)
    }

    let x: i32 = perform_operation(true, 10, 20);
    println!("x is: {}", x);

    fn prefix_print(prefix: String) -> impl Fn(&str) {
        move |suffix| println!("{prefix} {suffix}")
    }

    fn special() {
        let pp = prefix_print("Hello,".to_string());

        pp("World!");
    }
    special();
}
