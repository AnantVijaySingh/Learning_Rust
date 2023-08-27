use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number:u32 = rand::thread_rng().gen_range(1..=10);

    // println!("The secret number is: {}", secret_number);

    loop {

        println!("Please input your guess now:");

        let mut guess:String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Try again");
                continue;
            },
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("Bingo! That is correct");
                break;
            },
            Ordering::Greater => println!("Too big"),
        }
    }
}
