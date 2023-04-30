/*
In this example, we define a function print_length that takes a reference to a String as its argument.
By passing a reference instead of the String itself, we are borrowing the String instead of taking ownership of it.
This allows us to use the String without transferring ownership to the function.
 */
//Borrowing
pub fn execute_borrowing() {
    fn print_length(s: &String) {
        println!("{}", s.len());
    }

    let s = String::from("hello");
    print_length(&s); // we borrow s by passing a reference to the function
    println!("{}", s); // we can still use s as it has not been moved
}