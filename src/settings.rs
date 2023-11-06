pub const ITERATIONS: usize = 80;
pub const THRESHOLD: f64 = 2.0;
pub const COLOR_SPACE: usize = 255;

pub const ASPECT_RATIO: f64 = (REAL_MIN.abs() + REAL_MAX.abs()) as f64 / (IMAG_MIN.abs() + IMAG_MAX.abs()) as f64;
pub const WIDTH: usize = (HEIGHT as f64 * ASPECT_RATIO) as usize;
pub const HEIGHT: usize = 1080;

pub const REAL_MIN: isize = -2;
pub const REAL_MAX: isize = 1;
pub const IMAG_MIN: isize = -1;
pub const IMAG_MAX: isize = 1;

pub const IMAGE_PATH: &str = r"mandelbrot.png";