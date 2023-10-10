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