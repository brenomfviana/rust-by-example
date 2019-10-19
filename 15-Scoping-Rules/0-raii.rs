// Variables in Rust do more than just hold data in the stack: they also own
// resources, e.g. Box<T> owns memory in the heap. Rust enforces RAII (Resource
// Acquisition Is Initialization), so whenever an object goes out of scope, its
// destructor is called and its owned resources are freed.
//
// This behavior shields against resource leak bugs, so you'll never have to
// manually free memory or worry about memory leaks again! Here's a quick
// showcase:

// raii.rs
fn create_box() {
  // Allocate an integer on the heap
  let _box1 = Box::new(3i32);

  // `_box1` is destroyed here, and memory gets freed
}

fn main() {
  // Allocate an integer on the heap
  let _box2 = Box::new(5i32);

  // A nested scope:
  {
    // Allocate an integer on the heap
    let _box3 = Box::new(4i32);

    // `_box3` is destroyed here, and memory gets freed
  }

  // Creating lots of boxes just for fun
  // There's no need to manually free memory!
  for _ in 0u32..1_000 {
    create_box();
  }

  // `_box2` is destroyed here, and memory gets freed
}
