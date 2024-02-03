use std::fs::File;
use std::io;
use std::io::ErrorKind;

fn main() {
    // panic!("Ahhhhhh 😩");

    let mut file_path_and_name = String::new();

    println!("Enter file path and name");

    io::stdin()
        .read_line(&mut file_path_and_name)
        .expect("Failed to read line");

    let file_path_and_name = file_path_and_name.trim();

    let file_result = File::open(file_path_and_name);

    let file = match file_result {
        Ok(file) => {file},
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => {
                    println!("File not found. New file has been created instead");

                    match File::create("helloWorld.txt") {
                        Ok(file) => file,
                        Err(error) => { panic!("Problem creating file: {}", error) }
                    }
                },
                other_error => { panic!("Problem opening file: {}", other_error) }
            }
        },
    };
}
