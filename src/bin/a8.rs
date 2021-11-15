// Topic: Organizing similar data using struct
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor


// flavour enum initialize
enum Flavour{
    Cola,
    Sprite,
    Fruity,
}

// drink struct initialize with flavour and fluid ounce data types
struct Drink{
    flavour: Flavour,
    fluid_oz: f64,
}

// function to print out drink flavor and ounces
fn print_drink(drink: Drink){
    
    //match expression to print the drink flavor
    match drink.flavour {
        Flavour::Cola => println!("flavour: Cola "),
        Flavour::Sprite => println!("flavour: Sprite"),
        Flavour::Fruity => println!("flavour: Fruity"),
    }

    println!("oz: {:?}", drink.fluid_oz);
}

fn main() {
    let cola = Drink{
        flavour: Flavour::Cola,
        fluid_oz: 3.5,
    };

    let sprite = Drink{
        flavour: Flavour::Sprite,
        fluid_oz: 5.5,
    };

    let fruity = Drink{
        flavour: Flavour::Fruity,
        fluid_oz: 2.5,
    };
    print_drink(cola);
    print_drink(sprite);
    print_drink(fruity);
}
