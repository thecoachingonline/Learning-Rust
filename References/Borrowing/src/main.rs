fn main() {
    let rocket_fuel = String::from("RP-1");
    let (rocket_fuel, length) = procees_fuel(rocket_fuel);
    println!("rocket_fuel is {} and length is {}", rocket_fuel, length);
}

fn procees_fuel(propellant: String) -> (String, usize) {
    println!("processing propellant {}...", propellant);
    let length = propellant.len();
    (propellant, length)
}
