// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
struct Person {
    age: i32,
    name: String,
    fav_color: String,
}

// * The name and colors should be printed using a function
fn print_name(name: &str) {
    println!("Name: {:?}", name);
}

fn print_color(color: &str) {
    println!("Color: {:?}", color);
}

fn main() {
    // * Create and store at least 3 people in a vector
    let people = vec![
        Person {
            age: 10,
            name: "Brother".to_owned(),
            fav_color: "Yellow".to_owned(),
        },
        Person {
            age: 14,
            name: String::from("Big Brother"),
            fav_color: String::from("Black"),
        },
        Person {
            age: 6,
            name: String::from("Small Sister"),
            fav_color: "Pink".to_owned(),
        },
    ];

    // * Iterate through the vector using a for..in loop
    for person in people {
        // * Use an if expression to determine which person's info should be printed
        if person.age <= 10 {
            println!("age: {:?}", person.age);
            print_name(&person.name);
            print_color(&person.fav_color);
        }
    }
}
