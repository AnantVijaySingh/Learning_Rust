fn main() {
    let mut s_1 = String::new();

    let some_data = "initial contents";
    let s_2 = some_data.to_string();

    let s_3 = "initial contents".to_string();

    let s_4 = String::from("Hello there");

    let mut hellos = Vec::new();

    hellos.push(String::from("Dobrý den"));
    hellos.push(String::from("Hello"));
    hellos.push(String::from("नमस्ते"));
    hellos.push(String::from("こんにちは"));
    hellos.push(String::from("안녕하세요"));
    hellos.push(String::from("你好"));
    hellos.push(String::from("Olá"));
    hellos.push(String::from("Hola"));

    for hello in hellos {
        println!("{}", hello);
    }

    // Appending to a String with push_str and push
    let mut s_5 = String::from("Up");
    s_5.push_str(" Down");

    println!("{}", s_5);

    s_5.push('!');

    println!("{}",s_5);

    // Concatenation with the + Operator or the format! Macro
    let s_6 = String::from("Hahaha");
    let s_7 = String::from("Hehehe");
    let s_8 = s_6 + &s_7; // s_6 is moved and can no longer be used

    println!("{}", s_8);

    let s_9 = String::from("Ice cream");
    let s_10 = String::from("Chocolate cake");
    let s_11 = String::from("Creme Brulee");

    let s_12 = format!("{s_9} - {s_10} - {s_11}");

    println!("{}", s_12);

}
