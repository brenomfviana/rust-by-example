// As an introduction to this section, to borrow from the official docs, "one
// should try to minimize the amount of unsafe code in a code base." With that
// in mind, let's get started! Unsafe annotations in Rust are used to bypass
// protections put in place by the compiler; specifically, there are four
// primary things that unsafe is used for:
//   - dereferencing raw pointers
//   - calling functions or methods which are unsafe (including calling a
//     function over FFI, see a previous chapter of the book)
//   - accessing or modifying static mutable variables
//   - implementing unsafe traits

use std::slice;

fn main() {
  // Raw Pointers
  // Raw pointers * and references &T function similarly, but references are
  // always safe because they are guaranteed to point to valid data due to the
  // borrow checker. Dereferencing a raw pointer can only be done through an
  // unsafe block.
  let raw_p: *const u32 = &10;
  unsafe {
    assert!(*raw_p == 10);
  }

  // Calling Unsafe Functions
  // Some functions can be declared as unsafe, meaning it is the programmer's
  // responsibility to ensure correctness instead of the compiler's. One example
  // of this is std::slice::from_raw_parts which will create a slice given a
  // pointer to the first element and a length.

  let some_vector = vec![1, 2, 3, 4];

  let pointer = some_vector.as_ptr();
  let length = some_vector.len();

  unsafe {
    let my_slice: &[u32] = slice::from_raw_parts(pointer, length + 5);
    println!("{:?}", my_slice);
    // assert_eq!(some_vector.as_slice(), my_slice);
  }
}
