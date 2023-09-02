fn main() {
    println!("Hello, world!");

    another_function(5);

    print_labeled_measurements(5, '👌');

    println!("The square of 7 is {}", square(7));

}

fn another_function(x: i32) {
    println!("Another function!");
    println!("The value of x is {}", x);
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement value is {}{}", value, unit_label)
}

fn square (x: i32) -> i32{
    i32::pow(x, 2)
}