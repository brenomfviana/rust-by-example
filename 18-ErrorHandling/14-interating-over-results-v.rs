// When you look at the results, you'll note that everything is still wrapped
// in Result. A little more boilerplate is needed for this.

fn main() {
  let strings = vec!["tofu", "93", "18"];
  let (numbers, errors): (Vec<_>, Vec<_>) = strings
    .into_iter()
    .map(|s| s.parse::<i32>())
    .partition(Result::is_ok);
  let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
  let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
  println!("Numbers: {:?}", numbers);
  println!("Errors: {:?}", errors);
}
