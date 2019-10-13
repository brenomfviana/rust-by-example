// https://doc.rust-lang.org/rust-by-example/fn/closures/output_parameters.html
// The book has a deprecated code as example of closures as output

fn create_fn() -> Box<dyn Fn()> {
  let text = "Fn".to_owned();
  Box::new(move || println!("This is a: {}", text))
}

fn create_fnmut() -> Box<dyn FnMut()> {
  let text = "FnMut".to_owned();
  Box::new(move || println!("This is a: {}", text))
}

fn main() {
  let fn_plain = create_fn();
  let mut fn_mut = create_fnmut();
  fn_plain();
  fn_mut();
}
