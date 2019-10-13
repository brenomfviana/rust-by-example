fn _fizzbuzz_for() {
  // `n` will take the values: 1, 2, ..., 100 in each iteration
  for n in 1..101 { // Or: 'for n in 1..=100 {'
    if n % 15 == 0 {
      println!("fizzbuzz");
    } else if n % 3 == 0 {
      println!("fizz");
    } else if n % 5 == 0 {
      println!("buzz");
    } else {
      println!("{}", n);
    }
  }
}

// iter(): This borrows each element of the collection through each iteration. Thus leaving the
// collection untouched and available for reuse after the loop.
//
// into_iter(): This consumes the collection so that on each iteration the exact data is provided.
// Once the collection has been consumed it is no longer available for reuse as it has been 'moved'
// within the loop.
//
// iter_mut(): This mutably borrows each element of the collection, allowing for the collection to
// be modified in place.
fn iterate_over_an_array() {
  // // iter
  // let names = vec!["Bob", "Frank", "Ferris"];
  // for name in names.iter() {
  //   match name {
  //     &"Ferris" => println!("There is a rustacean among us!"),
  //     _ => println!("Hello {}", name),
  //   }
  // }
  //
  // // into_iter
  // let names = vec!["Bob", "Frank", "Ferris"];
  //
  // for name in names.into_iter() {
  //   match name {
  //     "Ferris" => println!("There is a rustacean among us!"),
  //     _ => println!("Hello {}", name),
  //   }
  // }
  //
  // iter_mut
  let mut names = vec!["Bob", "Frank", "Ferris"];
  for name in names.iter_mut() {
    *name = match name {
      &mut"Ferris" => "There is a rustacean among us!",
      _ => "Hello",
    }
  }
  println!("names: {:?}", names);
}

fn main() {
  // _fizzbuzz_for();
  iterate_over_an_array();
}
