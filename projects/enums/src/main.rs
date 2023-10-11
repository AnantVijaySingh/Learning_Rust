use rand::Rng;

#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y:i32}, // Struct type
    Write(String), // Tuple type
    ChangeColor(i32, i32, i32), // Tuple type
}

impl Message {
    fn call(self: &Self) {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
enum USState {
    California,
    Washington,
    // -- snip --
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(USState),
}


fn main() {
    let m_1 = Message::Write(String::from("Hello"));
    m_1.call();

    let m_2 = Message::Quit;
    m_2.call();


    // Option

    let some_number = Some(5);
    let some_word: Option<String> = Some(String::from("Hey"));

    let absent_number: Option<i32> = None;

    // match
    let penny = Coin::Penny;
    value_in_cents(penny);

    value_in_cents(Coin::Quarter(USState::California));

    // Option<T>

    let five: Option<i32> = Some(5);
    let six = plus_one(five);
    println!("{:?}", six);

    let none = plus_one(None);

    // Other & _
    let dice_roll: u8 = rand::thread_rng().gen_range(1..=6);

    match dice_roll {
        3 => get_chocolate_cake(),
        6 => lose_chocolate_cake(),
        other => move_forward(other)
    }

    match dice_roll {
        3 => get_chocolate_cake(),
        6 => lose_chocolate_cake(),
        _ => re_roll(),
    }

    // Match and Ownership

    let opt: Option<String> = Some(String::from("Hey!"));

    match opt {
        Some(_) => {
            println!("Some!");
        },
        None => {
            println!("None!");
        }
    }

    println!("{:?}", opt);

    match &opt { // matching on a reference
        Some(value) => {
            println!("{}", value);
        },
        None => {
            println!("None!");
        }
    }

}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        },
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}

fn plus_one(value: Option<i32>) -> Option<i32> {
    match value {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn get_chocolate_cake() {
    println!("I got a slice of chocolate cake!");
}

fn lose_chocolate_cake() {
    println!("I had to give away my slice of chocolate cake!");
}

fn move_forward(x: u8) {
    println!("Move forward by {}", x);
}

fn re_roll() {
    println!("Roll the dice again");
}