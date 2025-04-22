use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, path::Path};

use image::Rgb;
use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSliceMut;

const CHUNK: usize = 1024;
const PIXEL_SIZE: usize = 4;

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

    img.par_chunks_mut(CHUNK * PIXEL_SIZE).for_each(|chunk| {
        chunk.chunks_exact_mut(PIXEL_SIZE).for_each(|pixel| {
            let pixel: &mut [u8; 3] = &mut pixel[..3].try_into().unwrap();
            let p = Rgb([pixel[0], pixel[1], pixel[2]]);
            let closest_color = palette
                .iter()
                .copied()
                .min_by(|&a, &b| diff(p, a).total_cmp(&diff(p, b)))
                .unwrap();
            let closest_color = closest_color.0;
            *pixel = closest_color;
        });
    });

    // Save the new image
    img.save("output.png").expect("Failed to save image");
}

fn diff(Rgb(a): Rgb<u8>, Rgb(b): Rgb<u8>) -> f32 {
    let diff = |idx| i32::abs_diff(a[idx] as i32, b[idx] as i32);
    let r_diff = diff(0);
    let g_diff = diff(1);
    let b_diff = diff(2);

    f32::sqrt((r_diff * r_diff + g_diff * g_diff + b_diff * b_diff) as f32)
}

fn from_str(hex_code: &str) -> Result<image::Rgb<u8>, std::num::ParseIntError> {
    // u8::from_str_radix(src: &str, radix: u32) converts a string
    // slice in a given base to u8
    let r: u8 = u8::from_str_radix(&hex_code[1..3], 16)?;
    let g: u8 = u8::from_str_radix(&hex_code[3..5], 16)?;
    let b: u8 = u8::from_str_radix(&hex_code[5..7], 16)?;

    Ok(image::Rgb([r, g, b]))
}

fn parse_colors(file: impl AsRef<Path>) -> impl Iterator<Item = Rgb<u8>> {
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
