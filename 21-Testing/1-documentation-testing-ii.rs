// The main purpose of documentation tests is to serve as examples that
// exercise the functionality, which is one of the most important guidelines.
// It allows using examples from docs as complete code snippets. But using ?
// makes compilation fail since main returns unit. The ability to hide some
// source lines from documentation comes to the rescue: one may write fn
// try_main() -> Result<(), ErrorType>, hide it and unwrap it in hidden main.
// Sounds complicated? Here's an example:

/// Using hidden `try_main` in doc tests.
///
/// ```
/// # // hidden lines start with `#` symbol, but they're still compileable!
/// # fn try_main() -> Result<(), String> { // line that wraps the body shown in doc
/// #   let res = try::try_div(10, 2)?;
/// #   Ok(()) // returning from try_main
/// # }
/// # fn main() { // starting main that'll unwrap()
/// #   try_main().unwrap(); // calling try_main and unwrapping
/// #                        // so that test will panic in case of error
/// # }
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
  if b == 0 {
    Err(String::from("Divide-by-zero"))
  } else {
    Ok(a / b)
  }
}
