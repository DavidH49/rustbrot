pub const ITERATIONS: usize = 80;
pub const THRESHOLD: f64 = 2.0;
pub const COLOR_RANGE: usize = 255;

// Aspect ratio is based on the complex range to render
// Used a terrible way of getting an absolute value here that somewhat works
pub const ASPECT_RATIO: f64 = (-REAL_MIN + REAL_MAX) / (-IMAG_MIN + IMAG_MAX);

// width = height * aspect
pub const WIDTH: usize = (HEIGHT as f64 * ASPECT_RATIO) as usize;
pub const HEIGHT: usize = 1080;

pub const REAL_MIN: f64 = -2.0;
pub const REAL_MAX: f64 = 1.0;
pub const IMAG_MIN: f64 = -1.0;
pub const IMAG_MAX: f64 = 1.0;

pub const IMAGE_PATH: &str = r"mandelbrot.png";