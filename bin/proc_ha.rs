
use cr2_to_tiff_halpha::{constants, print, vprintln, path, imagebuffer, raw_to_tiff, mean};

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
                        .help("Master dark file(s)")
                        .required(false)
                        .multiple(true)
                        .takes_value(true))
                    .arg(Arg::with_name(constants::param::PARAM_FLAT)
                        .short(constants::param::PARAM_FLAT_SHORT)
                        .long(constants::param::PARAM_FLAT)
                        .value_name("FLAT")
                        .help("Master flat file(s)")
                        .required(false)
                        .multiple(true)
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
                    .arg(Arg::with_name(constants::param::PARAM_OUTPUT)
                        .short(constants::param::PARAM_OUTPUT_SHORT)
                        .long(constants::param::PARAM_OUTPUT)
                        .value_name("OUTPUT")
                        .help("Output")
                        .required(true)
                        .takes_value(true))
                    .get_matches();


    let lights: Vec<&str> = matches.values_of(constants::param::PARAM_INPUTS).unwrap().collect();
    let darks: Vec<&str> = matches.values_of(constants::param::PARAM_DARK).unwrap().collect();
    let flats: Vec<&str> = matches.values_of(constants::param::PARAM_FLAT).unwrap().collect();

    if matches.is_present(constants::param::PARAM_VERBOSE) {
        print::set_verbose(true);
    }

    
    // Change to use median
    let darks_stack = mean::process_mean(darks).unwrap();
    let flats_stack = mean::process_mean(flats).unwrap();

    let mut stack = imagebuffer::ImageBuffer::new(1, 1).unwrap();
    let mut cnt = 0;

    for in_file in lights.iter() {
        if path::file_exists(in_file) {
            vprintln!("Processing File: {}", in_file);
            
            let calibrated = raw_to_tiff::calibrate_raw(in_file, &flats_stack, &darks_stack).unwrap();

            if cnt == 0 {
                stack = calibrated;
            } else {
                stack = stack.add(&calibrated).unwrap();
            }

            cnt = cnt + 1;
        } else {
            eprintln!("File not found: {}", in_file);
        }
    }

    let output = matches.value_of(constants::param::PARAM_OUTPUT).unwrap();
    if cnt > 0 {
        stack = stack.scale(1.0 / cnt as f32).unwrap();
        let stackmm = stack.get_min_max(-1.0).unwrap();
        vprintln!("    Stack Min/Max : {}, {} ({} images)", stackmm.min, stackmm.max, cnt);
        stack.save(output).expect(constants::status::OK);
    } else {
        eprintln!("No files used");
    }
    
}