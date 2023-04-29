pub fn run_functions() {
    // Define a function that takes two i32 parameters and returns their sum.
    fn add_numbers(x: i32, y: i32) -> i32 {
        x + y
    }

    // Define a function that takes a vector of i32 values and returns their sum.
    fn sum_numbers(numbers: &[i32]) -> i32 {
        let mut sum = 0;
        for &number in numbers {
            sum += number;
        }
        sum
    }

    // Define a function that takes a mutable reference to an i32 value and increments it by 1.
    fn increment_number(number: &mut i32) {
        *number += 1;
    }

    fn decrement_number(num: &mut i64) {
        *num -= 1;
    }

    // Define a function that takes a closure and applies it to an i32 value.
    fn apply_closure<F>(number: i32, closure: F) -> i32
        where
            F: Fn(i32) -> i32,
    {
        closure(number)
    }

    fn new_closure<Frn>(number: i64, apply: Frn) -> i32
        where
            Frn: Fn(i64) -> i32
    {
        apply(number)
    }

// Call the add_numbers function with two i32 values.
    let result1 = add_numbers(10, 20);
    println!("Result 1: {}", result1);

// Call the sum_numbers function with a vector of i32 values.
    let numbers = vec![1, 2, 3, 4, 5];
    let result2 = sum_numbers(&numbers);
    println!("Result 2: {}", result2);

// Call the increment_number function with a mutable i32 value.
    let mut number = 5;
    increment_number(&mut number);
    println!("Number: {}", number);

// Call the apply_closure function with an i32 value and a closure that multiplies it by 2.
    let result3 = apply_closure(10, |x| x * 2);
    println!("Result 3: {}", result3);

    let result4 = new_closure(10, |x| (x / 2) as i32);
    println!("Result 3: {}", result4);

    main();
}


// A function that takes an i32 parameter by value
/*
add_one_v1 takes an i32 parameter x by value. This means that when the function is called,
a copy of the value of x is made and passed to the function. Any changes made to x inside the function will not be
reflected outside of the function. This is the simplest way to pass a parameter, but it can be inefficient if the value being passed
is large.
 */
fn add_one_v1(x: i32) -> i32 {
    x + 1
}


// A function that takes an i32 parameter by reference
/*
add_one_v2 takes an i32 parameter x by mutable reference. This means that when the function is called,
a reference to x is passed to the function, which allows the function to modify the original value of x.
The &mut syntax is used to indicate that the reference is mutable. This method of passing parameters is more
efficient than passing by value for large values.

 */

fn add_one_v2(x: &mut i32) {
    *x += 1;
}

// A function that takes a mutable reference to a vector
/*
add_to_vec_v1 takes a mutable reference to a vector v and an i32 parameter x.
This allows the function to modify the original vector by adding the value x to it.
This method of passing parameters is useful when the function needs to modify a complex data structure.
 */
fn add_to_vec_v1(v: &mut Vec<i32>, x: i32) {
    v.push(x);
}

// A function that takes ownership of a vector and returns it back
/*
add_to_vec_v2 takes ownership of a vector v and an i32 parameter x.
This means that when the function is called, the original vector is moved into the function and can no
longer be accessed outside of the function. The function then modifies the vector by adding the value x to it
and returns the modified vector. This method of passing parameters is useful when the function needs to
take ownership of a complex data structure and modify it.
 */
fn add_to_vec_v2(mut v: Vec<i32>, x: i32) -> Vec<i32> {
    v.push(x);
    v
}

fn main() {
    // Call add_one_v1 with a value
    let x = 5;
    let y = add_one_v1(x);
    println!("{} + 1 = {}", x, y);

    // Call add_one_v2 with a mutable reference
    let mut z = 5;
    add_one_v2(&mut z);
    println!("5 + 1 = {}", z);

    // Call add_to_vec_v1 with a mutable reference to a vector
    let mut v1 = vec![1, 2, 3];
    add_to_vec_v1(&mut v1, 4);
    println!("v1 = {:?}", v1);

    // Call add_to_vec_v2 with ownership of a vector
    let v2 = vec![1, 2, 3];
    let v2 = add_to_vec_v2(v2, 4);
    println!("v2 = {:?}", v2);
}
