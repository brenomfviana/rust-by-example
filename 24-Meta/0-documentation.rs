// Use cargo doc to build documentation in target/doc.
// Use cargo test to run all tests (including documentation tests), and cargo
// test --doc to only run documentation tests.
// These commands will appropriately invoke rustdoc (and rustc) as required.
#![crate_name = "doc"]

/// A human being is represented here
pub struct Person {
  /// A person must have a name, no matter how much Juliet may hate it
  name: String,
}

impl Person {
  /// Returns a person with the name given them
  ///
  /// # Arguments
  ///
  /// * `name` - A string slice that holds the name of the person
  ///
  /// # Example
  ///
  /// ```
  /// // You can have rust code between fences inside the comments
  /// // If you pass --test to Rustdoc, it will even test it for you!
  /// use doc::Person;
  /// let person = Person::new("name");
  /// ```
  pub fn new(name: &str) -> Person {
    Person {
      name: name.to_string(),
    }
  }

  /// Gives a friendly hello!
  ///
  /// Says "Hello, [name]" to the `Person` it is called on.
  pub fn hello(& self) {
    println!("Hello, {}!", self.name);
  }
}

fn main() {
  let john = Person::new("John");
  john.hello();
}

// Run:
// rustc 0-documentation.rs [--crate-type lib]
// rustdoc --test --extern doc="libdoc.rlib" 0-documentation.rs
