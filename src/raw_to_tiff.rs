

use crate::imagebuffer::ImageBuffer;
use crate::path;


use std::process;
use std::fs;

const _14_BIT_MAX : u16 = 16383;
const _16_BIT_MAX : u16 = std::u16::MAX;

//http://lclevy.free.fr/cr2/
const SENSOR_DARK_LEVEL : u16 = 1023;

extern crate image;

use image::{DynamicImage, Rgb};


struct Offset {
    h: i32,
    v: i32,
}

struct MinMax {
    min: f32,
    max: f32,
}

fn calc_center_of_mass_offset(source:&ImageBuffer, threshold:u16) -> Offset {
    let mut ox: f32 = 0.0;
    let mut oy: f32 = 0.0;
    let mut count: u32 = 0;

    let mut offset = Offset{h:0, v:0};

    for y in (0..source.height).step_by(2) {
        for x in (0..source.width).step_by(2) {
            let val = source.get(x, y);
            if val >= threshold {
                ox = ox + (x as f32);
                oy = oy + (y as f32);
                count = count + 1;
            }   
        }
    }

    if count > 0 {
        ox = (source.width as f32 / 2.0) - (ox / (count as f32));
        oy = (source.height as f32 / 2.0) - (oy / (count as f32));
    }

    Offset{h:ox.round() as i32, v:oy.round() as i32}
}


fn linear_to_srgb(lin:f32) -> f32 {
    let mut v:f32 = lin;
    v = v / 65535.0;
    if v > 0.0031308 {
        v = 1.055 * v.powf(1.0 / 2.4) - 0.055;
    } else {
        v = 12.92 * v;
    }
    v = v * 65535.0;
    return v
}

// Determined the minimum and maximum values within the 
// red pixel channel.
fn get_min_max(source:&ImageBuffer) -> MinMax {
    
    let mut mx:f32 = 0.0;
    let mut mn:f32 = _14_BIT_MAX as f32;

    for y in (0..source.height).step_by(2) {
        for x in (0..source.width).step_by(2) {
            let val = source.get(x, y) as f32;
            mx = if val > mx { val } else { mx };
            mn = if val < mn { val } else { mn };
        }
    }
    // Sensor dark level override.. For now...
    mn = SENSOR_DARK_LEVEL as f32;

    MinMax{min:mn, max:mx}
}


fn create_scaled_final(source:&ImageBuffer, mn_mx:&MinMax) -> ImageBuffer {

    let dest_height = source.height / 2;
    let dest_width = source.width / 2;

    let mut dest = ImageBuffer::new(dest_width, dest_height);

    for y in (0..source.height).step_by(2) {
        for x in (0..source.width).step_by(2) {
            let put_x = x / 2;
            let put_y = y / 2;
            
            let mut val_f32 :f32 = source.get(x, y) as f32;

            val_f32 = (val_f32 - mn_mx.min) / (mn_mx.max - mn_mx.min) * (_16_BIT_MAX as f32);
            //val_f32 = linear_to_srgb(val_f32);
            let val:u16 = val_f32.round() as u16;

            dest.put(put_x, put_y, val);
        }
    }

    return dest
}



fn load_raw_image(raw_file:&str) -> ImageBuffer {
    println!("    Reading raw image file {}", raw_file);
    let buf = fs::read(raw_file).expect("read in");

    println!("    Decoding for raw pixel values");
    let processor = libraw::Processor::new();
    let raw_image = processor.decode(&buf).unwrap();
    ImageBuffer::from_libraw(&raw_image)
}

// Processes an input CR2 raw image file (Canon EOS)
pub fn process(raw_file:&str) {

    let source = load_raw_image(raw_file);


    let out_file = raw_file.replace("CR2", "tif").replace("cr2", "tif");
    println!("    Determined output file path to be {}", out_file);

    let mn_mx = get_min_max(&source);

    println!("    Pixel Min/Max (Red only): {}, {}", mn_mx.min, mn_mx.max);

    let offset = calc_center_of_mass_offset(&source, SENSOR_DARK_LEVEL);

    println!("    Horizonal center of Mass Offset: {}", offset.h);
    println!("    Vertical Center of Mass Offset: {}", offset.v);

    let scaled = create_scaled_final(&source, &mn_mx);

    scaled.save(&out_file);
    //raw_image_to_image_buffer(&raw_image, &out_file, mn_mx);
}