// To compile use: rustc --cfg some_condition ./3-cfg-custom.rs

#[cfg(some_condition)]
fn conditional_function() {
  println!("condition met!");
}

fn main() {
  conditional_function();
}
