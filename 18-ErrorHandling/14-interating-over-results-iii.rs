// Fail the entire operation with collect()
// Result implements FromIter so that a vector of results (Vec<Result<T, E>>)
// can be turned into a result with a vector (Result<Vec<T>, E>). Once an
// Result::Err is found, the iteration will terminate.

fn main() {
  let strings = vec!["tofu", "93", "18"];
  let numbers: Result<Vec<_>, _> = strings
    .into_iter()
    .map(|s| s.parse::<i32>())
    .collect();
  println!("Results: {:?}", numbers);
}

// This same technique can be used with Option.
