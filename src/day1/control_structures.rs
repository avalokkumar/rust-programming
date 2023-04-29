
pub fn run_control_structures() {
    let num = 10;

    if num == 10 {
        println!("num is equal to {}", num);
    }

    // if/else statement
    if num < 0 {
        println!("{} is negative", num);
    } else if num > 0 {
        println!("{} is positive", num);
    } else {
        println!("{} is zero", num);
    }

    // loop statement
    let mut i = 0;
    loop {
        if i >= 5 {
            break;
        }
        println!("i = {}", i);
        i += 1;
    }

    // while loop statement
    let mut j = 0;
    while j < 5 {
        println!("j = {}", j);
        j += 1;
    }

    // for loop statement
    let arr = [1, 2, 3, 4, 5];
    for element in arr.iter() {
        println!("arr element = {}", element);
    }

    for elem in arr.iter() {
        print!("{} ", elem )
    }

    print!("\n");
    // match statement
    let num2 = 3;
    match num2 {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"),
    }

    //multi line statement
    let num2 = 1;
    match num2 {
        1 => {
            println!("One");
        }
        2 => {
            println!("Two");
        }
        3 => {
            println!("Three");
        }
        _ => {
            println!("Something else");
        }
    }

}