// Tests are Rust functions that verify that the non-test code is functioning
// in the expected manner. The bodies of test functions typically perform some
// setup, run the code we want to test, then assert whether the results are
// what we expect.
//
// Most unit tests go into a tests mod with the #[cfg(test)] attribute. Test
// functions are marked with the #[test] attribute.
//
// Tests fail when something in the test function panics. There are some helper
// macros:
//   - assert!(expression) - panics if expression evaluates to false.
//   - assert_eq!(left, right) and assert_ne!(left, right) - testing left and
//     right expressions for equality and inequality respectively.

// Execute with: `cargo test`

pub fn add(a: i32, b: i32) -> i32 {
  a + b
}

// This is a really bad adding function, its purpose is to fail in this
// example.
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
  a - b
}

#[cfg(test)]
mod tests {
  // Note this useful idiom: importing names from outer (for mod tests) scope.
  use super::*;

  #[test]
  fn test_add() {
    assert_eq!(add(1, 2), 3);
  }

  #[test]
  fn test_bad_add() {
    // This assert would fire and test will fail.
    // Please note, that private functions can be tested too!
    assert_eq!(bad_add(1, 2), 3);
  }
}
