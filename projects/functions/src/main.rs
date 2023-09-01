fn main() {
    println!("Hello, world!");

    another_funtion(5);

    print_labeled_measurements(5, '👌');
}

fn another_funtion(x: i32) {
    println!("Another function!");
    println!("The value of x is {}", x);
}

fn print_labeled_measurements(value: i32, unit_lable: char) {
    println!("The measurement value is {}{}", value, unit_lable)
}