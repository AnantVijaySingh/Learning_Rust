use std::collections::HashMap;

fn main() {
    let mut houses = HashMap::new();

    houses.insert(String::from("Gryffindor"), 30);
    houses.insert(String::from("Slytherin"), 25);
    houses.insert(String::from("Hufflepuff"),25);
    houses.insert(String::from("Ravenclaw"), 35);

    let house_name = String::from("Ravenclaw");

    println!("{} has {} points", house_name, houses.get(&house_name).copied().unwrap_or(0));

    for (key, value) in &houses {
        println!("{}: {}", key, value);
    }

    houses.insert(String::from("Ravenclaw"), 30);

    println!("{} has {} points", house_name, houses.get(&house_name).copied().unwrap_or(0));

    // Entry method
    houses.entry(String::from("Papillionlisse")).or_insert(30);
    houses.entry(String::from("Hufflepuff")).or_insert(30);

    println!("{:?}", houses);

    //Updating a Value Based on the Old Value
    let text = "It takes a great deal of bravery to stand up to our enemies but just as much to \
    stand up to our friends";

    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", word_count);

}
