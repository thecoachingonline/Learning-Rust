#[derive(PartialEq, PartialOrd)]
struct Satellite {
    name: String,
    velocity: f64 // miles per second
}

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };
    let gps = Satellite {
        name: String::from("GPS"),
        velocity: 2.42
    };
    println!("Hubble == gps is {}", hubble == gps);
    println!("Hubble > gps is {}", hubble > gps);
}
