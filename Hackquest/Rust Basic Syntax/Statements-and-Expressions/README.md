# Statements and Expressions

A practical guide to understanding statements vs expressions in Rust, with examples of vectors, tuples, and code blocks.

## Overview

This program demonstrates the fundamental difference between statements and expressions in Rust. Understanding this concept is essential for writing idiomatic Rust code.

## Key Concepts

### Statements vs Expressions

**Statement**: Performs an action but does not return a value. Ends with a semicolon.

**Expression**: Evaluates to a value and can be used in assignments.

```rust
let a = 1;  // Statement: binds value to variable

// This WILL NOT compile:
// let b = (let a = 1);
// Error: expected expression, found `let` statement
```

The error occurs because statements don't return values, so you cannot assign them to variables.

### Basic Expression Example

```rust
let y = {
    let x = 3;
    x + 1  // Expression without semicolon
};
println!("The value of y is: {}", y);  // y = 4
```

## Code Walkthrough

### Working with Vectors

```rust
// Creating an empty vector
let b: Vec<f64> = Vec::new();
println!("b is {:?}", b);  // Output: b is []

// Adding elements to a mutable vector
let mut b: Vec<f64> = Vec::new();
b.push(1.23);
b.push(9.0);
println!("{:?}", b);  // Output: [1.23, 9.0]
```

Vectors are dynamic arrays. The `Vec<f64>` means a vector that holds f64 floating-point values.

### Vector of Unit Type

```rust
let mut v: Vec<()> = Vec::new();
v.push(());
v.push(());
println!("{:?}", v);  // Output: [(), ()]
```

The unit type `()` represents an empty value.

### Tuple Destructuring

```rust
let (a, c) = ("hi", false);  // Tuple type
```

Tuples can hold multiple values of different types.

### Code Block Expressions

```rust
let x: i32 = 5;

let y = {
    let x = 999999;
    let x_squared = x * x;
    let x_cube = x_squared * x;
    
    // The value of the following expression will be assigned to `y`
    Option::Some(x_cube + x_squared + x)
};
println!("y is {:?}", y);  // y = 155
```

This code block creates its own scope with a shadowed `x` variable and returns the computed value.

### Handling Integer Overflow

The above calculation will overflow. Here's the safe version:

```rust
let x: i32 = 999_999;

let y = {
    let x_squared = x.checked_mul(x);  // Option<i32>
    let x_cube = x_squared.and_then(|v| v.checked_mul(x));
    
    // Now sum them safely
    let result = x_cube
        .and_then(|v| v.checked_add(x_squared.unwrap_or(0)))  // add x^2
        .and_then(|v| v.checked_add(x));  // add x
    
    result  // final Option<i32>
};

println!("y = {:?}", y);  // Output: y = None
```

Using `checked_mul` and `checked_add` returns `None` on overflow instead of causing a panic.

### Expression vs Statement in Blocks

```rust
let z = {
    // This is an expression that calculates the value of x+1 and returns
    x + 1
    
    // If you add a semicolon (;), it becomes a statement with no return value.
    // The default in Rust is "unit type ()", at this time z = ()
    // x + 1;
};
println!("z = {:?}", z);  // Output: z = 6
```

### If as an Expression

```rust
// The if statement block is also an expression, so it can be used for assignment
// Similar to the ternary operator, in Rust we can write it like this
let p = if x % 2 == 1 { "odd" } else { "even" };
```

## Working with Structs (Commented Example)

The code includes a commented example showing how to work with vectors of custom structs:

```rust
#[derive(Debug)]
struct User {
    id: u32,
    name: String,
}

fn main() {
    let mut users: Vec<User> = Vec::new();
    
    users.push(User {
        id: 1,
        name: "Alice".to_string(),
    });
    users.push(User {
        id: 2,
        name: "Bob".to_string(),
    });
    
    println!("Users: {}", users.len());  // Output: Users: 2
    println!("User 1: {} - {}", users[0].id, users[0].name);  // Output: User 1: 1 - Alice
    
    println!("{:#?}", users);  // Pretty-print with formatting
    println!("{:?}", users);   // Standard debug print
}
```

## Running the Program

```bash
cargo run
```

Expected output:

```
b is []
[1.23, 9.0]
y is Some(999997000001000999999)
z = 6
```

## Key Takeaways

1. Statements perform actions but don't return values
2. Expressions evaluate to values
3. Omitting the semicolon on the last line makes it an expression
4. Code blocks can create new scopes
5. Use `checked_*` methods to handle potential overflow safely
6. If blocks are expressions and can return values
7. Vectors must be declared `mut` to allow modifications