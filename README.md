# rustbrot
Simple Mandelbrot renderer in Rust

## main.rs
Contains the `main` function and a function that prints information about the current configuration.

## renderer.rs
Contains all the functions that calculate and save the mandelbrot,
as well as other utility functions.

#### Calculating Functions
* `draw_mandelbrot`
* `z`

#### Drawing Functions
* `get_color`
* `save_render_as_png`

## settings.rs
Contains all the settings about how the mandelbrot is drawn.

* How many times `z()` is iterated per pixel
* Image size in pixels
* Drawing range on the complex plane
* Path to where the image is saved

# TODO
* Maybe add multithreading
