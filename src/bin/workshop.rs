fn main() {
    let mut name = "raju";
    println!("hello, {}", name);

    name = "sagar";
    println!("Hello, {}", name);

    // option
    // Some(value)
    // None
    let option: Option<i32> = None;
    if option.is_some() {
        let value = option.unwrap();
        println!("the value: {}", value);
    } else {
        println!("Option has no value");
    }

    let option = Some(20);
    println!("The value is: {:?}", option);
}
