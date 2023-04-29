/*
Ownership: Rust's ownership model ensures that there is always one and only one owner of a piece of data at any given time.
This is demonstrated in the takes_ownership() function, which takes ownership of the string variable by moving it into the function.
Once ownership is transferred to the function, the main function can no longer access the value.

Borrowing: To allow for temporary access to a value, Rust provides the concept of borrowing.
This is demonstrated in the borrows_string() function, which borrows the string variable by taking a reference to it (&).
This allows the function to read the value without taking ownership of it, so the main function can still access the
value after the function call.

Mutable Borrowing: In addition to borrowing a value for reading, Rust also allows borrowing a value for writing.
This is demonstrated in the mutates_string() function, which borrows the string variable mutably (&mut).
This allows the function to modify the value, but the main function still retains ownership of the variable.

Ownership Transfer with Return Values: Ownership can also be transferred out of a function by returning the value.
This is demonstrated in the gives_ownership() function, which creates a new String value and returns it,
transferring ownership to the calling function.

Borrowing with Return Values: Similarly, a function can return a reference to a value without transferring ownership.
This is demonstrated in the borrows_with_return() function, which borrows the String value and returns a reference to it.

Lifetime Annotations: Rust uses lifetime annotations to ensure that borrowed values do not outlive the values they reference.
This is demonstrated in the longest_string() function, which takes two string references as arguments and
returns a reference to the longest string. The lifetime annotation 'a indicates that the returned reference is
guaranteed to be valid for at least as long as the shorter of the two input references.
 */
pub fn execute_ownership_and_borrowing() {
    // Create a new string called `s1`.
    let mut s1 = String::from("hello");

    // Pass ownership of `s1` to a function, which then returns ownership back to `main()`.
    s1 = takes_ownership(s1);

    // Create a new integer called `x`.
    let x = 5;

    // Borrow `x` to a function, which prints its value but does not take ownership.
    borrows_immutable(&x);

    // Borrow `s1` mutably to a function, which modifies its value but does not take ownership.
    borrows_mutable(&mut s1);

    // Borrow `s1` immutably to a function, which prints its value but does not take ownership.
    borrows_immutable_str(&s1);

    // Create a new string called `s2`.
    let s2 = String::from("world");

    // Pass a reference of `s2` to a function, which borrows it immutably.
    borrows_string(&s2);

    // let mut s2 = String::from("world");

    // // Pass a reference of `s2` to a function, which borrows it mutably.
    // borrows_string_mut(&mut s2);

    // Try to borrow `s2` again, but this time mutably, which will fail because `s2` has already been borrowed mutably.
    // Uncomment the line below to see the compiler error.
    // borrows_string_mut(&mut s2);

    // Create a new string called `s3`.
    let s3 = String::from("foo");

    // Pass ownership of `s3` to a function, which returns a tuple with ownership of `s3` and a new string called `s4`.
    let (s3, s4) = takes_and_gives_back(s3);

    // Try to use `s3` again, but this time it will fail because ownership has been moved to `takes_and_gives_back()`.
    // Uncomment the line below to see the compiler error.
    // println!("{}", s3);

    // Print `s4`, which now has ownership.
    println!("{}", s4);
}

// This function takes ownership of a string and returns ownership back to the caller.
fn takes_ownership(s: String) -> String {
    println!("{}", s);
    s + " world"
}

// This function borrows an immutable reference to an integer, but does not take ownership.
fn borrows_immutable(i: &i32) {
    println!("{}", i);
}

fn borrows_immutable_str(i: &String) {
    println!("{}", i);
}

// This function borrows a mutable reference to a string, modifies it, but does not take ownership.
fn borrows_mutable(s: &mut String) {
    s.push_str(" world");
    println!("{}", s);
}

// This function borrows an immutable reference to a string, but does not take ownership.
fn borrows_string(s: &String) {
    println!("{}", s);
}

// This function borrows a mutable reference to a string, but does not take ownership.
fn borrows_string_mut(s: &mut String) {
    s.push_str(" world");
    println!("{}", s);
}

// This function takes ownership of a string and returns ownership of that string and a new string.
fn takes_and_gives_back(s: String) -> (String, String) {
    (s, String::from("bar"))
}
