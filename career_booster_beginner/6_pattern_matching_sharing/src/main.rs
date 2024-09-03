fn main() {
    let a = (1, 2);
    let (y, z) = a;
    println!("y: {y}, z: {z}");

    enum Meal {
        FishAndChips {
            chip_proportion: f64,
        },
        Hamburger {
            vegetarian: bool,
        },
    }

    // let meal = Meal::Hamburger {
    //     vegetarian: true,
    // };
    // potential error is guarded when meal is something else
    let meal = Meal::FishAndChips { chip_proportion: 0.8 };
    // let Meal::Hamburger { vegetarian } = meal;

    if let Meal::Hamburger { vegetarian } = meal {
        println!("I had a hamburger!");
    } else {
        println!("I had something else!");
    }
}
