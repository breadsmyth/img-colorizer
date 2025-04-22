use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, path::Path};

use image::Rgba;

fn main() {
    // Get command line arguments
    let mut args = env::args();
    // SAFETY: This unwrap cannot fail as it captures the first argument, which is always the path
    // to the binary
    let binary_name = args.next().unwrap();

    let [Some(colors_file), Some(image_file)] = std::array::from_fn(|_| args.next()) else {
        eprintln!("Usage: {binary_name} <colors> <image>");
        return;
    };

    let palette: Vec<_> = parse_colors(colors_file).collect();
    let img = image::open(&image_file)
        .unwrap_or_else(|_| panic!("Failed to open image file: {}", image_file));
    let mut img = img.into_rgba8();

    for pixel in img.pixels_mut() {
        // Find which color in the list is closest to the pixel color
        let mut closest_color = Rgba([0; 4]);
        let mut closest_diff = f32::MAX;
        for &color in &palette {
            let diff_value = diff(*pixel, color);
            if diff_value < closest_diff {
                closest_diff = diff_value;
                closest_color = color;
            }
        }
        *pixel = closest_color;
    }

    // Save the new image
    img.save("output.png").expect("Failed to save image");
}

fn diff(Rgba(a): Rgba<u8>, Rgba(b): Rgba<u8>) -> f32 {
    let diff = |idx| i32::abs_diff(a[idx] as i32, b[idx] as i32);
    let r_diff = diff(0);
    let g_diff = diff(1);
    let b_diff = diff(2);

    f32::sqrt((r_diff * r_diff + g_diff * g_diff + b_diff * b_diff) as f32)
}

fn from_str(hex_code: &str) -> Result<image::Rgba<u8>, std::num::ParseIntError> {
    // u8::from_str_radix(src: &str, radix: u32) converts a string
    // slice in a given base to u8
    let r: u8 = u8::from_str_radix(&hex_code[1..3], 16)?;
    let g: u8 = u8::from_str_radix(&hex_code[3..5], 16)?;
    let b: u8 = u8::from_str_radix(&hex_code[5..7], 16)?;

    Ok(image::Rgba([r, g, b, 255]))
}

fn parse_colors(file: impl AsRef<Path>) -> impl Iterator<Item = Rgba<u8>> {
    let file = file.as_ref();
    let file = match File::open(file) {
        Ok(file) => file,
        Err(err) => {
            panic!("Failed to open colors file at path {file:?} with error {err}")
        }
    };
    let file = BufReader::new(file);
    file.lines().map(|line| {
        let line = line.expect("Failed to read line");
        match from_str(&line) {
            Ok(color) => color,
            Err(err) => panic!("Failed to parse color: {}, {err}", line),
        }
    })
}
