// To check functions that should panic under certain circumstances, use
// attribute #[should_panic]. This attribute accepts optional parameter
// expected = with the text of the panic message. If your function can panic in
// multiple ways, it helps make sure your test is testing the correct panic.
pub fn divide_non_zero_result(a: u32, b: u32) -> u32 {
  if b == 0 {
    panic!("Divide-by-zero error");
  } else if a < b {
    panic!("Divide result is zero");
  }
  a / b
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_divide() {
    assert_eq!(divide_non_zero_result(10, 2), 5);
  }

  #[test]
  #[should_panic]
  fn test_any_panic() {
    divide_non_zero_result(1, 0);
  }

  #[test]
  #[should_panic(expected = "Divide result is zero")]
  fn test_specific_panic() {
    divide_non_zero_result(1, 10);
  }
}
