use std::env;
use std::fs::File;
use std::io::BufRead;

use image::{GenericImageView, Rgba};

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <colors> <image>", args[0]);
        return;
    }

    let colors_file = &args[1];
    let image_file = &args[2];

    let palette = parse_colors(colors_file);
    let img = image::open(image_file).expect(format!("Failed to open image file: {}", image_file).as_str());

    // Create a new image the same size as the file
    let (width, height) = img.dimensions();
    let mut new_img = image::RgbaImage::new(width, height);

    for pixel in img.pixels() {
        let x = pixel.0;
        let y = pixel.1;
        let pixel_color = pixel.2;

        // Find which color in the list is closest to the pixel color
        let mut closest_color = None;
        let mut closest_diff = f64::MAX;
        for color in &palette {
            let diff_value = diff(pixel_color, *color);
            if diff_value < closest_diff {
                closest_diff = diff_value;
                closest_color = Some(color.clone());
            }
        }
        if let Some(closest_color) = closest_color {
            new_img.put_pixel(x, y, closest_color);
        }

    }

    // Save the new image
    new_img.save("output.png").expect("Failed to save image");
}

fn diff(a: Rgba<u8>, b: Rgba<u8>) -> f64 {
    let r_diff = (a[0] as i32 - b[0] as i32).abs() as u32;
    let g_diff = (a[1] as i32 - b[1] as i32).abs() as u32;
    let b_diff = (a[2] as i32 - b[2] as i32).abs() as u32;

    f64::sqrt((r_diff * r_diff + g_diff * g_diff + b_diff * b_diff) as f64)
}

fn from_str(hex_code: &str) -> Result<image::Rgba<u8>, std::num::ParseIntError> {
    // u8::from_str_radix(src: &str, radix: u32) converts a string
    // slice in a given base to u8
    let r: u8 = u8::from_str_radix(&hex_code[1..3], 16)?;
    let g: u8 = u8::from_str_radix(&hex_code[3..5], 16)?;
    let b: u8 = u8::from_str_radix(&hex_code[5..7], 16)?;

    Ok(image::Rgba([r, g, b, 255]))
}

fn parse_colors(file: &str) -> Vec<Rgba<u8>> {
    let mut colors = Vec::new();
    let file = File::open(file).expect(format!("Failed to open colors file: {}", file).as_str());
    for line in std::io::BufReader::new(file).lines() {
        let line = line.expect("Failed to read line");
        colors.push(from_str(&line).expect(format!("Failed to parse color: {}", line).as_str()));
    }
    colors
}
