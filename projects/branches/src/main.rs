fn main() {
    let number: i32 = 17;

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3 oe 2")
    }

    // if is an expression
    let condition:bool = true;
    let x:i32 = if condition { 5 } else { 7 };
    println!("Value of x is {}", x);
}
