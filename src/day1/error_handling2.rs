use std::fs::File;               // Import the standard library's file module.
use std::io::{self, BufRead, BufReader, Error as IoError};    // Import the standard library's IO module.
use std::error::Error;           // Import the standard library's Error module.
use std::fmt;                   // Import the standard library's fmt module.

// Define a custom error struct called MyError that contains a message.
#[derive(Debug)]
struct MyError {
    message: String // A string message describing the error.
}

// Implement the fmt::Display trait for MyError to allow it to be printed.
// This trait allows the error to be displayed in a human-readable format.
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

// Implement the Error trait for MyError to provide additional functionality.
// This trait allows the error to be used as a standard error type.
impl Error for MyError {}

// A function that returns a Result containing an i32 or a custom MyError.
// This function divides two numbers and returns the result if the divisor is not zero.
// If the divisor is zero, it returns a custom MyError with a message.
fn divide(x: i32, y: i32) -> Result<i32, MyError> {
// Check if y is zero, return a MyError if it is.
    if y == 0 {
        return Err(MyError { message: "cannot divide by zero".to_string() });
    }
// If y is not zero, return the result of x/y wrapped in a Result.
    Ok(x / y)
}

// A function that uses the ? operator to propagate errors.
// This function reads a file and prints each line to the console.
// It returns an empty Ok value if it completes successfully, or a MyError if an error occurs.
fn read_file() -> Result<(), MyError> {
// Open a file named "file.txt", return an error if the file cannot be opened.
    let file = File::open("src/day1/data/file.txt")?;
// Create a buffered reader for the file.
    let reader = BufReader::new(file);
// Iterate over each line in the reader.
    for line in reader.lines() {
// Get the line, return an error if an error occurs.
        let line = line?;
// Print the line.
        println!("{}", line);
    }
// Return an empty Ok value.
    Ok(())
}

// A function that handles errors with match statements.
// This function reads user input from the console and prints it.
// It returns an empty Ok value if it completes successfully, or a MyError if an error occurs.
fn read_user_input() -> Result<(), MyError> {
// Create a mutable String to hold user input.
    let mut input = String::new();
// Read a line from stdin into input, returning an error if one occurs.
    match io::stdin().read_line(&mut input) {
// If the read succeeds, print the input and return an empty Ok value.
        Ok(_) => {
            println!("You entered: {}", input.trim());
            Ok(())
        }
// If the read fails, return a MyError with a message including the error.
        Err(error) => Err(MyError { message: format!("error reading user input: {}", error) })
    }
}

// Implement the From trait for MyError to allow it to be created from an io::Error.
// This is useful for converting errors from other libraries into custom errors.
impl From<io::Error> for MyError {
    fn from(error: io::Error) -> Self {
        MyError { message: format!("IO error: {}", error) }
    }
}


// The main function.
pub fn execute_error_handling2() -> Result<(), Box<dyn Error>> {
// Handling errors with match statements.
    let result = divide(10, 0);
    match result {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error)
    }
    // Propagating errors with the ? operator.
    read_file()?;

// Handling errors with the try! macro.
    let file = File::open("src/day1/data/file.txt").map_err(|error| MyError { message: format!("error opening file: {}", error) })?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.map_err(|error: IoError| MyError { message: format!("error reading line: {}", error) })?;
        println!("{}", line);
    }

// Handling errors with the ? operator in combination with the try! macro.
    read_user_input()?;

// Return Ok(()) to indicate that the program completed successfully.
    Ok(())
}