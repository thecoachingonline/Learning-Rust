fn main() {
    let message = String::from("Greeting from Earth!");
    println!("message is {}", message);

    let last_world = &message[15..15+5];
    println!("last_world is {}", last_world);

    let planets = [1, 2, 3, 4, 5, 6, 7, 8];
    let inner_planets: &[i32] = &planets[..4];
    println!("inner_planets are {:?}", inner_planets);
}
