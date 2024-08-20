fn main() {
    let mut n: u16 = 99;
    let mut s: &mut u16 = &mut n;
    let mut n2: u16 = *s;
    n2 += 1;
    println!("s: {}", s);
    println!("n2: {}", n2);
}
