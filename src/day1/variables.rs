// Declare a mutable variable
// let mut variable_name = value;
//
// // Declare an immutable variable
// let variable_name = value;
//
// // Declare a constant
// const CONSTANT_NAME: data_type = value;

//declaring variables and constants in Rust
#[warn(dead_code)]
pub fn run() {
    // Declare an immutable variable
    let x = 5;

    // Declare a mutable variable
    let mut y = 10;

    // Print the value of x and y
    println!("x = {}, y = {}", x, y);

    // Change the value of y
    y = 15;

    // Print the new value of y
    println!("New value of y = {}", y);

    // This will give an error because x is immutable
    // x = 6;

    constant_example();
}

//difference between mutable and immutable variables
#[warn(dead_code)]
fn constant_example() {
    // Declare a constant
    const MAX_VALUE: i64 = 9223372036854775807;

    // Print the value of the constant
    println!("Max value is {}", MAX_VALUE);

    // This will give an error because we cannot change the value of a constant
    // MAX_VALUE = 200;
}


