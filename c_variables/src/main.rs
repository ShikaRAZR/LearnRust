use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    // like immutable variables, contants are values that are bound to a name, very few differences
    // you cant use mut on a constant, and you use 'const' instead of 'let'

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // compiler can evaluate a limited set of operations at compile time

    let y = 5;
    let y = y + 1; // first variable is shadowed by the second with the keyword 'let'
    {
        let y = y * 2; //shadows second variable but only in the scope
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");



    // shadowing different from mut cause you get compile-time error if you try to reassign var without let
    // we creates a new variable when we use let keyword again

    //this will not give an error
    let spaces = "   ";
    let spaces = spaces.len();

    //this will give an error, since you can't change a var type without shadowing
    //let mut spaces = "   "
    //spaces = spaces.len();

    //rust is a statically typed language, can infer the type of "42"
    let guess: u32 = "42".parse().expect("Not a number!");

    float_point_types();
    numeric_operations();
    boolean_type();
    character_type();
    tuple_type();
    tuple_type_two();
    array_type();
    invalid_array_element_access();
}


// Scalar Types
fn float_point_types() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}

fn numeric_operations() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    println!("{quotient}");

    // remainder
    let remainder = 43 % 5;
}

fn boolean_type() {
    let t = true;

    let f: bool = false; // with explicit type annotation
    println!("{}",t)
}

fn character_type() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    // char literals with single quotes
    // char is 4 bytes in size, represents Unicode Scalar Value
}

//Compound Types
fn tuple_type() {// grouping a number of values, has fixed length
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    let (x, y, z) = tup; // to get individual values

    println!("The value of y is: {y}");
}

fn tuple_type_two() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    // use period to get individual values
    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}

fn array_type() {// all elements in an array must have same type, also has a fixed length, 
    let a = [1, 2, 3, 4, 5];

    //Good use case for array: array of months, my marco program with array of coords, time
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5]; //explicit, i32 var type, length of 5

    let a = [3; 5]; // length of 5, each element value is 3
    let a = [3, 3, 3, 3, 3]; // concise way

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
}

fn invalid_array_element_access() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    //inputing number larger than array length returns runtime error, panic
    // it will check index length before executing
    // many low level languages will not do this check, invalid memory would be accessed
    let element = a[index];
    // println! never ran if this panic occurs 

    println!("The value of the element at index {index} is: {element}");
}