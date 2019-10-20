// Unit tests are testing one module in isolation at a time: they're small and
// can test private code. Integration tests are external to your crate and use
// only its public interface in the same way any other code would. Their
// purpose is to test that many parts of your library work correctly together.
// Cargo looks for integration tests in tests directory next to src.

// File src/lib.rs:

// Assume that crate is called adder, will have to extern it in integration
// test.
pub fn add(a: i32, b: i32) -> i32 {
  a + b
}

// Each Rust source file in tests directory is compiled as a separate crate.
// One way of sharing some code between integration tests is making module with
// public functions, importing and using it within tests.

// File tests/common.rs:
pub fn setup() {
  // some setup code, like creating required files/directories, starting
  // servers, etc.
}

// File with test: tests/integration_test.rs

// extern crate we're testing, same as any other code will do.
// extern crate adder; // This is commented because the crate does not exists

// importing common module.
// mod common; // This is commented because the module does not exists

#[test]
fn test_add() {
  // using common code.
  common::setup();
  assert_eq!(adder::add(3, 2), 5);
}
