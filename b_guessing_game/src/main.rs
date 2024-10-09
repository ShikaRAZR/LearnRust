use std::io; //io intput/output library comes from std library, brings it into this scope
use rand::Rng; // Rng trait defines methods that random number generators implement, brought into scope
use std::cmp::Ordering; // Orderting type, another enum, with variants Less, Greater, Equal, brings into scope

fn main() { // program runs from here
    println!("Guess the number!");

    // rand::thread_rng, uses a particular random number generator local to execution
    //gen_range(start..=end), brought into scope, is inclusive of lower and upper bounds
    let secret_number = rand::thread_rng().gen_range(1..=100); // var type is default as i32, a 32-bit number
    println!("The secret number is {secret_number}");


    loop{
        println!("Please input your guess.");

        //variables are immutable (default), add 'mut' before var (modifiable)
        //String::new, function that returns a new instance of a String (included in standard lib)
        let mut guess = String::new(); 


        // if 'use std::io;' was not called you could have done:
        // std::io::stdin()
        io::stdin() //io module, with stdin function 

            // APPENDS user input into string, DOESN'T OVERWRITE, string needs to be mutable
            // & indicates that this argument is a reference, data accessed without copying data to memory
            // references are immutable by default, &mut makes it mutable
            .read_line(&mut guess)

            // Error handling, read_line returns a Result value, an enumeration 
            // aka enum, can be in one of multiple possible states called a variant (Ok and Err)
            // code will still compile without .expect but you will get warning
            .expect("Failed to read line");

        // three lines of text, but is still SINGLE logical line of code:
        // io::stdin().read_line(&mut guess).expect("Failed to read line");

        // guess was a previously used variable
        // shadowing lets us reuse the guess variable, it shadows the previous value with a new one
        // used to convert from one type to another
        // trim eliminates white space, and also Enters ('\n')
        // parse converts string to another type, we use u32, unsigned 32-bit integer good for a small positive number (Only Signed can be negative)
        // if string cant be parsed it will return an Err variant, expect() will be called and print, program will crash
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num, // if parse is sucessful, will return an Ok value that contains Result number
            Err(_) => continue, //if unsucessful, return Err that contains info about error, underscore (_) is a catchall value
            // "continue" tells program to break loop and go through next iteration of loop
        };

        println!("You guessed: {}", guess); // '{}' is a placeholder for 'guess' var

        // cmp compares 2 values and returns a variant of Ordering enum brought into scope
        // match expression to decide what to do based on Ordering variant
        match guess.cmp(&secret_number){ 
            //match expression are made up of arms, will return one of these: 
            // Ordering::Less, Ordering::Greater, Ordering::Equal (these are the arms)
            // the match expression ends once one of the arm's code is executed 
            // if guess is 50 and secret_number is 25 (last scenario not checked)
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }


    print_math()
    
}

fn print_math(){
    let x = 5;
    let y = 10;
    println!("x = {x} and y + 2 = {}", y + 2);
}