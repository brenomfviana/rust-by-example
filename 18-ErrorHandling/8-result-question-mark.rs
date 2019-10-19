// Sometimes we just want the simplicity of unwrap without the possibility of a
// panic. Until now, unwrap has forced us to nest deeper and deeper when what
// we really wanted was to get the variable out. This is exactly the purpose of
// ?.
//
// Upon finding an Err, there are two valid actions to take:
//   - panic! which we already decided to try to avoid if possible
//   - return because an Err means it cannot be handled
//
// ? is almost1 exactly equivalent to an unwrap which returns instead of panics
// on Errs. Let's see how we can simplify the earlier example that used
// combinators:
use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) ->
  Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;
    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
  match result {
    Ok(n)  => println!("n is {}", n),
    Err(e) => println!("Error: {}", e),
  }
}

fn main() {
  print(multiply("10", "2"));
  print(multiply("t", "2"));
}
