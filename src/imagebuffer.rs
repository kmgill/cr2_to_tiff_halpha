
use crate::path;
use std::process;

extern crate image;
use image::{DynamicImage, Rgb};

pub struct ImageBuffer {
    buffer: Vec<u16>,
    width: u32,
    height: u32,
}

impl ImageBuffer {
    pub(crate) fn new(width:u32, height:u32) -> Self {
        Self{buffer:Vec::with_capacity(width as usize * height as usize),
            width:width,
            height:height
        }
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

    pub fn save(&self, to_file:&str) {
        let mut out_img = DynamicImage::new_rgb16(self.width, self.height).into_rgb16();
        
        for y in (0..self.width) {
            for x in (0..self.height) {
                let val = self.get(x, y);
                out_img.put_pixel(x, y, Rgb([val, 0, 0]));
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

