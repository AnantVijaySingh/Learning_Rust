use std::fs;
use serde_json;

fn main() {
    let file_path = "book_list.json";
    let contents = fs::read_to_string(file_path).expect("Couldn't read file");

}
