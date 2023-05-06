use crate::input_u32;

pub struct Menu {}

impl Menu {
    pub fn new() -> Self {
        Menu {}
    }

    pub fn print_menu(&self) {
        println!("Menu:");
        println!("1. Add Song");
        println!("2. Play");
        println!("3. Pause");
        println!("4. Stop");
        println!("5. Exit");
    }

    pub fn get_choice(&self) -> u32 {
        input_u32("Enter your choice: ")
    }
}