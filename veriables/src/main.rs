use std::collections::HashMap;

fn main() {
    // Basic variable declarations
    let x = 5; // Immutable variable
    let mut y = 10; // Mutable variable

    // Shadowing
    let y = y + 5; // Shadowing the previous value of 'y'
    let y = y * 2; // Shadowing again

    // Tuple variables
    let tup: (i32, f64, u8) = (500, 6.4, 1); // Tuple with explicit types
    let (_, z, _) = tup; // Destructuring the tuple, only interested in the second element

    // Array variables
    let arr = [1, 2, 3, 4, 5]; // Array with inferred type [i32; 5]
    let first_element = arr[0]; // Accessing array elements

    // String variables
    let s = String::from("hello"); // Mutable String object
    let s_len = s.len(); // Getting the length of the string

    // Constant variables
    const MAX_POINTS: u32 = 100_000;

    // Using variables in expressions
    let sum = x + y; // Using variables in an arithmetic expression
    let is_greater = z > (first_element as f64); // Using variables in a comparison expression

    // Variable scope
    {
        let scoped_var = "I'm only available in this scope";
        println!("{}", scoped_var);
    } // scoped_var is no longer accessible here

    // HashMap
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);

    // Pattern matching
    match scores.get("Blue") {
        Some(&score) => println!("Blue team's score is: {}", score),
        None => println!("Blue team's score not found!"),
    }

    // Structs and methods
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is: {}", rect.area());

    // Printing out the values
    println!("The value of x is: {}", x);
    println!("The value of y after shadowing is: {}", y);
    println!("The second element of the tuple is: {}", z);
    println!("The first element of the array is: {}", first_element);
    println!("The length of the string '{}' is: {}", s, s_len);
    println!("The maximum points constant is: {}", MAX_POINTS);
    println!("The sum of x and y is: {}", sum);
    println!(
        "Is z greater than the first element of the array? {}",
        is_greater
    );
}

// Defining a struct
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementing a method for the struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
