use std::fs::{self, File};
use std::io::Write;

const _14_BIT_MAX : u16 = 16383;

/*
fn get_min_max(raw_image:&libraw::RawImage) -> (f32, f32) {

    let top_margin:i32  = raw_image.sizes().top_margin as i32;
    let left_margin:i32 = raw_image.sizes().left_margin as i32;

    let mut mx:f32 = 0.0;
    let mut mn:f32 = _14_BIT_MAX as f32;
    let h:i32 = raw_image.sizes().raw_height as i32;
    let w:i32 = raw_image.sizes().raw_width as i32;
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


fn raw_image_to_vector(raw_image:&libraw::RawImage, vec:&mut Vec<u8>, mn_mx:(f32, f32)) {
    let top_margin:i32  = raw_image.sizes().top_margin as i32;
    let left_margin:i32 = raw_image.sizes().left_margin as i32;

    let h:i32 = raw_image.sizes().raw_height as i32;
    let w:i32 = raw_image.sizes().raw_width as i32;
    for y in (0..h).step_by(2) {
        for x in (0..w).step_by(2) {
            if y >= top_margin && x >= left_margin {
                let idx = y * w + x;
                let mut val_f32 :f32 = raw_image[idx as usize] as f32;

                val_f32 = (val_f32 - mn_mx.0) / (mn_mx.1 - mn_mx.0) * 65535.0;

                let val:u16 = val_f32 as u16;
                vec.extend_from_slice(&val.to_be_bytes());
                vec.extend_from_slice(&(0 as u16).to_be_bytes());
                vec.extend_from_slice(&(0 as u16).to_be_bytes());
            }
        }
    }
}

pub fn process(raw_file:&str) {
    let buf = fs::read(raw_file).expect("read in");

    let processor = libraw::Processor::new();
    let raw_image = processor.decode(&buf).unwrap();

    let out_file = raw_file.replace("CR2", "ppm").replace("cr2", "ppm");

    let mut out = File::create(out_file).expect("create out");
    let header = format!(
        "P6 {} {} {}\n",
        raw_image.sizes().width / 2,
        raw_image.sizes().height / 2,
        65535
    );
    println!("{:?}", raw_image.sizes());
    out.write_all(header.as_ref()).expect("header");

    let mut out_vec:Vec<u8> = Vec::with_capacity(raw_image.sizes().raw_width as usize * raw_image.sizes().raw_width as usize * 2);
    println!("Len:  {}", raw_image.len());

    let mn_mx = get_min_max(&raw_image);
    println!("Min/Max: {}, {}", mn_mx.0, mn_mx.1);

    raw_image_to_vector(&raw_image, &mut out_vec, mn_mx);
    out.write_all(&out_vec).expect("pixels");
}
*/