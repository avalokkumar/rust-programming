/*
Traits
A trait defines a set of methods that a type must implement.
Traits are similar to interfaces in other programming languages. Here is an example of a trait called Animal:

This trait has two methods: name() and sound(). Any type that implements the Animal trait must provide
an implementation for these two methods.
 */
trait Animal {
    fn name(&self) -> String;
    fn sound(&self) -> String;
}


struct Cat {
    name: String,
}

/*
The Cat struct has a name field and provides an implementation for the Animal trait by implementing the name() and sound() methods.

We can use the Animal trait to write a function that takes any type that implements the Animal trait:
 */
impl Animal for Cat {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn sound(&self) -> String {
        String::from("meow")
    }
}

/*
The make_sound() function takes a reference to a value that implements the Animal trait and prints its name and sound.

We can create a Cat instance and pass it to the make_sound() function:
 */
fn make_sound(animal: &dyn Animal) {
    println!("{} says {}", animal.name(), animal.sound());
}

pub fn execute_traits() {
    let cat = Cat { name: String::from("Fluffy") };
    make_sound(&cat);
}