
use cr2_to_tiff_halpha::{mean, constants, print};

#[macro_use]
extern crate clap;

use clap::{Arg, App};

fn main() {
    
    let matches = App::new(crate_name!())
                    .version(crate_version!())
                    .author(crate_authors!())
                    .arg(Arg::with_name(constants::PARAM_OUTPUT)
                        .short(constants::PARAM_OUTPUT_SHORT)
                        .long(constants::PARAM_OUTPUT)
                        .value_name("OUTPUT")
                        .help("Output")
                        .required(true)
                        .takes_value(true))
                    .arg(Arg::with_name(constants::PARAM_INPUTS)
                        .short(constants::PARAM_INPUTS_SHORT)
                        .long(constants::PARAM_INPUTS)
                        .value_name("INPUTS")
                        .help("Input raws")
                        .required(true)
                        .multiple(true)
                        .takes_value(true))
                    .arg(Arg::with_name(constants::PARAM_VERBOSE)
                        .short(constants::PARAM_VERBOSE)
                        .help("Show verbose output"))
                    .get_matches();


    let vals: Vec<&str> = matches.values_of(constants::PARAM_INPUTS).unwrap().collect();

    if matches.is_present(constants::PARAM_VERBOSE) {
        print::set_verbose(true);
    }

    if matches.value_of(constants::PARAM_OUTPUT) == None {
        eprintln!("Error: Output path parameter required for stack output");
    } else {
        let output = matches.value_of(constants::PARAM_OUTPUT).unwrap();
        mean::run_mean_stack(vals, output); 
    }

}