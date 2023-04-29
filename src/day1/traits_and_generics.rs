// Define the Push trait, which provides a method for pushing an element onto a stack.
trait Push<T> {
    fn push(&mut self, element: T);
}

// Define the Stack struct, which holds a vector of elements of type T.
struct Stack<T> {
    elements: Vec<T>,
}

// Implement the Push trait for any type T that implements the Copy trait.
impl<T: Copy> Push<T> for Stack<T> {
    fn push(&mut self, element: T) {
        self.elements.push(element);
    }
}

// Methods for the Stack struct.
impl<T> Stack<T> {
    // Create a new empty stack.
    fn new() -> Stack<T> {
        Stack { elements: Vec::new() }
    }

    // Pop an element off the top of the stack.
    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    // Check if the stack is empty.
    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

pub fn execute_traits_and_generics() {
    // Create a new stack of integers.
    let mut int_stack = Stack::new();

    // Push some integers onto the stack.
    int_stack.push(1);
    int_stack.push(2);
    int_stack.push(3);

    // Pop the integers off the stack and print them.
    while let Some(int) = int_stack.pop() {
        println!("{}", int);
    }

    println!("\nint_stack is_empty :: {}\n", int_stack.is_empty());

    // Create a new stack of strings.
    let mut string_stack = Stack::new();

    // Push some strings onto the stack.
    string_stack.push("foo");
    string_stack.push("bar");
    string_stack.push("baz");

    // Pop the strings off the stack and print them.
    while let Some(string) = string_stack.pop() {
        println!("{}", string);
    }
}