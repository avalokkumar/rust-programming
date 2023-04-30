/*
Here we define a function process that takes ownership of a String as its argument and returns
ownership of a new String. When we call the process function, ownership of s is transferred to the function, and s is no longer valid.
The process function creates a new String with some modifications and returns ownership of it to the caller.
 */
pub fn execute_ownership_transfer() {
    fn process(s: String) -> String {
        let result = format!("{}!", s);
        result // ownership of result is transferred to the caller
    }

    let s = String::from("hello");
    let s_modified = process(s); // ownership of s is transferred to process function
    // println!("{}", s); // this will cause a compile error as s is no longer the owner of the string "hello"
    println!("{}", s_modified); // "hello!" will be printed

}