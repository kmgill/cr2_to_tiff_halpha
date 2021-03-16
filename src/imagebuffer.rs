
use crate::path;
use crate::constants;
extern crate image;
use image::{open, DynamicImage, Rgb};
use std::fs;

// A simple image raster buffer.
pub struct ImageBuffer {
    buffer: Vec<f32>,
    pub width: u32,
    pub height: u32,
}

pub struct Offset {
    pub h: i32,
    pub v: i32,
}

pub struct MinMax {
    pub min: f32,
    pub max: f32,
}

impl ImageBuffer {

    // Creates a new image buffer of the requested width and height
    pub(crate) fn new(width:u32, height:u32) -> Result<ImageBuffer, &'static str> {

        let mut v:Vec<f32> = Vec::with_capacity(width as usize * height as usize);
        v.resize(width as usize * height as usize, 0.0);

        Ok(ImageBuffer{buffer:v,
            width:width,
            height:height
        })
    }

    // Creates a new image buffer at the requested width, height and data
    pub(crate) fn from_vec(v:Vec<f32>, width:u32, height:u32) -> Result<ImageBuffer, &'static str> {

        if v.len() != (width * height) as usize {
            return Err(constants::DIMENSIONS_DO_NOT_MATCH_VECTOR_LENGTH);
        }

        Ok(ImageBuffer{buffer:v,
                    width:width,
                    height:height
        })
    }

    pub fn from_file(file_path:&str) -> Result<ImageBuffer, &'static str> {

        if !path::file_exists(file_path) {
            return Err(constants::FILE_NOT_FOUND);
        }

        let image_data = open(file_path).unwrap().into_luma16();
        let dims = image_data.dimensions();

        let width = dims.0;
        let height = dims.1;
        println!("    Input image dimensions: {:?}", image_data.dimensions());

        let mut v:Vec<f32> = Vec::with_capacity((width * height) as usize);
        for y in 0..height {
            for x in 0..width {
                let pixel = image_data.get_pixel(x, y);
                let value = pixel[0] as f32;
                v.push(value);
            }
        }

        ImageBuffer::from_vec(v, width, height)
    }

    pub fn from_libraw(raw_image:&libraw::RawImage) -> Result<ImageBuffer, &str> {
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

        let mut v:Vec<f32> = Vec::with_capacity(need_len as usize);

        for y in 0..h {
            for x in 0..w {
                if y >= top_margin && x >= left_margin {
                    let idx = y * w + x;
                    let val = raw_image[idx as usize] as f32;
                    v.push(val);
                }
            }
        }

        ImageBuffer::from_vec(v, w-left_margin, h-top_margin)
    }  

    pub fn from_cr2(raw_file:&str) -> Result<ImageBuffer, &str> {
        println!("    Reading raw image file {}", raw_file);

        if !path::file_exists(raw_file) {
            return Err("File does not exist");
        }
        let buf = fs::read(raw_file).expect("read in");
    
        println!("    Decoding for raw pixel values");
        let processor = libraw::Processor::new();
        let raw_image = processor.decode(&buf).unwrap();
    
        Ok(ImageBuffer::from_libraw(&raw_image).unwrap())
    }

    pub fn get(&self, x:u32, y:u32) -> Result<f32, &str> {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            return Ok(self.buffer[index as usize]);
        } else {
            return Err(constants::INVALID_PIXEL_COORDINATES); // TODO: learn to throw exceptions
        }
    }

    pub fn put_u16(&mut self, x:u32, y:u32, val:u16) -> Result<&str, &str> {
        self.put(x, y, val as f32)
    }

    pub fn put(&mut self, x:u32, y:u32, val:f32) -> Result<&str, &str>{
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.buffer[index as usize] = val;
            return Ok(constants::OK);
        } else {
            return Err(constants::INVALID_PIXEL_COORDINATES);
        }
    }

    // Computes the mean of all pixel values
    pub fn mean(&self) -> f32 {

        let mut total:f32 = 0.0;

        // It is *soooo* inefficient to keep doing this...
        for y in 0..self.height {
            for x in 0..self.width {
                total = total + self.get(x, y).unwrap()
            }
        }

        return total / (self.width * self.height) as f32;
    }

    pub fn divide(&self, other:&ImageBuffer) -> Result<ImageBuffer, &str> {

        if self.width != other.width || self.height != other.height {
            return Err(constants::ARRAY_SIZE_MISMATCH);
        }

        let need_len = self.width * self.height;
        let mut v:Vec<f32> = Vec::with_capacity(need_len as usize);
        
        for i in 0..need_len {
            let quotient = self.buffer[i as usize] / other.buffer[i as usize];
            v.push(quotient);
        }

        ImageBuffer::from_vec(v, self.width, self.height)
    }

    pub fn scale(&self, scalar:f32) -> Result<ImageBuffer, &str> {
        let need_len = self.width * self.height;
        let mut v:Vec<f32> = Vec::with_capacity(need_len as usize);

        for i in 0..need_len {
            let product = self.buffer[i as usize] * scalar;
            v.push(product);
        }

        ImageBuffer::from_vec(v, self.width, self.height)
    }

    pub fn multiply(&self, other:&ImageBuffer) -> Result<ImageBuffer, &str> {

        if self.width != other.width || self.height != other.height {
            return Err(constants::ARRAY_SIZE_MISMATCH);
        }

        let need_len = self.width * self.height;
        let mut v:Vec<f32> = Vec::with_capacity(need_len as usize);
        
        for i in 0..need_len {
            let product = self.buffer[i as usize] * other.buffer[i as usize];
            v.push(product);
        }

        ImageBuffer::from_vec(v, self.width, self.height)
    }

    pub fn add(&self, other:&ImageBuffer) -> Result<ImageBuffer, &str> {

        if self.width != other.width || self.height != other.height {
            return Err(constants::ARRAY_SIZE_MISMATCH);
        }

        let need_len = self.width * self.height;
        let mut v:Vec<f32> = Vec::with_capacity(need_len as usize);
        
        for i in 0..need_len {
            let result = self.buffer[i as usize] + other.buffer[i as usize];
            v.push(result);
        }

        ImageBuffer::from_vec(v, self.width, self.height)
    }

    pub fn subtract(&self, other:&ImageBuffer) -> Result<ImageBuffer, &str> {

        if self.width != other.width || self.height != other.height {
            return Err(constants::ARRAY_SIZE_MISMATCH);
        }

        let need_len = self.width * self.height;
        let mut v:Vec<f32> = Vec::with_capacity(need_len as usize);
        
        for i in 0..need_len {
            let difference = self.buffer[i as usize] - other.buffer[i as usize];
            v.push(difference);
        }

        ImageBuffer::from_vec(v, self.width, self.height)
    }

    pub fn red(&self) -> Result<ImageBuffer, &str> {
        let dest_height = self.height / 2;
        let dest_width = self.width / 2;
    
        let mut dest = ImageBuffer::new(dest_width, dest_height).unwrap();
    
        for y in (0..self.height).step_by(2) {
            for x in (0..self.width).step_by(2) {
                let put_x = x / 2;
                let put_y = y / 2;
                
                let val_f32 :f32 = self.get(x, y).expect(constants::OK) as f32;
    
                dest.put(put_x, put_y, val_f32).expect(constants::OK);
            }
        }
    
        Ok(dest)
    }


    pub fn crop(&self, height:u32, width:u32) -> Result<ImageBuffer, &str> {

        let mut cropped_buffer = ImageBuffer::new(width, height).unwrap();

        for y in 0..height {
            for x in 0..width {

                let src_x = ((self.width - width) / 2) + x;
                let src_y = ((self.height - height) / 2) + y;

                cropped_buffer.put(x, y, self.get(src_x, src_y).unwrap()).unwrap();
            }
        }

        return Ok(cropped_buffer)
    }

    pub fn shift(&self, horiz:i32, vert:i32) -> Result<ImageBuffer, &str> {

        let mut shifted_buffer = ImageBuffer::new(self.width, self.height).unwrap();

        for y in 0..self.height {
            for x in 0..self.width {
                let shift_x = x as i32 + horiz;
                let shift_y = y as i32 + vert;
            
                if shift_x >= 0 && shift_y >= 0 && shift_x < self.width as i32 && shift_y < self.height as i32 {
                    shifted_buffer.put(shift_x as u32, shift_y as u32, self.get(x, y).unwrap()).unwrap();
                }
            }
        }
        return Ok(shifted_buffer)
    }

    pub fn calc_center_of_mass_offset(&self, threshold:f32) -> Result<Offset, &str> {
        let mut ox: f32 = 0.0;
        let mut oy: f32 = 0.0;
        let mut count: u32 = 0;
    
        for y in 0..self.height {
            for x in 0..self.width {
                let val = self.get(x, y).unwrap();
                if val >= threshold {
                    ox = ox + (x as f32);
                    oy = oy + (y as f32);
                    count = count + 1;
                }   
            }
        }
    
        if count > 0 {
            ox = (self.width as f32 / 2.0) - (ox / (count as f32));
            oy = (self.height as f32 / 2.0) - (oy / (count as f32));
        }
    
        Ok(Offset{h:ox.round() as i32, v:oy.round() as i32})
    }

    // Determined the minimum and maximum values within the 
    // red pixel channel.
    pub fn get_min_max(&self, override_dark:f32) -> Result<MinMax, &str> {
        
        let mut mx:f32 = 0.0;
        let mut mn:f32 = constants::_14_BIT_MAX as f32;

        for y in 0..self.height {
            for x in 0..self.width {
                let val = self.get(x, y).unwrap() as f32;
                mx = if val > mx { val } else { mx };
                mn = if val < mn { val } else { mn };
            }
        }
        if override_dark >= 0.0 {
            mn = override_dark;
        }
        
        Ok(MinMax{min:mn, max:mx})
    }

    pub fn save(&self, to_file:&str) -> Result<&str, &str> {
        let mut out_img = DynamicImage::new_rgb16(self.width, self.height).into_rgb16();
        
        for y in 0..self.height {
            for x in 0..self.width {
                let val = self.get(x, y).unwrap().round() as u16;
                out_img.put_pixel(x, y, Rgb([val, val, val]));
            }
        }

        println!("    Writing image buffer to file");
        if path::parent_exists_and_writable(&to_file) {
            out_img.save(to_file).unwrap();
            return Ok(constants::OK);
        } else {
            eprintln!("Parent does not exist or cannot be written: {}", path::get_parent(to_file));
            return Err(constants::PARENT_NOT_EXISTS_OR_UNWRITABLE);
        }
    
    }
}


