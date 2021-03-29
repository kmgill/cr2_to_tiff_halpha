
use cr2_to_tiff_halpha::{constants, print, raw_to_tiff};

#[macro_use]
extern crate clap;

use clap::{Arg, App};

fn main() { 
    let matches = App::new(crate_name!())
                    .version(crate_version!())
                    .author(crate_authors!())
                    .arg(Arg::with_name(constants::param::PARAM_DARK)
                        .short(constants::param::PARAM_DARK_SHORT)
                        .long(constants::param::PARAM_DARK)
                        .value_name("DARK")
                        .help("Master dark file")
                        .required(false)
                        .takes_value(true))
                    .arg(Arg::with_name(constants::param::PARAM_FLAT)
                        .short(constants::param::PARAM_FLAT_SHORT)
                        .long(constants::param::PARAM_FLAT)
                        .value_name("FLAT")
                        .help("Master flat file")
                        .required(false)
                        .takes_value(true))
                    .arg(Arg::with_name(constants::param::PARAM_INPUTS)
                        .short(constants::param::PARAM_INPUTS_SHORT)
                        .long(constants::param::PARAM_INPUTS)
                        .value_name("INPUTS")
                        .help("Input raws")
                        .required(true)
                        .multiple(true)
                        .takes_value(true))
                    .arg(Arg::with_name(constants::param::PARAM_VERBOSE)
                        .short(constants::param::PARAM_VERBOSE)
                        .help("Show verbose output"))
                    .get_matches();


    let vals: Vec<&str> = matches.values_of(constants::param::PARAM_INPUTS).unwrap().collect();

    if matches.is_present(constants::param::PARAM_VERBOSE) {
        print::set_verbose(true);
    }

    let dark = if matches.value_of(constants::param::PARAM_DARK) == None { constants::status::EMPTY } else { matches.value_of(constants::param::PARAM_DARK).unwrap() };
    let flat = if matches.value_of(constants::param::PARAM_FLAT) == None { constants::status::EMPTY } else { matches.value_of(constants::param::PARAM_FLAT).unwrap() };
    raw_to_tiff::run_convert(vals, dark, flat);
}