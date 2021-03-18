

use crate::imagebuffer::ImageBuffer;
use crate::path;
use crate::constants;

extern crate image;

/*
fn linear_to_srgb(lin:f32) -> f32 {
    let mut v:f32 = lin;
    v = v / constants::_16_BIT_MAX as f32;
    if v > 0.0031308 {
        v = 1.055 * v.powf(1.0 / 2.4) - 0.055;
    } else {
        v = 12.92 * v;
    }
    v = v * constants::_16_BIT_MAX as f32;
    return v
}
*/

// Processes an input CR2 raw image file (Canon EOS)
fn process_file(raw_file:&str, flat:&ImageBuffer, dark:&ImageBuffer) {

    let source = ImageBuffer::from_cr2(raw_file).unwrap();

    let red = source.red().unwrap();

    let mut corrected = red;

    // Should support one or the other being left out
    if !dark.is_empty() && !flat.is_empty() {
        let darkflat = flat.subtract(&dark).unwrap();

        let mean_flat = darkflat.mean();
        println!("    Dark/Flat Mean Value: {}", mean_flat);

        let red_minus_dark = corrected.subtract(&dark).unwrap();

        // Over-simplification:
        corrected = red_minus_dark.scale(mean_flat).unwrap().divide(&flat).unwrap();
    }

    let scaled = corrected.normalize(0.0, constants::_16_BIT_MAX).unwrap();
    println!("    Scaled Red-Only Buffer Width: {}", scaled.width);
    println!("    Scaled Red-Only Buffer Height: {}", scaled.height);


    let offset = scaled.calc_center_of_mass_offset(20000.0).unwrap();
    println!("    Horizonal center of Mass Offset: {}", offset.h);
    println!("    Vertical Center of Mass Offset: {}", offset.v);

    let shifted = scaled.shift(offset.h, offset.v).unwrap();
    let cropped = shifted.crop(1400, 1400).unwrap();

    let scaled2 = cropped.normalize(0.0, constants::_16_BIT_MAX).unwrap();

    let out_file = raw_file.replace("CR2", "tif").replace("cr2", "tif");
    println!("    Determined output file path to be {}", out_file);
    scaled2.save(&out_file).expect(constants::OK);
}

pub fn run_convert(file_list:Vec<&str>, dark_file:&str, flat_file:&str) {

    println!("Flat File: {}", flat_file);
    println!("Dark File: {}", dark_file);

    if !flat_file.is_empty() && !path::file_exists(flat_file) {
        eprintln!("Flat file not found: {}", flat_file);
        return;
    }

    if !dark_file.is_empty() && !path::file_exists(dark_file) {
        eprintln!("Dark file not found: {}", dark_file);
        return;
    }

    let flat = if flat_file.is_empty() { ImageBuffer::new_empty().unwrap() } else { ImageBuffer::from_file(flat_file).unwrap() };
    let dark = if dark_file.is_empty() { ImageBuffer::new_empty().unwrap() } else { ImageBuffer::from_file(dark_file).unwrap() };

    for in_file in file_list.iter() {
        if path::file_exists(in_file) {
            println!("Processing File: {}", in_file);
            process_file(&in_file, &flat, &dark);
        } else {
            eprintln!("File not found: {}", in_file);
        }
    }
}