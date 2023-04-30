/*
Here, we create a mutable variable x with the value 5. Then we create a new scope and borrow a
mutable reference to x using y.
We add 1 to the value pointed to by y, which modifies x as well. Finally, we print the value of x which is now 6.
 */
pub fn execute_borrowing_mutable_reference() {
    let mut x = 5;

    {
        let y = &mut x;
        *y += 1;
    }

    println!("x = {}", x);
}