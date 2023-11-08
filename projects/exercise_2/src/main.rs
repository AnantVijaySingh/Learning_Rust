// Given a list of integers, use a vector and return the median (when sorted, the value in the
// middle position) and mode (the value that occurs most often; a hash map will be helpful here)
// of the list.

use rand::Rng;
use std::collections::HashMap;

fn main() {
    let list_of_integers = list_of_num();

    println!();
    print!("The list of integers is: {:?}", list_of_integers);

    let mut integer_hashmap: HashMap<i32, i32> = HashMap::new();

    for int in list_of_integers {
        let count = integer_hashmap.entry(int).or_insert(0);
        *count +=1;
    }


    println!();
    println!("Integers & Frequency:    {:?}", integer_hashmap);
    println!("Mode is: {:?}", mode(&integer_hashmap));
    println!();

    let mut int_list: Vec<i32> = Vec::new();

    for int in list_of_integers {
        int_list.push(int);
    }

    int_list.sort_unstable();

    println!("Vector is: {:?}", int_list);
    println!("Median is: {:?}",
             match int_list.get(int_list.len() / 2) {
                None => {0}
                Some(value) => {*value}
             }
    )
}


fn list_of_num() -> [i32; 10] {
    let mut num: [i32; 10] = [0; 10];

    for i in 0..num.len() {
        let temp = rand::thread_rng().gen_range(1..=10);
        num[i] = temp;
    }
    num
}

fn mode(map: &HashMap<i32, i32>) -> i32 {
    let mut mode: i32 = 0;
    let mut mode_value = 0;

    for (key, value) in map {
        if mode_value < *value {
            mode_value = *value;
            mode = *key;
        }
    }
    mode
}