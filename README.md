[![Rust](https://github.com/kmgill/cr2_to_tiff_halpha/actions/workflows/rust.yml/badge.svg)](https://github.com/kmgill/cr2_to_tiff_halpha/actions/workflows/rust.yml)

# cr2_to_tiff_halpha
Converts a Canon EOS raw image file (CR2) to a 16bit tiff, optimized for full-disk hydrogen-alpha imaging.

Warning: I am attempting to learn programming Rust and this is my first project to use it. I'm sorta just throwing code at it as I learn the language. Refactorization, optimization, and simplication will come later.


## Usage:
```
cr2_to_tiff_halpha 0.1.0
Kevin M. Gill <apoapsys@gmail.com>

USAGE:
    cr2_to_tiff_halpha [OPTIONS] --inputs <INPUTS>... --operation <OPERATION>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --dark <DARK>              Master dark file
    -f, --flat <FLAT>              Master flat file
    -i, --inputs <INPUTS>...       Input raws
    -o, --operation <OPERATION>    Processing operation [possible values: convert, mean]
    -O, --output <OUTPUT>          Output
```

## Examples:
Examples currently reflect execution from the development environment. I haven't made it any futher than that...

### Create Master Dark/Flats:
`$ cargo run --bin mkmean -- -i /data/Astrophotography/Sun/2021-03-16/dark/*CR2 -O /data/Astrophotography/Sun/2021-03-16/dark-v1.tif `

### Run using master dark & flat:
`cargo run --bin cr2totiff -- -i /data/Astrophotography/Sun/2021-03-16/light/IMG_*.CR2 -f /data/Astrophotography/Sun/2021-03-16/flat-v1.tif -d /data/Astrophotography/Sun/2021-03-16/dark-v1.tif`

### End-to-End Processing:
`cargo run --bin proc_ha -- -i /data/Astrophotography/Sun/2021-03-16/light/IMG_*.CR2 -f /data/Astrophotography/Sun/2021-03-16/flat/*CR2  -d /data/Astrophotography/Sun/2021-03-16/dark/*CR2 -O /data/Astrophotography/Sun/2021-03-16/test-stack-v1.tif`
