fn main() {
    let outer_planet: String;
    {
        let mut inner_planet = String::from("Mercury");
        outer_planet = inner_planet.clone();
        inner_planet.clear();
        println!("inner_planet is {}", inner_planet);
    }
    println!("outer_planet is {}", outer_planet);
}
