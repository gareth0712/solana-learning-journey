fn main() {
    // By default, variables are immutable in Rust. ☝️ But you still have the option to make them mutable.

    // Immutable Variables
    // variables declaration
    let a;
    // variables initialization
    a = 2;
    // It is often convenient to merge the declaration and initialization into a single statement.
    let _b = 3;

    // Type Ascription
    // 👌 You can inform the compiler of the type you wish the variable to have through type ascription.
    let c: i32 = 5;

    // Mutability
    // 👉 To declare a mutable variable, you can use `let mut`
    let mut x = 2;
    println!("x: {}", x);
    x = 3;
    println!("x changed: {}", x);

    // Scoping
    // Create a new scope
    {
        let y = 4;
        // We can use x here
        println!("x still available in new scope: {}", x); // 3
        println!("y: {}", y); // 4
    }
    println!("x: {}", x); // 3
    // println!("{}", y); // won't compile because y is "not in scope"

    // Shadowing
    // ☝️ It is allowed to redefine a variable with the same name.
    let u = 4;
    let u = 5;
    println!("u: {}", u); // 5

    let v = 6;
    {
        let v = 7;
        println!("v in inner scope: {}", v); // 7
    }
    println!("v: {}", v); // 6

    // Patterns
    // The `let` statement is quite powerful. ☝️ You can provide a pattern instead of just the variable name.
    let (w, z) = (8, 9);
    println!("w: {}", w);
    println!("z: {}", z);
    // The parenthesis in the above statement is not a particular language construct. You are just destructuring a tuple. You can also destructure structs.

    // Define the `Person` type.
    struct Person {
        name: &'static str,
        age: u32,
        likes_brownies: bool,
    }
    // Create a `Person` from its parts.
    let p: Person = Person {
        name: "Mick",
        age: 30,
        likes_brownies: true,
    };
    // Deconstruct the `Person` back into its parts,
    // omitting fields other than `name` and `age`.
    let Person { name, age, .. } = p;
    println!("name: {}", name);
    println!("age: {}", age);

    // the `..` is used to denote "any remaining fields". It allows you to only specify the fields that you care about. The complete description of what `let` can do can be found in the reference.
}
