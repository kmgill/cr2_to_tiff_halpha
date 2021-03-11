use std::fs;
const _14_BIT_MAX : u16 = 16383;

extern crate image;

use image::{DynamicImage, Rgb};

// Determined the minimum and maximum values within the 
// red pixel channel.
fn get_min_max(raw_image:&libraw::RawImage) -> (f32, f32) {

    let top_margin:u32  = raw_image.sizes().top_margin as u32;
    let left_margin:u32 = raw_image.sizes().left_margin as u32;

    let mut mx:f32 = 0.0;
    let mut mn:f32 = _14_BIT_MAX as f32;
    let h:u32 = raw_image.sizes().raw_height as u32;
    let w:u32 = raw_image.sizes().raw_width as u32;

    for y in (0..h).step_by(2) {
        for x in (0..w).step_by(2) {
            if y >= top_margin && x >= left_margin {
                let idx = y * w + x;
                let val_i32 :f32 = raw_image[idx as usize] as f32;
                mx = if val_i32 > mx { val_i32 } else { mx };
                mn = if val_i32 < mn { val_i32 } else { mn };
            }
        }
    }
    return (mn, mx)
}

// Transfers from the raw image to a tiff file. Converts values from 14bit representation
// to 16bit, normalized between supplied min & max. Only utilizes red channel, 
// optimized for hydrogen-alpha observations, and assumes a RGGB bayer pattern.
fn raw_image_to_image_buffer(raw_image:&libraw::RawImage, out_file : &str, mn_mx:(f32, f32)) {
    let top_margin:u32  = raw_image.sizes().top_margin as u32;
    let left_margin:u32 = raw_image.sizes().left_margin as u32;

    let h:u32 = raw_image.sizes().raw_height as u32;
    let w:u32 = raw_image.sizes().raw_width as u32;

    println!("    Raw pixel buffer height: {}", h);
    println!("    Raw pixel buffer width: {}", w);
    println!("    Raw pixel buffer top margin: {}", top_margin);
    println!("    Raw pixel buffer left margin: {}", left_margin);

    let mut out_img = DynamicImage::new_rgb16((w-left_margin)/2, (h-top_margin)/2).into_rgb16();
    println!("    Created output buffer of width/height of {}, {}", out_img.width(), out_img.height());

    for y in (0..h).step_by(2) {
        for x in (0..w).step_by(2) {
            if y >= top_margin && x >= left_margin {
                let idx = y * w + x;
                let mut val_f32 :f32 = raw_image[idx as usize] as f32;

                val_f32 = (val_f32 - mn_mx.0) / (mn_mx.1 - mn_mx.0) * 65535.0;

                let val:u16 = val_f32 as u16;

                let put_x = (x - left_margin) / 2;
                let put_y = (y - top_margin) / 2;

                if put_x < out_img.width() && put_y < out_img.height() {
                    out_img.put_pixel(put_x, put_y, Rgb([val, 0, 0]));
                } 
            }
        }
    }

    println!("    Writing image buffer to file");
    out_img.save(out_file).unwrap();
}

// Processes an input CR2 raw image file (Canon EOS)
pub fn process(raw_file:&str) {
    println!("    Reading raw image file {}", raw_file);
    let buf = fs::read(raw_file).expect("read in");

    println!("    Decoding for raw pixel values");
    let processor = libraw::Processor::new();
    let raw_image = processor.decode(&buf).unwrap();

    let out_file = raw_file.replace("CR2", "tif").replace("cr2", "tif");
    println!("    Determined output file path to be {}", out_file);

    let mn_mx = get_min_max(&raw_image);
    println!("    Pixel Min/Max (Red only): {}, {}", mn_mx.0, mn_mx.1);

    raw_image_to_image_buffer(&raw_image, &out_file, mn_mx);
}