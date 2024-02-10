use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{self, Read};
use std::io::ErrorKind;
use rand::Rng;

enum FnToRun {
    One,
    Two,
    Three
}

impl FnToRun {
    fn run_fn (file_path_and_name: &str) {
        let random_number:u32 = rand::thread_rng().gen_range(1..=3);
        match random_number {
            1 => {one(file_path_and_name);},
            2 => {two(file_path_and_name);},
            3 => {three(file_path_and_name);},
            _ => {}
        };
    }
}

fn main() {
    // panic!("Ahhhhhh 😩");

    let mut file_path_and_name = String::new();

    println!("Enter file path and name");

    io::stdin()
        .read_line(&mut file_path_and_name)
        .expect("Failed to read line");

    let file_path_and_name = file_path_and_name.trim();

    FnToRun::run_fn(file_path_and_name);

    let username = read_username_from_file_long_form(file_path_and_name).unwrap_or_else(|error| {
        println!("{}", error);
        String::from("No username found")
    });

    println!("{}", username);
    println!("{:?}", read_username_from_file_short_form(file_path_and_name));
    println!("{:?}", read_username_from_file_shortest(file_path_and_name));
}


fn read_username_from_file_long_form(file_path_and_name: &str) -> Result<String, io::Error>{
    let username_file_result = File::open("Hello");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username= String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => return Err(e),
    }
}

fn read_username_from_file_short_form(file_path_and_name: &str) -> Result<String, io::Error> {
    let mut username_file = File::open("Hello")?;

    let mut username = String::new();

    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_shortest(file_path_and_name: &str) -> Result<String, io::Error> {
    fs::read_to_string("Hello")
}


fn one (file_path_and_name: &str)  {

    let file_name = file_path_and_name.clone();

    let file_result = File::open(file_path_and_name);

    let file = match file_result {
        Ok(file) => {file},
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => {
                    println!("File not found. New file has been created instead");

                    match File::create(file_name) {
                        Ok(file) => file,
                        Err(error) => { panic!("Problem creating file: {}", error) }
                    }
                },
                other_error => { panic!("Problem opening file: {}", other_error) }
            }
        },
    };
}

fn two (file_path_and_name: &str) {
    let file_result = File::open(file_path_and_name)
        .expect("{} should have been included in the project folder");
}

fn three (file_path_and_name: &str) {
    let file_result = File::open(file_path_and_name).unwrap();
}
