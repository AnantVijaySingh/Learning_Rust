fn main() {

    // Ways to declare a vector
    let v_1: Vec<i32> = Vec::new();

    let v_2 = vec![1, 2, 3];

    // Adding to a vector
    let mut v_4 = Vec::new();

    v_4.push(3);
    v_4.push(1);
    v_4.push(4);
    v_4.push(1);
    v_4.push(5);

    // Reading elements from a vector
    let third: &i32 = &v_4[3];
    println!("The third number in pi is: {}", third);

    let third: Option<&i32> = v_4.get(7);
    match third {
        Some(third) => println!("The third number in pi is: {}", third),
        None => println!("There is no third element")
    }

    // Iterating over values in a vector
    for n_ref in &v_4 {
        println!("Values of pi: {}", n_ref);
    }

    for n_ref in &mut v_4 {
        println!("Messing up the vector v_4 {}", *n_ref + 1);
    }

    // Using an Enum to Store Multiple Types
    #[derive(Debug)]
    enum SpreadsheetCell {
        Integer(i32),
        Decimal(f64),
        Text(String),
    }

    let mut row = Vec::new();

    row.push(SpreadsheetCell::Integer(3));
    row.push(SpreadsheetCell::Decimal(1.1));
    row.push(SpreadsheetCell::Text(String::from("Orange")));

    for i in row {
        match i {
            SpreadsheetCell::Text(value) => println!("The text in the row is: {}", value),
            other => (),
        }
    }


}
