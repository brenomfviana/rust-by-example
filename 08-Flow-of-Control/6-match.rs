// https://doc.rust-lang.org/stable/rust-by-example/flow_control/match.html
// Line 15: deprecated on original post. Range '...' is now '..='

fn main() {
  let number = 13;
  // TODO ^ Try different values for `number`

  println!("Tell me about {}", number);
  match number {
    // Match a single value
    1 => println!("One!"),
    // Match several values
    2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
    // Match an inclusive range
    13..=19 => println!("A teen"),
    // Handle the rest of cases
    _ => println!("Ain't special"),
  }

  let boolean = true;
  // Match is an expression too
  let binary = match boolean {
    // The arms of a match must cover all the possible values
    false => 0,
    true => 1,
    // TODO ^ Try commenting out one of these arms -- Result: it doesn't work
  };

  println!("{} -> {}", boolean, binary);
}
