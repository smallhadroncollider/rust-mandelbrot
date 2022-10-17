use std::io::Write;
use num::Complex;
use std::str::FromStr;

pub type Options = ((usize, usize), Complex<f64>, Complex<f64>);

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

fn parse_options(args: &[String]) -> Result<Options, &str> {
    let bounds = parse_pair::<usize>(&args[2], 'x').ok_or("Could not parse image dimensions")?;
    let upper_left = parse_complex(&args[3]).ok_or("Could not parse upper left corner point")?;
    let lower_right = parse_complex(&args[4]).ok_or("Could not parse lower right corner point")?;

    Ok((bounds, upper_left, lower_right))
}

pub fn usage(command: &String) {
    writeln!(std::io::stderr(), "Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", command).unwrap();
    writeln!(std::io::stderr(), "Example: {} mandel.png 1000x750 -1.2,0.35 -1.0,0.2", command).unwrap();
    std::process::exit(1);
}

pub fn parse_args() -> Result<(String, Options), String> {
    // get command-line arguments
    let args: Vec<String> = std::env::args().collect();
    let command = args[0].to_owned();

    if args.len() != 5 {
        return Err(command);
    }

    let filename = args[1].to_owned();

    let options = match parse_options(&args) {
        Ok(n) => n,
        Err(e) => {
            writeln!(std::io::stderr(), "{}", e).unwrap();
            return Err(command);
        }
    };

    Ok((filename, options))
}
