/*
Here we define the same Point struct as in the previous example. We also define a function
print_point that takes an immutable reference
to a Point and prints its coordinates. In the main function, we create an instance of Point called p, and we pass a reference to it to print_point.
Since print_point only needs to read the values of p.x and p.y, we can safely pass an immutable reference to p without transferring ownership of p.
 */
pub fn execute_immutable_borrowing_struct_field() {
    let p = Point { x: 0, y: 0 };
    print_point(&p);
}

struct Point {
    x: i32,
    y: i32,
}

fn print_point(p: &Point) {
    println!("({}, {})", p.x, p.y);
}