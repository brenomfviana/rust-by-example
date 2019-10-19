// There are times when we'll want to stop processing on errors (like with ?)
// but keep going when the Option is None. A couple of combinators come in
// handy to swap the Result and Option.

use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
  let opt = vec.first().map(|first| {
    first.parse::<i32>().map(|n| 2 * n)
  });

  let opt = opt.map_or(Ok(None), |r| r.map(Some))?;

  Ok(opt)
}

fn main() {
  let numbers = vec!["42", "93", "18"];
  let empty = vec![];
  let strings = vec!["tofu", "93", "18"];

  println!("The first doubled is {:?}", double_first(numbers));
  println!("The first doubled is {:?}", double_first(empty));
  println!("The first doubled is {:?}", double_first(strings));
}
