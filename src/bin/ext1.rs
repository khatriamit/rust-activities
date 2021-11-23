enum Light {
    Bright,
    Dull,
}

fn display_light(light: &Light) {
    match light {
        Light::Bright => println!("Bright"),
        Light::Dull => println!("Dull"),
    }
}

fn main() {
    let dull = Light::Dull;
    let br = Light::Bright;
    display_light(&dull);
    display_light(&br);
    display_light(&br);
}
