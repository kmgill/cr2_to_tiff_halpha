

use crate::imagebuffer::{MinMax, ImageBuffer};
use crate::path;
use crate::constants;

use std::process;

extern crate image;

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



fn create_scaled_final(source:&ImageBuffer, mn_mx:&MinMax) -> ImageBuffer {

    let mut dest = ImageBuffer::new(source.width, source.height).unwrap();

    for y in 0..source.height {
        for x in 0..source.width {
            let mut val_f32 :f32 = source.get(x, y).expect(constants::OK) as f32;
            val_f32 = (val_f32 - mn_mx.min) / (mn_mx.max - mn_mx.min) * (constants::_16_BIT_MAX as f32);
            dest.put(x, y, val_f32).expect(constants::OK);
        }
    }
    return dest
}


// Processes an input CR2 raw image file (Canon EOS)
pub fn process_file(raw_file:&str, flat:&ImageBuffer) {

    let source = ImageBuffer::from_cr2(raw_file).unwrap();

    let mean_flat = flat.mean();
    println!("    Flat Mean Value: {}", mean_flat);
    println!("    Flat Buffer Width: {}", flat.width);
    println!("    Flat Buffer Height: {}", flat.height);

    let out_file = raw_file.replace("CR2", "tif").replace("cr2", "tif");
    println!("    Determined output file path to be {}", out_file);

    let mn_mx = source.get_min_max(constants::SENSOR_DARK_LEVEL).unwrap();

    println!("    Pixel Min/Max (Red only): {}, {}", mn_mx.min, mn_mx.max);

    let red = source.red().unwrap();

    // TODO: Try implementing this correctly...
    let corrected =  red;//red.scale(mean_flat).unwrap().divide(&flat).unwrap();

    let scaled = create_scaled_final(&corrected, &mn_mx);
    println!("    Scaled Red-Only Buffer Width: {}", scaled.width);
    println!("    Scaled Red-Only Buffer Height: {}", scaled.height);

    
    let offset = corrected.calc_center_of_mass_offset(constants::DEFAULT_CENTER_OF_MASS_THRESHOLD).unwrap();
    println!("    Horizonal center of Mass Offset: {}", offset.h);
    println!("    Vertical Center of Mass Offset: {}", offset.v);

    let shifted = corrected.shift(offset.h, offset.v).unwrap();
    let cropped = shifted.crop(1400, 1400).unwrap();

    cropped.save(&out_file).expect(constants::OK);
}

pub fn run_convert(file_list:Vec<&str>) {

    // TODO: This should be optional and parameterized. And with a dark & bias
    let flat = ImageBuffer::from_file("/data/Astrophotography/Sun/flat.tif").unwrap();

    for in_file in file_list.iter() {
        if path::file_exists(in_file) {
            println!("Processing File: {}", in_file);
            process_file(&in_file, &flat);
        } else {
            eprintln!("File not found: {}", in_file);
            process::exit(1);
        }
    }
}