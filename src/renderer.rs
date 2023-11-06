use complex_type::complex::Complex;
use crate::settings::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

// Draws, as the name suggests, the mandelbrot and returns a Vec<u8> for exporting
pub fn draw_mandelbrot() -> Vec<u8> {
    let mut image_data: Vec<u8> = Vec::new();

    for im in 0..HEIGHT {
        for re in 0..WIDTH {
            let c = get_grid_position(re, im);
            let m = z(c);

            let color = get_color(m);
            image_data.push(color); // Red
            image_data.push(color); // Green
            image_data.push(color); // Blue
            image_data.push(255);   // Alpha
        }
    }

    image_data
}


// Repeats (z = z² + c) until |z| > 2 or n >= ITERATIONS; Then returns n
pub fn z(c: Complex) -> usize {
    let mut z = Complex::zero();
    let mut n = 0;

    while z.abs() <= Complex::uniform(THRESHOLD) && n < ITERATIONS {
        z = z * z + c;
        n += 1;
    }

    n
}


pub fn get_grid_position(re: usize, im: usize) -> Complex {
    Complex::new(
        REAL_MIN as f64 + (re as f64 / WIDTH as f64) * (REAL_MAX - REAL_MIN) as f64,
        IMAG_MIN as f64 + (im as f64 / HEIGHT as f64) * (IMAG_MAX - IMAG_MIN) as f64
    )
}


// Calculates the color for parameter m; 255 is black, 0 is white
pub fn get_color(m: usize) -> u8 {
    (COLOR_SPACE - (m * COLOR_SPACE / ITERATIONS)) as u8
}


// Saves image_data to IMAGE_PATH as PNG
pub fn save_render_as_png(image_data: Vec<u8>) {
    let path = Path::new(IMAGE_PATH);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, WIDTH as u32, HEIGHT as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    //encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&*image_data).unwrap();
}