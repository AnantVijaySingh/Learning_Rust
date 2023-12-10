use std::fs::OpenOptions;
use std::io::{Write, Error};
use std::thread;
use std::time::Duration;
use rand::prelude::*;

fn update_file(file_path: &str) -> Result<(), Error> {
    // Generate random data
    let random_data: u32 = random();

    // Open the file in write mode, create it if it doesn't exist, and append to it
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_path)?;

    // Write the random data to the file
    writeln!(file, "{}", random_data)?;

    println!("Random data written to the file {}: {}", file_path, random_data);

    Ok(())
}

fn main() {
    // Number of files
    let file_count = 10000;

    // Number of times to update each file
    let update_count = 1;

    // Create and update files in parallel
    let handles: Vec<_> = (0..file_count)
        .map(|i| {
            let file_path = format!("/Users/anantvijaysingh/Library/CloudStorage/ProtonDrive-anantv@proton.me/Hehehe/file_10sec_{}.txt", i);
            let cloned_path = file_path.clone();

            thread::spawn(move || {
                for _ in 0..update_count {
                    if let Err(err) = update_file(&cloned_path) {
                        eprintln!("Error updating {}: {}", cloned_path, err);
                    }

                    // Sleep for 30 seconds before the next update
                    thread::sleep(Duration::from_secs(5));
                }
            })
        })
        .collect();

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}





// use std::fs::OpenOptions;
// use std::io::{Write, Error};
// use std::thread::sleep;
// use std::time::Duration;
// use rand::prelude::*;
//
// fn main() -> Result<(), Error> {
//     let file_path = "/Users/anantvijaysingh/Library/CloudStorage/ProtonDrive-anantv@proton.me/Hahahahaha/file.txt"; // Replace with the actual file path
//
//     // Number of times to edit the file
//     let edit_count = 210;
//
//     for _ in 0..edit_count {
//         // Generate random data
//         let random_data: u32 = random();
//
//         // Open the file in write mode, create it if it doesn't exist, and append to it
//         let mut file = OpenOptions::new()
//             .write(true)
//             .create(true)
//             .append(true)
//             .open(file_path)?;
//
//         // Write the random data to the file
//         writeln!(file, "{}", random_data)?;
//
//         println!("Random data written to the file: {}", random_data);
//
//         // File is automatically closed when it goes out of scope
//
//         // Sleep for 30 seconds before the next edit
//         sleep(Duration::from_secs(5));
//     }
//
//     Ok(())
// }
