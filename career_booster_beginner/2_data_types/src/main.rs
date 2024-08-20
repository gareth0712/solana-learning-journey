fn main() {
    // A) Scalar Types
    // 1) Integers
    // a) Signed and unsigned integers
    // -128 to 127
    const A: i8 = 127;
    println!("a: {}", A);
    // 2^16 / 2 * (-1) to 2^16 / 2 - 1, i.e. -32768 to 32767
    const B: i16 = -32768;
    println!("b: {}", B);
    // 0 to 2^128 - 1
    const C: u128 = 123456789012345678901234567890;
    println!("c: {}", C);
    // 32 bit or 64 bit depending on the compiling system
    let d: isize = 1;
    println!("d: {}", d);
    // b) Integers: Dex, Hex, Octal, Binary, Byte (u8)
    // using mut keyword to allow re-assignment
    let mut e: u128 = 40_500;
    println!("e: {}", e);
    e = 40_900u128;
    println!("e changed? {}", e);
    e = 40_600_u128;
    println!("e changed? {}", e);
    // Octal, using 0o prefix
    let f: i128 = 0o77;
    println!("f: {}", f);
    // Hex, using 0x prefix
    let g: i128 = 0xff;
    println!("g: {}", g);
    // Binary, using 0b prefix
    let mut h: i128 = 0b1111_0000;
    println!("h: {}", h);
    h = 0b11111111_i128;
    println!("h changed? {}", h);
    let i: u8 = b'A';
    println!("i: {}", i);

    // 2) Floating-point numbers
    let j: f32 = 2022f32;
    println!("j: {}", j);
    let k: f64 = 2023f64;
    println!("k: {}", k);
    let l: f32 = 6.;
    println!("l: {}", l);
    let m: f32 = 16.0;
    println!("m: {}", m);
    let n: f32 = 2.002E1;
    println!("n: {}", n);

    // 3) Boolean
    let o: bool = true;
    println!("o: {}", o);

    // 4) Character
    let p: char = 'A';
    println!("p: {}", p);
    let q: char = '😻';
    println!("q: {}", q);
    // for more than one character, use string

    // B) Compound Types
    // 1) Arrays - fixed size
    let array: [u32; 3] = [1, 2, 3];
    println!("array: {:?}", array);
    println!("array[0]: {}", array[0]);
    // accessing an element outside the array will cause a panic
    // println!("array[3]: {}", array[3]);

    // 2) Slices - dynamic size
    // also named unsized type; dynamically sized type (DST)
    let slice: &[u32] = &[1, 2, 3, 4];
    println!("slice: {:?}", slice);

    // 3) string
    let s: &str = "Hello World!";
    println!("s: {}", s);

    // 4) Tuples
    let tuple: (bool, u32, f64) = (true, 2, 3.0);
    println!("tuple: {:?}", tuple);
    println!("tuple.0: {}", tuple.0);
    println!("tuple.1: {}", tuple.1);
    println!("tuple.2: {}", tuple.2);

    // 5) Tuple Structs
    // a) struct keyword
    // 👉 It can be useful to name tuples and can be done with tuple structs.
    struct MyTuple(bool, u32, f64);
    let tuple_hi = MyTuple(false, 2, 3.0);
    println!("tuple_hi.0: {}", tuple_hi.0);
    println!("tuple_hi.1: {}", tuple_hi.1);
    println!("tuple_hi.2: {}", tuple_hi.2);

    // b) type alias
    type MyTupleAlias = (bool, u32, f64);
    let tuple_hi_alias: MyTupleAlias = (false, 2, 3.0);
    println!("tuple_hi_alias.0: {}", tuple_hi_alias.0);
    println!("tuple_hi_alias.1: {}", tuple_hi_alias.1);
    println!("tuple_hi_alias.2: {}", tuple_hi_alias.2);

    // 6) Structs
    // We use regular structs over tuple struct when we want to give the fields a custom name
    struct MyStruct {
        should_do_groceries: bool,
        birth_year: u32,
        height_in_meters: f64,
    }
    let my_struct = MyStruct {
        should_do_groceries: true,
        birth_year: 1990,
        height_in_meters: 1.75,
    };

    println!("my_struct.should_do_groceries: {}", my_struct.should_do_groceries);
    println!("my_struct.birth_year: {}", my_struct.birth_year);
    println!("my_struct.height_in_meters: {}", my_struct.height_in_meters);

    // 7) Enums
    // Purpose: make sure the value holds one of multiple kinds of values which are known at compile time
    enum CardinalDirection {
        North,
        East,
        South,
        West,
    }

    let d = CardinalDirection::East;
    if let CardinalDirection::East = d {
        println!("We are going east!");
    } else {
        println!("We are not going east but in some other direction!");
    }

}
