// Ignore the failed items with filter_map()
fn main() {
  let strings = vec!["tofu", "93", "18"];
  let numbers: Vec<_> = strings
    .into_iter()
    .map(|s| s.parse::<i32>())
    .filter_map(Result::ok)
    .collect();
  println!("Results: {:?}", numbers);
}
