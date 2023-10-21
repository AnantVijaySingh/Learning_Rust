use crate::garden::vegetables::{Asparagus, NutritionalValue};

pub mod garden;

fn main() {

    let plant = Asparagus {
        name: String::from("Asparagus"),
        nutritional_value: NutritionalValue::High
    };

    println!("{} has {:?} nutritional value", plant.name, plant.nutritional_value);
}
