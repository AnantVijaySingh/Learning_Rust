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

fn main() {
    let m_1 = Message::Write(String::from("Hello"));
    m_1.call();

    let m_2 = Message::Quit;
    m_2.call();

}
