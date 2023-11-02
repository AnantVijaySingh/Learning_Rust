extern crate image;
use image::{ImageBuffer, Rgb};
use std::path::Path;
use std::fs;
use rand::Rng;

fn main() {
    let width = 1000;
    let height = 1000;
    let num_images = 5000; // Number of images to generate

    // Create a directory to store the image files
    let _ = fs::create_dir("image_files");

    // Create a separate image with a random color for each iteration
    for i in 0..num_images {
        // Generate a random color
        let color = generate_random_color();

        // Create an image with the specified color
        let imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_pixel(width, height, color);

        // Specify the file path with an index and store it in the "image_files" directory
        let file_path = format!("image_files/random_color_image{}.jpg", i);

        // Save the image as a JPEG file
        imgbuf.save(Path::new(&file_path)).unwrap();
    }
}

fn generate_random_color() -> Rgb<u8> {
    let mut rng = rand::thread_rng();
    let red = rng.gen_range(0..256);
    let green = rng.gen_range(0..256);
    let blue = rng.gen_range(0..256);
    Rgb([red as u8, green as u8, blue as u8])
}
