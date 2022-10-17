extern crate num;
extern crate image;
extern crate crossbeam;
extern crate num_cpus;

use std::io::Write;
use num::Complex;
use std::str::FromStr;
use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0., im: 0. };

    for i in 0..limit {
        z = (z * z) + c;

        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    s.find(separator).map(| index | {
        match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None
        }
    }).flatten()
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    parse_pair(s, ',').map(|(re, im)| Complex { re, im })
}

fn pixel_to_point(bounds: (usize, usize),
                  pixel: (usize, usize),
                  upper_left: Complex<f64>,
                  lower_right: Complex<f64>)
    -> Complex<f64>
{
    let (width, height) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

fn render(pixels: &mut [u8],
          bounds: (usize, usize),
          upper_left: Complex<f64>,
          lower_right: Complex<f64>)
{
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0 .. bounds.1 {
        for column in 0 .. bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);

            pixels[row * bounds.0 + column] =
                match escape_time(point, 255) {
                    None => 0,
                    Some(count) => 255 - count as u8,
                };
        }
    }
}

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error>
{
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);
    encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32,
                   ColorType::Gray(8))?;

    Ok(())
}

type Options = ((usize, usize), Complex<f64>, Complex<f64>);

fn generate_pixels(pixels: &mut Vec<u8>, (bounds, upper_left, lower_right): Options) {
    let threads = num_cpus::get();
    let rows_per_band = bounds.1 / threads + 1;

    {
        let bands: Vec<&mut [u8]> =
            pixels.chunks_mut(rows_per_band * bounds.0).collect();

        crossbeam::scope(| spawner | {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right = pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

                spawner.spawn(move || {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        });
    }
}

fn get_args(args: &[String]) -> Result<Options, &str> {
    let bounds = parse_pair::<usize>(&args[2], 'x').ok_or("Could not parse image dimensions")?;
    let upper_left = parse_complex(&args[3]).ok_or("Could not parse upper left corner point")?;
    let lower_right = parse_complex(&args[4]).ok_or("Could not parse lower right corner point")?;

    Ok((bounds, upper_left, lower_right))
}

fn usage(args: &[String]) {
    writeln!(std::io::stderr(), "Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]).unwrap();
    writeln!(std::io::stderr(), "Example: {} mandel.png 1000x750 -1.2,0.35 -1.0,0.2", args[0]).unwrap();
    std::process::exit(1);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        return usage(&args);
    }

    let filename = &args[1];

    let options @ (bounds, _, _) = match get_args(&args) {
        Ok(n) => n,
        Err(e) => {
            writeln!(std::io::stderr(), "{}", e).unwrap();
            return usage(&args);
        }
    };

    let mut pixels = vec![0; bounds.0 * bounds.1];
    generate_pixels(&mut pixels, options);
    write_image(filename, &pixels, options.0).expect("error writing PNG file");
}
