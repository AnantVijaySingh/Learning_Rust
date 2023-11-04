use std::collections::HashMap;

fn main() {
    let mut houses = HashMap::new();

    houses.insert(String::from("Gryffindor"), 30);
    houses.insert(String::from("Slytherin"), 25);
    houses.insert(String::from("Hufflepuff"),25);
    houses.insert(String::from("Ravenclaw"), 35);

    let house_name = String::from("Ravenclaw");

    println!("{} has {} points", house_name, houses.get(&house_name).copied().unwrap_or(0));

    for (key, value) in houses {
        println!("{}: {}", key, value);
    }

}
