use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde_json;

struct Book {
    name: String,
    author: String,
    goodreads_link: String,
}

fn main() {
    let fil_path = Path::new("book_list.json");
    let file = File::open(fil_path);
    let book:Vec<Book> = serde_json::from_reader(file).expect("Error");

}
