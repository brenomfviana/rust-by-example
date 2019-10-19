// Panicking in the previous example's multiply does not make for robust code.
// Generally, we want to return the error to the caller so it can decide what
// is the right way to respond to errors.
//
// We first need to know what kind of error type we are dealing with. To
// determine the Err type, we look to parse(), which is implemented with the
// FromStr trait for i32. As a result, the Err type is specified as
// ParseIntError.
//
// In the example below, the straightforward match statement leads to code that
// is overall more cumbersome.

use std::num::ParseIntError;

// With the return type rewritten, we use pattern matching without `unwrap()`.
fn multiply(first_number_str: &str, second_number_str: &str) ->
  Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
      Ok(first_number) => {
        match second_number_str.parse::<i32>() {
          Ok(second_number) => {
            Ok(first_number * second_number)
          },
          Err(e) => Err(e),
        }
      },
      Err(e) => Err(e),
    }
}

fn print(result: Result<i32, ParseIntError>) {
  match result {
    Ok(n) => println!("n is {}", n),
    Err(e) => println!("Error: {}", e),
  }
}

fn main() {
  // This still presents a reasonable answer.
  let twenty = multiply("10", "2");
  print(twenty);

  // The following now provides a much more helpful error message.
  let tt = multiply("t", "2");
  print(tt);
}
