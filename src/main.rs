use std::io::Write;

mod args;
use crate::args::{
    parse_args,
    usage,
};

mod mandelbrot;
use crate::mandelbrot::generate_pixels;

mod io;
use crate::io::write_image;

fn main() {
    // get command-line arguments
    let (filename, options) = match parse_args() {
        Ok(n) => n,
        Err(command) => {
            return usage(&command);
        },
    };

    // generate the image
    let pixels = generate_pixels(options);
    match write_image(&filename, &pixels, options.0) {
        Ok(_) => (),
        Err(_) => writeln!(std::io::stderr(), "Error writing PNG file").unwrap(),
    };
}
