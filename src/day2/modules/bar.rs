// File: bar.rs

// Define a struct called "Person"
pub struct Person {
    name: String,
    age: u32
}

impl Person {
    // Define a public method called "new"
    pub fn new(name: &str, age: u32) -> Person {
        Person {
            name: name.to_string(),
            age: age,
        }
    }

    // Define a public method called "name"
    pub fn name(&self) -> &str {
        &self.name
    }

    // Define a public method called "age"
    pub fn age(&self) -> u32 {
        self.age
    }
}
