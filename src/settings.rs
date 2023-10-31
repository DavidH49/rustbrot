use complex_type::complex::Complex;

pub const ITERATIONS: usize = 100;
pub const THRESHOLD: f64 = 2.0;
pub const COLOR_SPACE: usize = 255;

pub const WIDTH: usize = (HEIGHT as f64 * 1.5) as usize;
pub const HEIGHT: usize = 8000;

pub const REAL_MIN: isize = -2;
pub const REAL_MAX: isize = 1;
pub const IMAG_MIN: isize = -1;
pub const IMAG_MAX: isize = 1;

pub const IMAGE_PATH: &str = r"mandelbrot.png";
pub const DATA_SIZE: usize = WIDTH * HEIGHT * 4;


pub fn get_grid_position(re: usize, im: usize) -> Complex {
    Complex::new(
        REAL_MIN as f64 + (re as f64 / WIDTH as f64) * (REAL_MAX - REAL_MIN) as f64,
        IMAG_MIN as f64 + (im as f64 / HEIGHT as f64) * (IMAG_MAX - IMAG_MIN) as f64
    )
}