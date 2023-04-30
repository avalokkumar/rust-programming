
/*
Here, we create a string slice of s with the first word. This allows us to use a reference to a
portion
of the original string without copying the entire string.
 */


// Define a public function `execute_borrowing_with_slices`
pub fn execute_borrowing_with_slices() {
    // Create a new `String` variable `s` containing "hello world"
    let s = String::from("hello world");

    // Call the `first_word` function, passing a reference to `s`
    // and assign the result to the `word` variable
    let word = first_word(&s);

    // Print the first word found by the `first_word` function
    println!("The first word is: {}", word);
}

// Define a private function `first_word` that takes a reference to a string slice (`&str`) as an argument and returns a reference to a string slice
fn first_word(s: &str) -> &str {
    // Convert the string slice to a byte slice (`&[u8]`) using `as_bytes` method
    let bytes = s.as_bytes();

    // Iterate over the byte slice with the `iter()` method, and enumerate each item (byte) with its index
    for (i, &item) in bytes.iter().enumerate() {
        // If a space is found (ASCII code 32), return a reference to the string slice up to that point (`..i`)
        if item == b' ' {
            return &s[..i];
        }
    }

    // If no space is found, return a reference to the entire string slice (`&s[..]`)
    &s[..]
}