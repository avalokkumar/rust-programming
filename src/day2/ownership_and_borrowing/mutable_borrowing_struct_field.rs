/*
Here we define a struct Point with two i32 fields x and y. We create a mutable instance of this
struct called p with both fields initialized to 0. Then we create a new scope and borrow a mutable reference to the x
field of p using r. We set the value pointed to by r to 5, which modifies the x field of p as well.
Finally, we print the value of p.x which is now 5.
 */
pub fn execute_mutable_borrowing_struct_field() {
    let mut p = Point { x: 0, y: 0 };

    {
        let r = &mut p.x;
        *r = 5;
    }

    println!("p.x = {}", p.x);
}

struct Point {
    x: i32,
    y: i32,
}