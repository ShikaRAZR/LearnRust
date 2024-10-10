fn main() {
    let number = 3;
    
    // if expresion followed by a boolean condition
    // these expressions are cometimes called arms like in match
    if number < 5 { 
        println!("condition was true");
    } else {
        println!("condition was false");
    }



    // This causes an error:
    //if number {
        // println!("number was three");
    //}


    
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    // if a number is divisible by 4 AND 3, will only show 4



    let condition = true;
    // blocks of code evalute to last expression, here its 5 or 6
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");



    // this will cause an error, incompatible types
    // number type is determined at compile time, not runtime
    // let number = if condition { 5 } else { "six" };
}
