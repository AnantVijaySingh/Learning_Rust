use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // panic!("Ahhhhhh 😩");

    let greeting_file_result = File::open("/Users/anantvijaysingh/RustroverProjects/Learning_Rust/projects/error_handling/src/hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        // Err(error) => panic!("Problem opening the file: {:?}", error),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("/Users/anantvijaysingh/RustroverProjects/Learning_Rust/projects/error_handling/src/hello.txt") {
                Ok(new_file) => new_file,
                Err(error_file_create) => panic!("Problem creating file {:?}", error_file_create),
            }
            other_error => {
                panic!("Problem opening the file {:?}", other_error);
            }
        },
    };

}
