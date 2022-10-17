use std::io::Write;

use crate::mandelbrot as mandelbrot;
use crate::io as io;
use crate::args as args;

pub fn go() -> () {
    // get command-line arguments
    let (filename, options) = match args::parse() {
        Ok(n) => n,
        Err(command) => {
            return args::usage(&command);
        },
    };

    // generate the image
    let pixels = mandelbrot::generate_pixels(options);
    match io::write_image(&filename, &pixels, options.0) {
        Ok(_) => (),
        Err(_) => writeln!(std::io::stderr(), "Error writing PNG file").unwrap(),
    };
}
