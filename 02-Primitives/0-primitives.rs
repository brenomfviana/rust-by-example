// Scalar Types
//   signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
//   unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
//   floating point: f32, f64
//   char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
//   bool either true or false
//   and the unit type (), whose only possible value is an empty tuple: ()
//   Despite the value of a unit type being a tuple, it is not considered a compound type because
//   it does not contain multiple values.
//
// Compound Types
//   arrays like [1, 2, 3]
//   tuples like (1, true)

fn main() {
  // Variables can be type annotated.
  let logical: bool = true;
  println!("{}", logical);

  let a_float: f64 = 1.0;  // Regular annotation
  println!("{:.2}", a_float);
  let an_integer = 5i32; // Suffix annotation
  println!("{}", an_integer);

  // Or a default will be used.
  let default_float = 3.0; // `f64`
  println!("{}", default_float);
  let default_integer = 7;   // `i32`
  println!("{}", default_integer);

  // A type can also be inferred from context
  let mut inferred_type = 12; // Type i64 is inferred from another line
  println!("{}", inferred_type);
  inferred_type = 4294967296i64;
  println!("{}", inferred_type);

  // A mutable variable's value can be changed.
  let mut mutable = 12; // Mutable `i32`
  println!("{}", mutable);
  mutable = 21;
  println!("{}", mutable);

  // Error! The type of a variable can't be changed.
  // mutable = true;

  // Variables can be overwritten with shadowing.
  let mutable = true;
  println!("{}", mutable);
}
