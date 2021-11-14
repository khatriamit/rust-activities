// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {
    // variable initialize
    let mut my_var = 1;

    // start iterating using loop statement
    loop{
        println!("{:?}", my_var);
        my_var+=1;

        // put condition to break out of the loop
        if my_var == 5{
            break;
        }
    }
}
