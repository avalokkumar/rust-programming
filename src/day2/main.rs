// File: main.rs

// mod modules {
//     pub(crate) mod foo;
//     pub(crate) mod bar;
// }
//
// mod standard_library {
//     pub(crate) mod library_usage;
// }

mod ownership_and_borrowing {
    // pub(crate) mod ownership;
    // pub(crate) mod borrowing;
    // pub(crate) mod mutable_borrowing;
    // pub(crate) mod ownership_transfer;
    // pub(crate) mod borrowing_mutable_reference;
    // pub(crate) mod mutable_borrowing_struct_field;
    pub(crate) mod borrowing_with_slices;
}

// This line tells Rust to load the module "foo" from the file "foo.rs"
// use modules::foo;
// This line tells Rust to load the module "bar" from the file "bar.rs"
// use modules::bar;

// use ownership_and_borrowing::ownership;
// use crate::ownership_and_borrowing::borrowing;
// use crate::ownership_and_borrowing::mutable_borrowing;
// use crate::ownership_and_borrowing::ownership_transfer;
// use crate::ownership_and_borrowing::borrowing_mutable_reference;
// use crate::ownership_and_borrowing::mutable_borrowing_struct_field;
// use crate::ownership_and_borrowing::immutable_borrowing_struct_field;
use crate::ownership_and_borrowing::borrowing_with_slices;

// use standard_library::library_usage;

// This line brings the trait std::io::Write into scope, so we can use it later
// use std::io::Write;

fn main() {
    // run_modules();
    // library_usage::execute_library_usage();
    run_ownership_and_borrowing();
}

fn run_ownership_and_borrowing() {
// ownership::execute_ownership();
    // borrowing::execute_borrowing();
    // mutable_borrowing::execute_mutable_borrowing();
    // ownership_transfer::execute_ownership_transfer();
    // borrowing_mutable_reference::execute_borrowing_mutable_reference();
    // mutable_borrowing_struct_field::execute_mutable_borrowing_struct_field();
    // immutable_borrowing_struct_field::execute_immutable_borrowing_struct_field();
    borrowing_with_slices::execute_borrowing_with_slices();
}

fn run_modules() {
    // Call the greet() function from the "foo" module
    foo::greet();

    // Create a new person and print their name and age
    let person = bar::Person::new("Alice", 25);
    println!("Name: {}", person.name());
    println!("Age: {}", person.age());

    // Write a message to the standard error stream
    writeln!(std::io::stderr(), "An error occurred").unwrap();
}