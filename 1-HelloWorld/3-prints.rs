// format!: write formatted text to String
// print!: same as format! but the text is printed to the console (io::stdout).
// println!: same as print! but a newline is appended.
// eprint!: same as format! but the text is printed to the standard error (io::stderr).
// eprintln!: same as eprint!but a newline is appended.

fn main() {
  // In general, the `{}` will be automatically replaced with any
  // arguments. These will be stringified.
  println!("{}", format!("{} days", 31));

  // Without a suffix, 31 becomes an i32. You can change what type 31 is
  // by providing a suffix.

  // There are various optional patterns this works with. Positional
  // arguments can be used.
  println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

  // As can named arguments.
  println!("{subject} {verb} {object}",
           object="the lazy dog",
           subject="the quick brown fox",
           verb="jumps over");

  // Special formatting can be specified after a `:`.
  println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

  // You can right-align text with a specified width. This will output
  // "     1". 5 white spaces and a "1".
  println!("{number:>width$}", number=1, width=6);

  // You can pad numbers with extra zeroes. This will output "000001".
  println!("{number:>0width$}", number=1, width=6);

  // Rust even checks to make sure the correct number of arguments are
  // used.
  eprintln!("My name is {0}, {1} {0}", "Bond", "James");
  // FIXME ^ Add the missing argument: "James"

  // Create a structure named `Structure` which contains an `i32`.
  #[allow(dead_code)]
  struct Structure(i32);

  // However, custom types such as this structure require more complicated
  // handling. This will not work.
  // println!("This struct `{}` won't print...", Structure(3));
  // FIXME ^ Comment out this line.

	// Activity: Print PI value
	let pi = 3.141592;
  println!("Pi is roughly {:.*}", 2, pi);
}
