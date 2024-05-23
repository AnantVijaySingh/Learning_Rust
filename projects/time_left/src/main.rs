use chrono::{Duration, Local, Timelike, Datelike, TimeZone};
use std::io::{self, Write};

fn main() {
    // Get the current time
    let now = Local::now();

    // Ask for user input for the target date and time
    let mut input_date_time = String::new();

    print!("When we will meet next (MM-DD HHMM): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_date_time).unwrap();
    let input_date_time = input_date_time.trim();

    // Parse the input date and time
    let current_year = now.year();
    let input_with_year = format!("{}-{}", current_year, input_date_time);
    let target_time = match Local.datetime_from_str(&input_with_year, "%Y-%m-%d %H%M") {
        Ok(dt) => dt,
        Err(e) => {
            eprintln!("Error parsing date and time: {}", e);
            return;
        }
    };

    // Calculate the time difference
    let time_difference = target_time - now;

    // Extract different components of the time difference
    let days = time_difference.num_days();
    let hours = time_difference.num_hours();
    let minutes = time_difference.num_minutes();
    let seconds = time_difference.num_seconds();
    let milliseconds = time_difference.num_milliseconds();
    let nanoseconds = time_difference.num_nanoseconds().unwrap();

    // Print the result in distinct units
    println!("There are:");
    println!("{} days left", days);
    println!("{} hours left", hours);
    println!("{} minutes left", minutes);
    println!("{} seconds left", seconds);
    println!("{} milliseconds left", milliseconds);
    println!("{} nanoseconds left", nanoseconds);
}

// Counting down the time to when I'll meet my better half :)