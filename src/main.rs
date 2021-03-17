
mod path;
mod raw_to_tiff;
mod imagebuffer;
mod constants;
mod mean;

#[macro_use]
extern crate clap;

use clap::{Arg, App};


fn main() {

    let matches = App::new(crate_name!())
                    .version(crate_version!())
                    .author(crate_authors!())
                    .arg(Arg::with_name(constants::PARAM_OPERATION)
                        .short(constants::PARAM_OPERATION_SHORT)
                        .long(constants::PARAM_OPERATION)
                        .value_name("OPERATION")
                        .help("Processing operation")
                        .required(true)
                        .possible_values(&[constants::OP_CONVERT, constants::OP_CALC_MEAN])
                        .takes_value(true))
                    .arg(Arg::with_name(constants::PARAM_OUTPUT)
                        .short(constants::PARAM_OUTPUT_SHORT)
                        .long(constants::PARAM_OUTPUT)
                        .value_name("OUTPUT")
                        .help("Output")
                        .required(false)
                        .takes_value(true))
                    .arg(Arg::with_name(constants::PARAM_DARK)
                        .short(constants::PARAM_DARK_SHORT)
                        .long(constants::PARAM_DARK)
                        .value_name("DARK")
                        .help("Master dark file")
                        .required(false)
                        .takes_value(true))
                    .arg(Arg::with_name(constants::PARAM_FLAT)
                        .short(constants::PARAM_FLAT_SHORT)
                        .long(constants::PARAM_FLAT)
                        .value_name("FLAT")
                        .help("Master flat file")
                        .required(false)
                        .takes_value(true))
                    .arg(Arg::with_name(constants::PARAM_INPUTS)
                        .short(constants::PARAM_INPUTS_SHORT)
                        .long(constants::PARAM_INPUTS)
                        .value_name("INPUTS")
                        .help("Input raws")
                        .required(true)
                        .multiple(true)
                        .takes_value(true)
                        )
                    .get_matches();


    let vals: Vec<&str> = matches.values_of(constants::PARAM_INPUTS).unwrap().collect();

    if matches.value_of(constants::PARAM_OPERATION) == Some(constants::OP_CONVERT) {

        let dark = if (matches.value_of(constants::PARAM_DARK) == None) { constants::EMPTY } else { matches.value_of(constants::PARAM_DARK).unwrap() };
        let flat = if (matches.value_of(constants::PARAM_FLAT) == None) { constants::EMPTY } else { matches.value_of(constants::PARAM_FLAT).unwrap() };
        raw_to_tiff::run_convert(vals, dark, flat);

    } else if matches.value_of(constants::PARAM_OPERATION) == Some(constants::OP_CALC_MEAN) {

        if matches.value_of(constants::PARAM_OUTPUT) == None {
            eprintln!("Error: Output path parameter required for stack output");
        } else {
            let output = matches.value_of(constants::PARAM_OUTPUT).unwrap();
            mean::run_mean_stack(vals, output); 
        }
        
    }

}
