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

    // loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {}", count);

    lift_off();

    let a:[i32; 5] = [10, 20 ,30, 40, 50];
    print_array_1(a);

    let upto = 5;
    count_using_for(upto);

    largest_prime_number_in_i8();
}

fn lift_off() {
    let mut countdown:i32 = 3;

    println!("🚀 Lift off in T minus");

    while countdown > 0 {
        println!("{}", countdown);

        countdown -= 1;
    }

    println!("LIFTOFF!!!");
}

fn print_array_1(a: [i32; 5]) {

    for element in a {
        println!("The value is: {}", element);
    }
}

fn count_using_for(upto: i32) {

    for count in 1..upto {
        println!("..{}", count);
    }
}

fn largest_prime_number_in_i8 () {
    let max = i32::MAX;

    println!("- - - - - - - - - - - - - - - - - - - - - - ");

    for largest_prime in (2..max).rev() {
        if is_prime(largest_prime) {
            println!("Largest prime number that i32 can hold is: {}", largest_prime);
            break
        }
    }
}

fn is_prime(number: i32) -> bool {
    for n in 2..number {
        if number % n == 0 {
            println!("{} is not prime as it is divisible by {}", number, n);
            return false;
        }
    }
    true
}