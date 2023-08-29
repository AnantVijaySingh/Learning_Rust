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

    // Data types
    // Rust is a statically typed language

    let mut integer:u8 = 254;
    integer = integer + 1;

    println!("Value of integer is {}", integer);

    let a:f32 = 3.14515926535;
    let b:f64 = 3.14515926535;

    println!("Value of a is {}", a);
    println!("Value of b is {}", b);

    let t:bool = true;
    let emoji:char = '😻'; //Rust’s char type is four bytes in size and represents a Unicode Scalar Value
    println!("{}", emoji);

}
