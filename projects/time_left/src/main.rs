use chrono::{Duration, Local, Timelike, Datelike, TimeZone};
use rand::Rng;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration as StdDuration;

fn print_time_remaining(target_time: chrono::DateTime<Local>) {
    let now = Local::now();
    let time_difference = target_time - now;

    // Extract different components of the time difference
    let days = time_difference.num_days();
    let hours = time_difference.num_hours() % 24;
    let minutes = time_difference.num_minutes() % 60;
    let seconds = time_difference.num_seconds() % 60;
    let milliseconds = time_difference.num_milliseconds() % 1000;
    let nanoseconds = time_difference.num_nanoseconds().unwrap() % 1_000_000_000;

    // Print the result in distinct units
    println!("There are:");
    println!("{} days, {} hours, {} minutes, {} seconds, {} milliseconds and {} nanoseconds left", days, hours, minutes, seconds, milliseconds, nanoseconds);
}

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

    let target_time = Arc::new(target_time);
    let target_time_clone = Arc::clone(&target_time);

    // Thread to handle periodic updates
    let handle = thread::spawn(move || {
        loop {
            let now = Local::now();
            if now >= *target_time_clone {
                println!("The target time has been reached!");
                break;
            }

            // Print the time remaining
            print_time_remaining(*target_time_clone);

            // Generate a random sleep duration between 30 and 90 minutes
            let sleep_minutes = rand::thread_rng().gen_range(30..=90);
            println!("Next update in {} minutes.", sleep_minutes);
            thread::sleep(StdDuration::from_secs((sleep_minutes * 60) as u64));
        }
    });

    // Main thread to handle user input for manual updates
    let target_time_clone = Arc::clone(&target_time);
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let now = Local::now();
        if now >= *target_time_clone {
            println!("The target time has been reached!");
            break;
        }

        // Print the time remaining
        print_time_remaining(*target_time_clone);
    }

    // Wait for the periodic update thread to finish
    handle.join().unwrap();
}

// Counting down the time to when I'll meet my better half :)
