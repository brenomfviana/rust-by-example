# Notes

## To remember
- `_` is used before variables, functions and methods to silence the compiler or to ignore a component in a destructure;
- Casting: `let integer = decimal as u8;`;
- Tuple assignment: `let (integer, boolean) = (1, true);`;
- Initialize array with the same values: `let ys: [i32; 500] = [0; 500];`;
- Define a multable variable: `let mut mutable_binding = 1;`;
- Aliasing: `type NanoSecond = u64;`;
- Labels for command loop: `'outer: loop {...`. To break: `break 'outer;`;
- Set a value from a loop directly: `let result = loop {...`;
- For: `for n in 1..101 {`;
- `match` is like C `switch`, but better;
  - We can define a range: `match {... n @ 1 ..= 12 => ...`;
- Diverging functions: functions with no return;
- Assignment: can copy, move or borrow;
  - Move: pointer;
  - Borrowing: variable by reference. Mutable variables freezes and can be mutable or immutable borrowing;


## POO Mapping
- `struct`: class;
  - To implement methods use: `impl <struct-name> { <functions>; }`
- Inheritance/Polymorphism: `traits`;
  - Defines functions which can be implemented by structs;
-

## Interesting features
- Destructures let the code cleaner and powerful (and they can be used in a lot of ways: `struct`, `match`, `ref` etc.);
- We can use `enum` to create linked lists;
- Labels for command loop: `'outer: loop`;
- Set a value from a loop directly: `let result = loop {...`;
- The `return` command can be omitted in the last line and without `;`;
- Closures: functions that can be stored by a variable;
  - Interesting example: `9-Functions/8-closures-std-i`;
  - As parameters: `9-Functions/7-closures-as-parameters.rs`;
- Higher order functions: `9-Functions/10-higher-order-functions.rs`.
- Phantom type may be very useful, mainly for metrics;
- Lifetimes can help to optimize the use of memory;
- A very powerful macro system. `// It is still to advanced for me.`


### Interesting traits
- From: `6-Conversion/0-from-and-into.rs`


## Still learning about
- crate;
- `Option<T>`;
- `Box<T>`: is a pointer to a heap allocated type;
- `if let Some(i) = number`;
- `while let Some(i) = number`;
- Generics;
  - `where`;
  - Associated types;
- Ord and operators traits;
- A very powerfull macro system.


## Printing
- `1-HelloWorld/3-prints.rs`
