fn main() {
  // Integer value
  let an_integer = 1u32;
  // copy `an_integer` into `copied_integer`
  let copied_integer = an_integer;
  println!("An integer: {:?}", an_integer);
  println!("The copied integer: {:?}", copied_integer);

  // Boolean value
  let a_boolean = true;
  println!("A boolean: {:?}", a_boolean);

  // Empty tuple
  let unit = ();
  println!("Meet the unit value: {:?}", unit);


  // The compiler warns about unused variable bindings; these warnings can
  // be silenced by prefixing the variable name with an underscore
  let _unused_variable = 3u32;
  let _noisy_unused_variable = 2u32; // Old: let noisy_unused_variable = 2u32;
  // FIXME ^ Prefix with an underscore to suppress the warning
}
