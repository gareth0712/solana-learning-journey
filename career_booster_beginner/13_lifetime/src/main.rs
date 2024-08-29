fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}
fn main() {
    let string1 = String::from("Hello");
    let string2 = String::from("World!");
    let result: &str = longest(&string1, &string2);

    // How lifetime helps to prevent dangling references
    // let result;
    // {
    //     let string1 = String::from("Hello");
    //     let string2 = String::from("World!");
    //     result = longest(&string1, &string2);
    // } // This would fail to compile because string1 and string2 go out of scope at the end of the block

    println!("The longest string is {}", result);
}
