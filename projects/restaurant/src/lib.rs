mod front_of_house;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

mod back_of_house {

    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer (toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches"),
            }
        }

        pub fn food_order_confirmation (meal: &Breakfast) {
            println!("{:?}", meal);
        }
    }

    pub enum Appetizer {
        Soup,
        Salad
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() { }
}

fn deliver_order () {
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant () {

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // With use keyword
    hosting::add_to_waitlist();

    // In case of Structs, fields are private by default even for a public Struct
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");

    back_of_house::Breakfast::food_order_confirmation(&meal);

    // In the case of enum, all files are public for a public enum
    let order_1 = back_of_house::Appetizer::Soup;
    let order_2 = back_of_house::Appetizer::Salad;

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
