
mod path;
mod raw_to_tiff;
mod imagebuffer;
mod constants;
mod mean;

use clap::{Arg, App};

fn main() {

    let matches = App::new("cr2_to_tiff_alpha")
                    .version("0.1.0")
                    .author("Kevin M. Gill <apoapsys@gmail.com>")
                    .arg(Arg::with_name(constants::OP_OPERATION_INPUT)
                        .short("o")
                        .long("operation")
                        .value_name("OPERATION")
                        .help("Processing operation")
                        .required(true)
                        .takes_value(true))
                    .arg(Arg::with_name("inputs")
                        .short("i")
                        .long("inputs")
                        .value_name("INPUTS")
                        .help("Input raws")
                        .required(true)
                        .multiple(true)
                        .takes_value(true)
                        )
                    .get_matches();


    let vals: Vec<&str> = matches.values_of("inputs").unwrap().collect();
    if matches.value_of(constants::OP_OPERATION_INPUT) == Some(constants::OP_CONVERT) {
        raw_to_tiff::run_convert(vals);
    } else if matches.value_of(constants::OP_OPERATION_INPUT) == Some(constants::OP_CALC_MEAN) {
        mean::run_mean_stack(vals);
    }

}
