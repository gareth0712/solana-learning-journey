use std::str::FromStr;
// A trait describes an interface that implementors of a trait must implement. Traits are used to abstract implementation details, so you can generalize the behavior. The Rust standard library provides a lot of traits. In this lesson, you will go over most of the categories of traits.

// A) ============== Conversion ==============
// 1) From and Into ==========================
// These 2 traits are closely related. They specify infallible value-to-value conversions.
// The `From` and `Into` traits are used for converting between types. The `From` trait is used for converting from a type to another type,
// while the `Into` trait is used for converting from another type to a type.
// The `From` trait is implemented for the `Into` trait, so you can use the `Into` trait to convert between types.
// pub trait From<T> {
//   fn from(value: T) -> Self;
// }

// pub trait Into<T> {
//   fn into(self) -> T;
// }

// For example, you can convert an `i32` to a custom type `MyType` using
struct MyType {
  value: i32,
}

impl From<i32> for MyType {
  fn from(value: i32) -> Self {
    MyType { value }
  }
}

fn from_demo() {
  let num = 10;
  let my_type = MyType::from(num);
  println!("MyType value: {}", my_type.value); // MyType value: 10
}
// When you implement `From<i32> for MyType` then `Into<MyType>` is automatically implemented for `i32` thanks to the blanket implementation that looks something like this 👉
// impl<A, B> Into<B> for A where B: From<A> {
//   fn into(self) -> B {
//     B::from(self)
//   }
// }
// So you can also convert an `i32` to a `MyType` using
fn into_demo() {
  let num = 10;
  let my_type: MyType = num.into(); // Using `Into`
  println!("MyType value: {}", my_type.value); // MyType value: 10
}

// Another example
#[derive(Debug)]
enum DatabaseError {
  DatabaseClosed,
  ProtocolViolation,
}

#[derive(Debug)]
enum ApplicationError {
  Database(DatabaseError),
  IO(std::io::Error),
}

impl From<DatabaseError> for ApplicationError {
  fn from(value: DatabaseError) -> Self {
    Self::Database(value)
  }
}

fn into_demo_2() {
  // You can use `Into::into` to convert the `DatabaseError` into an `ApplicationError`.
  let app_err: ApplicationError = DatabaseError::ProtocolViolation.into();
}

// 2) TryFrom and TryInto ==========================
// These 2 traits are used for fallible value-to-value conversions.
impl TryFrom<ApplicationError> for DatabaseError {
  type Error = ApplicationError;

  fn try_from(value: ApplicationError) -> Result<Self, Self::Error> {
    match value {
      ApplicationError::Database(value) => Ok(value),
      _ => Err(value),
    }
  }
}

fn try_from_demo() {
  // You can use `Into::into` to convert the `DatabaseError` into an `ApplicationError`.
  let app_err: ApplicationError = DatabaseError::ProtocolViolation.into();

  // You can use `TryInto::try_into` to convert the `ApplicationError` back into a `DatabaseError`.
  let db_err: DatabaseError = app_err.try_into().unwrap();

  // You can not turn an `ApplicationError::IO` variant into a `DatabaseError`, so this conversion fails.
  let still_app_err: ApplicationError = DatabaseError::try_from(
    ApplicationError::IO(std::io::Error::new(std::io::ErrorKind::Other, "Something bad happened!"))
  ).unwrap_err();
}

// 3) Display, ToString and FromStr ================
// Types that implement `Display` can be formatted as a user-facing human-readable string. Implementing the `Display` trait for a type will automatically implement `ToString` for that type. The standard library recommends not implementing `ToString` manually.
// For example
#[derive(Debug)]
enum Color {
  Blue,
  Yellow,
}

impl std::fmt::Display for Color {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(match self {
      Color::Blue => "blue",
      Color::Yellow => "yellow",
    })
  }
}

// fmt is called internally when you use the `{}` format specifier.
fn fmt_demo() {
  println!("{}", Color::Yellow); // yellow
}

// fmt is also called when you use the `to_string` method.
fn to_string_demo() {
  let color = Color::Blue;
  let color_str = color.to_string();
  println!("{}", color_str); // blue
}

// The `FromStr` trait should be implemented when you want to allow parsing a value from a string.
impl std::str::FromStr for Color {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "blue" => Ok(Color::Blue),
      "yellow" => Ok(Color::Yellow),
      _ => Err(()),
    }
  }
}

fn from_str_demo() {
  // Use the `from_str` method directly to convert a string into a `Color`.
  let color_result = Color::from_str("blue");

  match color_result {
    Ok(color) => println!("Parsed color: {:?}", color),
    Err(_) => println!("Failed to parse color"),
  }

  // Try parsing an invalid color
  //   let invalid_color_result = Color::from_str("red");

  //   match invalid_color_result {
  //     Ok(color) => println!("Parsed color: {:?}", color),
  //     Err(_) => println!("Failed to parse color: 'red' is not a valid color"),
  //   }
}

// The `FromStr` trait is used by the `parse` method on `str`. The `parse` method is defined on `str` and is available for all types that implement `FromStr`.
// impl str {
//    pub fn parse<F: FromStr>(&self) -> Result<F, F::Err> {
//       FromStr::from_str(self)
//    }
// }

fn parse_demo() {
  // `str::parse::<T>` is defined if `T` implements `FromStr`.
  let color = "blue".parse::<Color>().unwrap(); // Color::Blue
}

// 4) AsRef and AsMut ==============================
// The `AsRef` and `AsMut` traits can be implemented for types that can be temporarily interpreted as another type.
use std::path::{ Path, PathBuf };

fn print_path<P: AsRef<Path>>(path: P) {
  let path = path.as_ref();

  println!("{}", path.display());
}

fn as_ref_as_mut_demo() {
  let a: &'static str = "static_str";
  print_path(a);

  let b: String = String::from("owned_string");
  print_path(b);

  let c: PathBuf = PathBuf::from("owned path");
  print_path(c);
}

// TODO: Continue on other traits
fn main() {}
