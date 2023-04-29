use std::fs::File;
use std::io::{self, BufRead, BufReader};


/*
Generics allow you to write code that can work with different types. Here is an example of a generic function:
 */

/*
This function takes a slice of any type T that implements the PartialOrd trait (which means it can be compared to other values of its type).
The function returns an Option<&T> (an option containing a reference to a value of type T), which will be None if the slice is empty.

The function finds the maximum value in the slice by iterating over each item and comparing it to the current maximum value.
If an item is greater than the current maximum, it becomes the new maximum.
 */


fn find_max<T: PartialOrd>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        return None;
    }

    let mut max = &list[0];

    for item in list {
        if item > max {
            max = item;
        }
    }

    Some(max)
}

struct Pair<T, U> {
    first: T,
    second: U,
}

trait Area {
    fn area(&self) -> f64;
}

struct Rectangle<T> {
    width: T,
    height: T,
}

impl<T: std::ops::Mul<Output = T> + Copy + Into<f64>> Area for Rectangle<T> {
    fn area(&self) -> f64 {
        let w: f64 = self.width.into();
        let h: f64 = self.height.into();
        w * h
    }
}

struct Circle<T> {
    radius: T,
}

impl<T: std::ops::Mul<Output = T> + Copy + Into<f64>> Area for Circle<T> {
    fn area(&self) -> f64 {
        let r: f64 = (self.radius).into();
        std::f64::consts::PI * r * r
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &item in list.iter() {
        if item > max {
            max = item;
        }
    }
    max
}

enum MyError<T> {
    Io(io::Error),
    Parse(T),
}

impl<T> From<io::Error> for MyError<T> {
    fn from(error: io::Error) -> Self {
        MyError::Io(error)
    }
}

fn read_numbers(filename: &str) -> Result<Vec<i32>, MyError<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut numbers = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let number = line.parse().map_err(MyError::Parse)?;
        numbers.push(number);
    }
    Ok(numbers)
}

pub fn execute_generics() {
    // 1. Writing a generic function to find the largest element in a vector:

    let list = vec![4, 2, 8, 1, 5];
    match find_max(&list) {
        Some(max) => println!("Max value: {}", max),
        None => println!("List is empty"),
    }

    let list = vec!["apple", "banana", "orange"];
    match find_max(&list) {
        Some(max) => println!("Max value: {}", max),
        None => println!("List is empty"),
    }

    //2. Writing a generic struct to store values of different types:

    let pair1 = Pair { first: 5, second: "hello" };
    let pair2 = Pair { first: 3.14, second: true };

    println!("pair1: ({}, {})", pair1.first, pair1.second);
    println!("pair2: ({}, {})", pair2.first, pair2.second);

    //3. Writing a generic trait to define behavior for different types:
    let rectangle = Rectangle { width: 10.0, height: 20.0 };
    println!("Rectangle area: {}", rectangle.area());

    let circle = Circle { radius: 5 };
    println!("Circle area: {}", circle.area());

    //Algorithms
    let mut data = vec![5, 2, 7, 1, 8, 4];
    data.sort();
    println!("Sorted data: {:?}", data);

    let mut names = vec!["Alice", "Bob", "Charlie", "Dave"];
    names.sort();
    println!("Sorted names: {:?}", names);

    //Functions

    let integers = vec![10, 20, 30, 5, 15];
    let largest_integer = largest(&integers);
    println!("Largest integer: {}", largest_integer);

    let floats = vec![1.5, 2.7, 0.5, 4.1, 3.8];
    let largest_float = largest(&floats);
    println!("Largest float: {}", largest_float);

    let strings = vec!["cat", "dog", "bird", "fish", "ant"];
    let largest_string = largest(&strings);
    println!("Largest string: {}", largest_string);

    //Error Handling
    let result = read_numbers("src/day1/data/numbers.txt");
    match result {
        Ok(numbers) => println!("Read numbers: {:?}", numbers),
        Err(error) => match error {
            MyError::Io(io_error) => println!("IO error: {}", io_error),
            MyError::Parse(parse_error) => println!("Parse error: {}", parse_error),
        },
    }

}

/*
Uses of Generics

Data structures:
Generics can be used to create data structures such as linked lists, binary trees, hash tables, and more. By using generics, these data structures can be designed to store any type of data. For example, a linked list can be created to store integers or strings, or any other type of data, without needing to write separate code for each type.

Collections:
Rust's standard library provides a collection of generic data structures such as Vec, HashMap, HashSet, etc. These collections can store any type of data, making them very useful when working with dynamic or heterogeneous data.

Algorithms:
Generics are very useful in writing algorithms that can operate on multiple types of data. For example, Rust's standard library provides the sort() function which can be used to sort any type of data that implements the Ord trait.

Functions:
Generics can be used in functions to make them more flexible and reusable. For example, consider a function that finds the largest element in a vector of integers. This function can be made generic to work with any type that implements the PartialOrd trait, allowing it to be used with other types such as floating-point numbers or strings.

Error handling:
Generics can be used in error handling to create error types that can hold any type of data. For example, Rust's standard library provides the Result type which is generic over two types: the Ok value and the Err value. This allows errors to be returned with any type of data.
 */