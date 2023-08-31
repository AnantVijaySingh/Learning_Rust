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

    let _t:bool = true;
    let emoji:char = '😻'; //Rust’s char type is four bytes in size and represents a Unicode Scalar Value
    println!("{}", emoji);

    // Compound data types
    let tuple:(i32, f64, u8) = (500, 6.4, 1);

    let (_p,q,_r) = tuple;

    println!("The value of q is {}", q);

    let _five_hundred = tuple.0;
    let six_point_four = tuple.1;

    println!("six point four {}", six_point_four);

    let arr:[i32; 5] = [1, 2, 3, 4, 5];

    let months:[&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    // You can also initialize an array to contain the same value for each element by specifying the
    // initial value, followed by a semicolon, and then the length of the array in square brackets
    let a = [3; 5];

    println!("{}", a[0]);

    let t = ([1; 2], [3; 4]);

    let (a,_) = t;

    println!("{}", a[1] + t.1[1]);
}
