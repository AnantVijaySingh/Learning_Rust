use std::fs::OpenOptions;
use std::io::{Write, Error};
use std::thread::sleep;
use std::time::Duration;
use rand::prelude::*;

fn main() -> Result<(), Error> {
    let file_path = "/Users/anantvijaysingh/Library/CloudStorage/ProtonDrive-anantv@proton.me/ReleaseBuildMacOS/file.txt"; // Replace with the actual file path

    // Number of times to edit the file
    let edit_count = 210;

    for _ in 0..edit_count {
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

        println!("Random data written to the file: {}", random_data);

        // File is automatically closed when it goes out of scope

        // Sleep for 30 seconds before the next edit
        sleep(Duration::from_secs(5));
    }

    Ok(())
}
