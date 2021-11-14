// Topic: Looping using the while statement
//
// Program requirements:
// * Counts down from 5 to 1, displays the countdown
//   in the terminal, then prints "done!" when complete.
//
// Notes:
// * Use a mutable integer variable
// * Use a while statement
// * Print the variable within the while loop
// * Do not use break to exit the loop

fn main() {
    // variable initialize
    let mut my_var = 5;

    // start iterating using loop statement
    while my_var != 0{
        println!("{:?}", my_var);
        my_var-=1;
    }
    println!("done!")
}
