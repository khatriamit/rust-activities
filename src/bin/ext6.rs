struct LineItem {
    name: String,
    count: i32,
}

fn print_name(name: &str) {
    println!("name: {:?}", name);
}

fn main() {
    let receipt = vec![
        LineItem {
            name: "cereal".to_owned(),
            count: 4,
        },
        LineItem {
            name: String::from("meat"),
            count: 10,
        },
    ];

    for item in receipt {
        print_name(&item.name);
        // println!("name: {:?}, count: {:?}", item.name, item.count);
        println!("count: {:?}", item.count);
    }
}
