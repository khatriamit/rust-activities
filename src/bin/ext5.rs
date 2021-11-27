// borrow issue in string slice caused by ownership

struct Name {
    // name: &str, // cannot use string slice as it uses borrowed value and after teh
    // lifetime of Name struct it tries to clean the memory but it
    // can't as it has no ownership in name value
    name: String,
}

fn main() {
    let my_name = "amit".to_owned();
    let my_name = String::from("amit");
    let nm = Name { name: my_name };
}
