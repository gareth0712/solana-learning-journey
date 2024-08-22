fn main() {
    // A) Statements and Expressions
    // Rust programs are composed of sequences of statements. Statements are terminated by a semicolon `;`
    // Statements exist so that you can cause side effects.

    // 👉 Examples of side effects are printing to the terminal, writing some data into an external database. Without side effects, programs would not be very useful because they would not modify any state outside their execution. Statements often contain expressions.

    // An `if` statement is an expression statement containing an `if` expression

    // 👉 It is like a ternary if. The if statement is an expression that in other languages is sometimes called the ternary conditional operator. This expression is often written as
    // a ? b : c
    // where the expression evaluates to b when a is true; and to c if a is false

    let brownies_eaten = 0;
    // if and match
    if brownies_eaten == 0 {
        println!("I had no brownies today.");
    } else {
        println!("I had at least one brownie today.");
    }

    // 😀 You can reduce the duplication in the above code like so.
    let quantifier;
    if brownies_eaten == 0 {
        quantifier = "no brownies";
    } else {
        quantifier = "at least one brownie";
    }
    println!("I had {quantifier} today.");

    // You can simplify the above code by realizing that `if` can be used as an expression
    let quantifier = if brownies_eaten == 0 { "no brownies" } else { "at least one brownie" };
    println!("I had {quantifier} today.");

    // Just like `if match` is also an expression
    let quantifier = match brownies_eaten {
        0 => "no brownies",
        1 => "a brownie",
        _ => "multiple brownies",
    };
    println!("I had {quantifier} today.");

    // B) Scopes
    // A scope is a block of code enclosed between { and }. A scope can end with a statement, in which case the scope evaluates to nothing, or with an expression, in which case the scope evaluates to the result of the expression.
    let x;
    {
        println!("`x` has no value yet!");
        x = 3; // notice the final semicolon
    }

    let x = {
        println!("`x` has no value yet!");
        3 // notice the lack of a semicolon, indicating an expression.
    }; // notice the final semicolon, the let statement needs to end somewhere!

    // loop
    // There is one `loop` construct, namely loop, that can be written as an expression.
    let mut i = 0;
    let x = loop {
        if i > 7 {
            break i;
        }
        i += i * 2 + 1;
    };
    println!("{x}");
}
