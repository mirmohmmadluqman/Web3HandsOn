// fn main() {
//     // Statement: use the 'let' keyword to create a variable and bind it to a value
//     let a = 1;

//     // Statement: since statements do not return a value, attempting to bind a statement (let a = 1) to variable b will result in a compilation error
//     //
//     //     error: expected expression, found `let` statement
//     //  --> src/main.rs:6:14 // But now it is 14:14
//     //   |
//     // 6 |     let b = (let a = 1);
//     //   |              ^^^
//     //   |
//     //   = note: only supported directly in conditions of `if` and `while` expressions
//     // let b = (let a = 1);
//     // println!("The value of b is: {}", b); // This line will not compile
//     //                                ^ help: a local variable with a similar name exists: `a`

//     // Expression: the return value is x + 1
//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is: {}", y); // y = 4
//     //     mirmohmmadluqman@Lenovo-ThinkPad-P1:/mnt/c/Users/Mir Mohmmad Luqman/********/Web3HandsOn/Hackquest/Rust Basic Syntax/Statements-and-Expressions$ cargo run
//     //    Compiling Statements-and-Expressions v0.1.0 (/mnt/c/Users/Mir Mohmmad Luqman/********/Trath/Web3HandsOn/Hackquest/Rust Basic Syntax/Statements-and-Expressions)
//     // warning: unused variable: `a`
//     //  --> src/main.rs:3:9
//     //   |
//     // 3 |     let a = 1;
//     //   |         ^ help: if this is intentional, prefix it with an underscore: `_a`
//     //   |
//     //   = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

//     // warning: `Statements-and-Expressions` (bin "Statements-and-Expressions") generated 1 warning
//     //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.05s
//     //      Running `target/debug/Statements-and-Expressions`
//     // The value of y is: 4
// }

fn main() {
    //The following 4 are statements
    let a = 1;
    let b: Vec<f64> = Vec::new(); // vec means creating a dynamic array of type f64
    println!("b is {:?}", b);
    //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.89s
    //      Running `target/debug/Statements-and-Expressions`
    // b is []      <--
    // y is 155
    // z = 6

    let mut b: Vec<f64> = Vec::new();
    b.push(1.23);
    b.push(9.0);
    println!("{:?}", b);
    //         Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.06s
    //      Running `target/debug/Statements-and-Expressions`
    // b is []
    // [1.23, 9.0] <--
    // y is 155
    // z = 6

    // Vector of Unit type `Vec<()>`
    // fn main() {
    // let mut v: Vec<()> = Vec::new();
    // v.push(());   // only () allowed
    // v.push(());

    // println!("{:?}", v);  // [(), ()]
    // }

    let (a, c) = ("hi", false); // Tuple type
    let x: i32 = 5;

    // This is a code block expression
    let y = {
        let x = 999999;
        let x_squared = x * x;
        let x_cube = x_squared * x;

        //The value of the following expression will be assigned to `y`
        Option::Some(x_cube + x_squared + x)
    };
    println!("y is {:?}", y); // y = 155

    // It will overflow therefore use this:
    // ```
    //     fn main() {
    //     let x: i32 = 999_999;

    //     let y = {
    //         let x_squared = x.checked_mul(x);       // Option<i32>
    //         let x_cube = x_squared.and_then(|v| v.checked_mul(x));

    //         // Now sum them safely
    //         let result = x_cube
    //             .and_then(|v| v.checked_add(x_squared.unwrap_or(0)))  // add x^2
    //             .and_then(|v| v.checked_add(x));                      // add x

    //         result   // final Option<i32>
    //     };

    //     println!("y = {:?}", y);
    // }
    // ```
    // Correct Beginner Example to Prevent overflow
    // ```
    //         Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.13s
    //      Running `target/debug/Statements-and-Expressions`
    // y = None <--

    let z = {
        // This is an expression that calculates the value of x+1 and returns
        x + 1

        // If you add a semicolon (;), it becomes a statement with no return value.
        // The default in Rust is "unit type ()", at this time z = ()
        // x + 1;
    };
    println!("z = {:?}", z);

    // The if statement block is also an expression, so it can be used for assignment or return directly
    // Similar to the ternary operator, in Rust we can write it like this
    let p = if x % 2 == 1 { "odd" } else { "even" };
}

// #[derive(Debug)]
// struct User {
//     id: u32,
//     name: String,
// }

// fn main() {
//     let mut users: Vec<User> = Vec::new();

//     users.push(User {
//         id: 1,
//         name: "Alice".to_string(),
//     });
//     users.push(User {
//         id: 2,
//         name: "Bob".to_string(),
//     });

//     println!("Users: {}", users.len()); // 2
//     //         Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.43s
//     //      Running `target/debug/Statements-and-Expressions`
//     // Users: 2 <--

//     println!("User 1: {} - {}", users[0].id, users[0].name);
//     //         Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.17s
//     //      Running `target/debug/Statements-and-Expressions`
//     // Users: 2
//     // User 1: 1 - Alice <--

//     println!("{:#?}", users);
//     //         Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.52s
//     //      Running `target/debug/Statements-and-Expressions`
//     // Users: 2
//     // User 1: 1 - Alice
//     // [
//     //     User {
//     //         id: 1,
//     //         name: "Alice",
//     //     },
//     //     User {
//     //         id: 2,
//     //         name: "Bob",
//     //     },
//     // ]
//     // All this ^^^ <--

//     println!("{:?}", users);
//     //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.72s
//     //      Running `target/debug/Statements-and-Expressions`
//     // Users: 2
//     // User 1: 1 - Alice
//     // [
//     //     User {
//     //         id: 1,
//     //         name: "Alice",
//     //     },
//     //     User {
//     //         id: 2,
//     //         name: "Bob",
//     //     },
//     // ]
//     // [User { id: 1, name: "Alice" }, User { id: 2, name: "Bob" }] <--
// }

// // Note: I wrote it in this way because I want to see how it looks like that...
// //       and can this be useful
