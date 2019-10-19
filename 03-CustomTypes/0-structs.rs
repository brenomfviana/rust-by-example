// There are three types of structures ("structs") that can be created using the struct keyword:
//   Tuple structs, which are, basically, named tuples.
//   The classic C structs
//   Unit structs, which are field-less, are useful for generics.
use std::fmt; // Import `fmt`

#[derive(Debug)]
struct Person<'a> {
  name: &'a str,
  age: u8,
}

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
  x: f32,
  y: f32
}

impl fmt::Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

// Structs can be reused as fields of another struct
#[allow(dead_code)] #[derive(Debug)]
struct Rectangle {
  p1: Point,
  p2: Point
}

impl fmt::Display for Rectangle {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[{}, {}]", self.p1, self.p2)
  }
}

fn rect_area(r: Rectangle) -> f32 {
  let width = (r.p1.x - r.p2.x).abs();
  let heigth = (r.p1.y - r.p2.y).abs();
  (width * heigth)
}

fn square(p: Point, s: f32) -> Rectangle {
  let p2 = Point { x: p.x + s, y: p.y + s };
  Rectangle { p1: p, p2: p2 }
}

fn main() {
  // Create struct with field init shorthand
  let name = "Peter";
  let age = 27;
  let peter = Person { name, age };
  // Print debug struct
  println!("{:?}", peter);

  // Instantiate a `Point`
  let point: Point = Point { x: 0.3, y: 0.4 };
  // Access the fields of the point
  println!("point coordinates: ({}, {})", point.x, point.y);

  // Make a new point by using struct update syntax to use the fields of our other one
  let new_point1 = Point { x: 0.1, ..point };
  // `new_point.y` will be the same as `point.y` because we used that field from `point`
  println!("second point: ({}, {})", new_point1.x, new_point1.y);
  // Copy the point
  let new_point2 = Point { ..point };
  println!("third point: ({}, {})", new_point2.x, new_point2.y);

  // Destructure the point using a `let` binding
  let Point { x: my_x, y: my_y } = point;

  println!("Build a square: {}", square(Point { x: my_x, y: my_y }, 1.0));

  let _rectangle = Rectangle {
    // struct instantiation is an expression too
    p1: Point { x: my_y, y: my_x },
    p2: point
  };
  println!("The rectangle area is: {}", rect_area(_rectangle));

  // Instantiate a unit struct
  let _nil = Nil;

  // Instantiate a tuple struct
  let pair = Pair(1, 0.1);

  // Access the fields of a tuple struct
  println!("pair contains {:?} and {:?}", pair.0, pair.1);

  // Destructure a tuple struct
  let Pair(integer, decimal) = pair;

  println!("pair contains {:?} and {:?}", integer, decimal);
}
