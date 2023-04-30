

/*
In this example, the String type represents a heap-allocated string. When s is created, it is the owner of the string "hello".
When t is assigned the value of s, ownership of the string is transferred to t, and s is no longer valid.
If we try to use s again, we get a compile error because it is no longer the owner of the string.
 */
//Ownership
#[allow(dead_code)]
pub fn execute_ownership() {
    let s = String::from("hello"); // s owns the string "hello"
    let p = s.clone();
    let t = s; // ownership of s is transferred to t
    println!("{}", t); // "hello" will be printed
    // println!("{}", s); // this will cause a compile error as s is no longer the owner of the string "hello"
    println!("{}", p);
}