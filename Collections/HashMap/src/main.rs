use std::collections::HashMap;

fn main() {
    let mut missions_flown = HashMap::new();
    missions_flown.insert("Hedfield", 3);
    missions_flown.insert("Hurley", 3);
    missions_flown.insert("Barron", 0);
    missions_flown.insert("Barron", 1);
    missions_flown.entry("Stone").or_insert(2);
    let kayla = missions_flown.entry("Barrow").or_insert(0);
    *kayla += 1;
    println!("missions_flown is {:?}", missions_flown);

    let barron_missions = missions_flown.get("Barrow");
    println!("barron_missions is {:?}", barron_missions);
}
