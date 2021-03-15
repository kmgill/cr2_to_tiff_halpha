
use crate::path;
use std::process;

extern crate image;
use image::{DynamicImage, Rgb};

pub struct ImageBuffer {
    buffer: Vec<u16>,
    pub width: u32,
    pub height: u32,
}

impl ImageBuffer {
    pub(crate) fn new(width:u32, height:u32) -> ImageBuffer {

        let mut v:Vec<u16> = Vec::with_capacity(width as usize * height as usize);
        v.resize(width as usize * height as usize, 0);

        ImageBuffer{buffer:v,
            width:width,
            height:height
        }
    }

    pub(crate) fn from_vec(v:Vec<u16>, width:u32, height:u32) -> ImageBuffer {
        ImageBuffer{buffer:v,
                    width:width,
                    height:height
        }
    }

    pub fn from_libraw(raw_image:&libraw::RawImage) -> ImageBuffer {
        let top_margin:u32  = raw_image.sizes().top_margin as u32;
        let left_margin:u32 = raw_image.sizes().left_margin as u32;
        let h:u32 = raw_image.sizes().raw_height as u32;
        let w:u32 = raw_image.sizes().raw_width as u32;

        println!("    Raw pixel buffer height: {}", h);
        println!("    Raw pixel buffer width: {}", w);
        println!("    Raw pixel buffer top margin: {}", top_margin);
        println!("    Raw pixel buffer left margin: {}", left_margin);

        let need_len = (h - top_margin) * (w - left_margin);
        println!("    Creating vector of capacity {}", need_len);

        let mut v:Vec<u16> = Vec::with_capacity(need_len as usize);

        for y in 0..h {
            for x in 0..w {
                if y >= top_margin && x >= left_margin {
                    let idx = y * w + x;
                    let val = raw_image[idx as usize];
                    
                    let put_x = (x - left_margin);
                    let put_y = (y - top_margin);

                    //println!("    Putting value into index {}", (put_y * (w-left_margin) + put_x));
                    v.push(val);
                }
            }
        }

        ImageBuffer::from_vec(v, w-left_margin, h-top_margin)
    }  

    pub fn get(&self, x:u32, y:u32) -> u16 {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            return self.buffer[index as usize]
        } else {
            return 0 // TODO: learn to throw exceptions
        }
    }

    pub fn put(&mut self, x:u32, y:u32, val:u16) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.buffer[index as usize] = val;
        } else {
            // TODO: learn to throw exceptions
        }
    }


    pub fn crop(&self, height:u32, width:u32) -> ImageBuffer {

        let mut cropped_buffer = ImageBuffer::new(width, height);

        cropped_buffer.put(0, 0, 65535);

        return cropped_buffer
    }


    pub fn save(&self, to_file:&str) {
        let mut out_img = DynamicImage::new_rgb16(self.width, self.height).into_rgb16();
        
        for y in 0..self.height {
            for x in 0..self.width {
                let val = self.get(x, y);
                out_img.put_pixel(x, y, Rgb([val, val, val]));
            }
        }

        println!("    Writing image buffer to file");
        if path::parent_exists_and_writable(&to_file) {
            out_img.save(to_file).unwrap();
        } else {
            eprintln!("Parent does not exist or cannot be written: {}", path::get_parent(to_file));
            process::exit(1);
        }
    
    }
}

