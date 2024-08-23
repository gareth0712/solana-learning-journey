fn main() {
    // Macros
    // ☝️ Macros are a powerful utility that lets you write less repetitive code. There are multiple kinds of macros: declarative macros and procedural macros

    // A) Declarative Macros - match input code against patterns and replace them with other patterns to produce new code
    // 👉 Declarative macros strongly resemble functions because they can take parameters and produce some output. Unlike functions, declarative macros match patterns against pieces of code provided to the macro, and substitute the input code with new code

    // e.g. the println! is a macro. The `!` mark at the end signals that you are invoking a declarative macro.

    // Declarative macros can be defined with the `macro_rules!` syntax extension. Syntax extensions are built into the compiler. Syntax extensions and user-defined declarative macros look the same, which does not matter in everyday use but can be confusing. The following example defines a macro called `create_vec!` that creates a vector from a list of items:
    macro_rules! create_vec {
        ($($item:expr),*) => {
        {
            let mut result = Vec::new();
            $(
                result.push($item);
            )*
            result
        }
        };
    }
    let items = create_vec!(1, 2, 3);
    println!("{items:?}");

    // B) Procedural Macros - transform input code in any way possible but take more effort to implement correctly
    // Procedural macros allow transforming input code into any new code. You can define your procedural macros, but this topic will not be treated in this course, as you will likely not need to know how to do this to write good Rust programs. Procedural macro definitions must reside in their crate with a particular crate type.

    // 3 types of procedural macros:
    // 1) Custom derive macros - allow you to define custom behavior for structs and enums
    // An example is `Default`. For any type, you can derive a `Default` trait implementation if all of its components also implement `Default`

    #[derive(Default)]
    struct MyType {
        name: String,
        items: Vec<i32>,
    }
    let v = MyType::default();
    assert!(v.name.is_empty());
    assert!(v.items.is_empty());

    // The generated implementation for MyType will look something like this:
    // impl Default for MyType {
    //     fn default() -> Self {
    //         Self {
    //             name: Default::default(),
    //             items: Default::default(),
    //         }
    //     }
    // }

    // 👉 You can derive multiple traits implementations.
    #[derive(Debug, Clone, Default, Eq, PartialEq)]
    struct YourType {
        name: String,
        items: Vec<i32>,
    }

    let v1 = YourType::default(); // Uses Default to instantiate a default value.
    let v2 = v1.clone(); // Uses Clone to create a clone of v1.
    assert_eq!(&v1, &v2); // Uses Eq to compare the two values for equality.
    println!("{v1:#?}"); // Uses Debug to print the value.

    // You can see how custom derive macros are extremely useful in generating essential, but repetitive code for you. Writing `#[derive(...)]` is much more concise than writing all the derived implementations by hand.

    // 2) Attribute-like macros - define custom attributes that can be applied to any item

    // Examples of items are function, constant and type definitions.
    #[my_attr_macro]
    fn x() {}

    #[my_attr_macro]
    const Y: u32 = 1;

    #[my_attr_marco]
    struct Z;
    // 👉 The macro can transform the code it applies to into any other code.

    // 3) Function-like macros - Function-like macros can generate any code from its input code. They are invoked in exactly the same way as declarative macros.

    // e.g. `my_fn_macro!(some stuff);`
}
