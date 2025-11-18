#![allow(unused)]

// Expression as a function that returns a value
fn max_plus_one(x: i32, y: i32) -> i32 {
     // In Rust, `if-else` is an expression. The value of the executed
     // block becomes the value of the entire `if-else` expression.
     // Since this is the last expression in the function, its value is returned.
     if x > y {
         x + 1
     } else {
         y + 1
     }
}

// Unit type () as a function that returns a value
// This function does not have an explicit return value type, and Rust returns the unit type () by default
fn print_hello() {
     // This is a statement, not an expression
     println!("hello");
}

// A divergent function that never returns, marked with !
fn diverging() -> ! {
     // Throw a panic exception and terminate the program.
     panic!("This function will never return!");
}   