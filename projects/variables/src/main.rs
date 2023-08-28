const VALUE_OF_PI: f64 = std::f64::consts::PI;

fn main() {
    let mut x:u32 = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    println!("Value of pi: {}", VALUE_OF_PI);

    let y = 7;

    let y = y + 2;

    {
        let y = y + 2;
        println!("Value of y in inner scope: {}", y);
    }

    println!("Value of y in outer scope: {}", y);
}
