// Topic: Decision making with match
// match is similar to if..else condition
// Every possible option must be accounted for in match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn main() {
    // initialize the variable
    let my_bool = true;
    // let my_bool = false;

    // start match the conditional statement 
    // => is fat arrow by naming
    match my_bool{
        true => println!("it's true"), 
        false => println!("it's false"),
    }

}
