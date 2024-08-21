fn main() {
    // Conditional execution
    // 🤓 The condition expressions must evaluate to a value of type `bool` otherwise it won't compile

    let should_print = true;
    if should_print {
        println!("Printing!");
    }

    let value = 10;
    if value > -10 && value < 10 {
        println!("Single digit!");
    } else {
        println!("Multiple digits!");
    }

    let value = 10;
    if value == 0 {
        println!("Zero!");
    } else if value > -10 && value < 10 {
        println!("Single digit!");
    } else {
        println!("Multiple digits!");
    }

    // Loops
    // 1) Simplest `loop`
    // loops indefinitely until you explicitly tell it to stop with a `break` statement or by returning from the current function through `return`
    let mut counter = 0;
    loop {
        println!("I can't stop!");
        counter += 1;
        if counter == 5 {
            break;
        }
    }

    // 2) `while` loop
    //  loop runs until a condition is met.
    let mut i = 5;
    while i != 0 {
        println!("{i}...");
        i -= 1;
    }
    println!("Launch!");

    // 3) `for` loop
    // loop runs until something called an iterator cannot yield more values. Most iterators can only produce a finite number of items, but some do not end until you hit a limit of the actual machine it is running on.
    for i in (1..=10).rev() {
        // putting i inside or outside {} is the same
        println!("{}...", i);
    }
    println!("Launch!");
    // i) 1..=10 is a range expression that creates a range from 1 up to and including 10
    // ii) 1..10 is a range that includes 1 but excludes 10
    // iii) .rev() calls a function on the range that produces a reversed iterator. That means you visit all the items from the end to the start. The `rev` function is implemented for any type that implements the `DoubleEndedIterator` trait

    // A trait is much like an interface that defines what you can do with values of types that implement that trait. This, in combination with generics, lets you add functionality to many types, even though you only have to write the code for that functionality once.

    // Use of continue in a loop
    for i in (1..=10).rev() {
        if i % 2 == 0 {
            continue;
        }
        println!("{i}...");
    }
    println!("Launch!");

    // We can add parentheses around the condition tho it is not required. But its a must for curly braces in the body of the condition.
    // if (i < 1) {
    //     println!("i is less than 1")
    // }
}
