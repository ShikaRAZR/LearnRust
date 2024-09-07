fn main() {
    println!("Hello, world!");

    another_function(5);

    print_labeled_measurement(5, 'h');

    functions_are_statements(); 

    calling_a_function_is_expression();

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}
// functions can be defined before or after main
fn another_function(x: i32) { // must declare type of each parameter
    println!("Another function.");
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn functions_are_statements() { 
    // functions are statements do not return values 
    // this will have an error
    //let x = (let y = 6);
    // some languages you can write x = y = 6, Rust doesnt do that
    println!("functions_are_statements")
}

fn calling_a_function_is_expression() { 
    // new scope block created with curly brackets is an expression
    let y = { // expression this will work
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 { // function with return value, return values arent named
    // you can use 'return' keyword with a value to return early
    // most functions return the last expression implicitly
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
    
    // This would produce an error (mismatch types) because nothing is being returned
    // x + 1;

    // This won't produce an error
    // return x + 1;
}