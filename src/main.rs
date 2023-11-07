pub use complex_type::complex::Complex;
pub use icecream::ic;
use mandelbrot::renderer as r;
use mandelbrot::settings::*;

fn main() {
    print_mandelbrot_info();

    let image_data = r::draw_mandelbrot();
    r::save_render_as_png(image_data);
}


fn print_mandelbrot_info() {
    let range_min = Complex::new(REAL_MIN, IMAG_MIN);
    let range_max = Complex::new(REAL_MAX, IMAG_MAX);

    println!("Drawing Mandelbrot with settings:");
    println!("\tRange ({}) to ({})", range_min, range_max);
    println!("\tResolution {} x {}", WIDTH, HEIGHT);
    println!("\t{} Iterations", ITERATIONS);
    println!("\tTo {}", IMAGE_PATH);
}