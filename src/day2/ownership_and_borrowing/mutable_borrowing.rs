/*
In this example, we define a function modify that takes a mutable reference to a String as its argument.
By passing a mutable reference, we can modify the String without transferring ownership. When we borrow mutably, we can only have one mutable reference at a time to a particular piece of data.
This ensures that there are no race conditions or data races when multiple threads are accessing the same data.
 */
pub fn execute_mutable_borrowing() {
    fn modify(s: &mut String) {
        s.push_str(", world!");
    }

    let mut s = String::from("hello");
    modify(&mut s); // we borrow s mutably by passing a mutable reference to the function
    println!("{}", s); // "hello, world!" will be printed

}