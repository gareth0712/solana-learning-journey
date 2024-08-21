fn main() {
    // Pattern matching allows you to deconstruct values of complex types into their parts. The pattern does not have to include all possibilities when conditionally matching. This is called a refutable pattern. At other times, for example, when declaring variables through `let`, the pattern must be irrefutable, meaning that the pattern can match any possible value of the type you are matching against.

    // The `let` statement is used to introduce variables into the programs. It may have seemed like you could only introduce one variable simultaneously. The `let` statement is quite powerful. It specifies not just a new name, but a so-called irrefutable pattern.

    // let <pattern>: <type> = <expression>;
    // `: <type>`` bit may be omitted if the compiler can infer the type of the variable.
    // `= <expression>` may be omitted if the programmer wishes to initialize the variable later.

    // Variables can be initialized by providing an expression. The type of expression must match the type of the pattern.
    // The following defines flowering: bool and mass: f64 with given values `true` and `10.0` respectively.
    struct Plant {
        flowering: bool,
        mass: f64,
    }
    let plant = Plant {
        flowering: true,
        mass: 10.0,
    };
    let Plant { flowering, mass } = plant;

    // Meal is a enum with two variants: FishAndChips and Hamburger.
    // FishAndChips variant has a property chip_proportion of type f64.
    // Hamburger variant has a property vegetarian of type bool.
    enum Meal {
        FishAndChips {
            chip_proportion: f64,
        },
        Hamburger {
            vegetarian: bool,
        },
    }

    let meal = Meal::Hamburger {
        vegetarian: true,
    };

    // The following code will not compile because the pattern is refutable, there is a possibility that meal could be a `FishAndChips` variant, e.g. Meal::FishAndChips { chip_proportion: 0.8 }. Given such refutable possibility, Rust requires us to specify an irrefutable pattern using `match`, `if let` or `while let`
    // let Meal::Hamburger { vegetarian } = meal;

    // 1) `if let`
    // We actually only care whether the meal is a hamburger variant or not. We don't care about its property. Thus, use the `..` operator to avoid declaring unused variables like the commented code below.
    // so that its used to match "any remaining fields"
    if let Meal::Hamburger { .. } = meal {
        println!("It is the hamburger vegetarian");
    }
    // if let Meal::Hamburger { vegetarian } = meal {
    //     println!("Is the hamburger vegetarian? {}", vegetarian);
    // }

    // 2) `match`
    // The `match` statement allows you to specify multiple patterns to be matched against. These patterns are matched against in order. This allows these patterns to overlap. This is useful because you can write more specific patterns and handle any other values not matched by those patterns later. The following example matches against an expression of type
    for n in 0..=5 {
        match n {
            1 => println!("It was one!"),
            2 => println!("It was two!"),
            // or-pattern
            3 | 4 => println!("It was a bit more than two!"),
            // match guard
            high if high >= 5 => println!("It was a high number! {high}"),
            // a pattern consisting only of the name `other`
            other => println!("It was {other}!"),
        }
    }

    // Match against an expression of a `enum` type
    let meal = Meal::Hamburger {
        vegetarian: true,
    };
    let meal2 = Meal::FishAndChips { chip_proportion: 0.8 };

    println!("Checking meal ...");
    match meal {
        Meal::FishAndChips { chip_proportion } => {
            if chip_proportion > 0.5 {
                println!("I had some fish and plenty of chips!");
            } else if chip_proportion < 0.5 {
                println!("I had plenty of fish and some chips!");
            } else {
                println!("I had fish and chips!");
            }
        }
        Meal::Hamburger { vegetarian } => {
            if vegetarian {
                println!("I had a vegetarian hamburger!");
            } else {
                println!("I had a meaty hamburger!");
            }
        }
    }

    // `match guard`` version
    // Whether it improved the maintainability of the code or not is subjective tho
    match meal {
        Meal::FishAndChips { chip_proportion } if chip_proportion > 0.5 => {
            println!("I had some fish and plenty of chips!");
        }
        Meal::FishAndChips { chip_proportion } if chip_proportion < 0.5 => {
            println!("I had plenty of fish and some chips!");
        }
        Meal::FishAndChips { chip_proportion } => {
            println!("I had fish and chips!");
        }
        Meal::Hamburger { vegetarian: true } => {
            println!("I had a vegetarian hamburger!");
        }
        Meal::Hamburger { vegetarian: false } => {
            println!("I had a meaty hamburger!");
        }
    }

    println!("Checking meal2 ...");
    match meal2 {
        Meal::FishAndChips { chip_proportion } => {
            if chip_proportion > 0.5 {
                println!("I had some fish and plenty of chips!");
            } else if chip_proportion < 0.5 {
                println!("I had plenty of fish and some chips!");
            } else {
                println!("I had fish and chips!");
            }
        }
        Meal::Hamburger { vegetarian } => {
            if vegetarian {
                println!("I had a vegetarian hamburger!");
            } else {
                println!("I had a meaty hamburger!");
            }
        }
    }

    // 3) `while let`
    // The `while let` statement lets you loop until the pattern does not match. If you like, try to determine what the following program prints.
    let mut meal = Meal::FishAndChips {
        chip_proportion: 0.6,
    };
    while let Meal::FishAndChips { chip_proportion } = meal {
        println!("Having fish and chips with chip proportion {:.2}...", chip_proportion);
        if chip_proportion > 0.3 {
            // Order a meal with less chips.
            meal = Meal::FishAndChips {
                chip_proportion: chip_proportion - 0.2,
            };
        } else {
            // Too fishy, let's get a hamburger next.
            meal = Meal::Hamburger { vegetarian: true };
        }
    }
    println!("I'm so done with fish and chips!");

    // furthermore, `for` loop
    // 👉 In the next example, we are iterating over an array of tuples of the type
    let tuples: [(usize, &'static str); 3] = [
        (1, "red"),
        (2, "white"),
        (3, "blue"),
    ];

    // in the `for <pattern> in <expression> { <body> }` statement, we pattern match against those tuples by assigning the names numbering and color to the first and second element of the tuples.
    for (numbering, color) in tuples {
        println!("Color #{numbering} is {color}!");
    }

    // Making use of `std::Iterator::enumerate` method further extends an existing iterator to provide an index along with the values coming from the existing iterator.

    let colors = ["red", "white", "blue"];

    for (index, color) in colors.into_iter().enumerate() {
        let numbering = index + 1;
        println!("Color #{numbering} is {color}!");
    }
}
