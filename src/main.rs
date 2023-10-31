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
    println!("Drawing Mandelbrot with settings:");
    println!("\tRange ({} + {}i) to ({} + {}i)", REAL_MIN, IMAG_MIN, REAL_MAX, IMAG_MAX);
    println!("\tResolution {} x {}", WIDTH, HEIGHT);
    println!("\t{} Iterations", ITERATIONS);
}