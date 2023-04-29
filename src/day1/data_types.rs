
pub fn run_data_types() {
    // Declaring a signed 8-bit integer variable named "a" and initializing it with value "-5".
    let a: i8 = -5;
    println!("a = {}", a);

// Declaring an unsigned 16-bit integer variable named "b" and initializing it with value "10".
    let b: u16 = 10;
    println!("b = {}", b);

// Declaring a signed 32-bit integer variable named "c" and initializing it with value "100".
    let c: i32 = 100;
    println!("c = {}", c);

// Declaring an unsigned 64-bit integer variable named "d" and initializing it with value "1000".
    let d: u64 = 1000;
    println!("d = {}", d);

// Declaring a single-precision floating-point variable named "e" and initializing it with value "3.14".
    let e: f32 = 3.14;
    println!("e = {}", e);

// Declaring a double-precision floating-point variable named "f" and initializing it with value "3.1415926535".
    let f: f64 = 3.1415926535;
    println!("f = {}", f);

// Declaring a boolean variable named "g" and initializing it with value "true".
    let g: bool = true;
    println!("g = {}", g);

// Declaring a boolean variable named "h" and initializing it with value "false".
    let h: bool = false;
    println!("h = {}", h);

// Declaring a character variable named "i" and initializing it with value "a".
    let i: char = 'a';
    println!("i = {}", i);

// Declaring a string slice variable named "j" and initializing it with value "hello, world!".
    let j: &str = "hello, world!";
    println!("j = {}", j);

// Declaring a string variable named "k" and initializing it with value "Hello, Rust!".
    let k: String = String::from("Hello, Rust!");
    println!("k = {}", k);

// Declaring an array named "l" of size 5 and initializing it with values [1, 2, 3, 4, 5].
    let l: [i32; 5] = [1, 2, 3, 4, 5];
    println!("l = {:?}", l);

// Declaring a tuple named "m" and initializing it with values (10, 3.14, 'a').
    let m: (i32, f32, char) = (10, 3.14, 'a');
    println!("m = {:?}", m);

    // Defining a struct named "Person" with fields "name" of type String and "age" of type i32.
    struct Person {
        name: String,
        age: i32,
    }

// Creating an instance of the Person struct named "n" with name "John" and age "30".
    let n = Person {
        name: String::from("John"),
        age: 30,
    };
    println!("n = {{ name: '{}', age: {} }}", n.name, n.age);

    // Defining an enum named "Color" with variants "Red", "Green", and "Blue".
    #[derive(Debug)]
    enum Color {
        Red,
        Green,
        Blue,
    }

    impl Color {
        // A static method that returns an array containing all of the enum variants.
        fn values() -> &'static [Color] {
            static VALUES: [Color; 3] = [Color::Red, Color::Green, Color::Blue];
            &VALUES
        }
    }

    // Creating an instance of the Color enum named "o" with variant "Red".
    let o = Color::Red;
    println!("o = {:?}", o);

    for color in Color::values().iter() {
        println!("Color: {:?}", color);
    }

}


