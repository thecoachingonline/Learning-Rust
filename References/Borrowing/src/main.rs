fn main() {
    let rocket_fuel = String::from("RP-1");
    let length = procees_fuel(&rocket_fuel);
    println!("rocket_fuel is {} and length is {}", rocket_fuel, length);
}

fn procees_fuel(propellant: &String) -> usize {
    println!("processing propellant {}...", propellant);
    let length = propellant.len();
    length
}
