
use crate::path;
use crate::constants;
use crate::vprintln;

extern crate image;
use image::{open, DynamicImage, Rgb};
use std::fs;

// A simple image raster buffer.
#[derive(Debug, Clone)]
pub struct ImageBuffer {
    buffer: Vec<f32>,
    pub width: usize,
    pub height: usize,
    empty: bool,
}

pub struct Offset {
    pub h: i32,
    pub v: i32,
}

pub struct MinMax {
    pub min: f32,
    pub max: f32,
}

#[allow(dead_code)]
impl ImageBuffer {

    // Creates a new image buffer of the requested width and height
    pub fn new(width:usize, height:usize) -> Result<ImageBuffer, &'static str> {

        let mut v:Vec<f32> = Vec::with_capacity(width * height);
        v.resize(width * height, 0.0);

        Ok(ImageBuffer{buffer:v,
            width:width,
            height:height,
            empty:false
        })
    }

    pub fn new_empty() -> Result<ImageBuffer, &'static str> {
        Ok(ImageBuffer{buffer:Vec::new(),
            width:0,
            height:0,
            empty:true
        })
    }

    // Creates a new image buffer at the requested width, height and data
    pub fn from_vec(v:Vec<f32>, width:usize, height:usize) -> Result<ImageBuffer, &'static str> {

        if v.len() != (width * height) {
            return Err(constants::status::DIMENSIONS_DO_NOT_MATCH_VECTOR_LENGTH);
        }

        Ok(ImageBuffer{buffer:v,
                    width:width,
                    height:height,
                    empty:false
        })
    }

    pub fn from_file(file_path:&str) -> Result<ImageBuffer, &'static str> {

        if !path::file_exists(file_path) {
            return Err(constants::status::FILE_NOT_FOUND);
        }

        let image_data = open(file_path).unwrap().into_luma16();
        let dims = image_data.dimensions();

        let width = dims.0 as usize;
        let height = dims.1 as usize;
        vprintln!("    Input image dimensions: {:?}", image_data.dimensions());

        
        let mut v:Vec<f32> = Vec::with_capacity(width * height);
        v.resize(width * height, 0.0);

        for y in 0..height {
            for x in 0..width {
                let pixel = image_data.get_pixel(x as u32, y as u32);
                let value = pixel[0] as f32;
                let idx = y * width + x;
                v[idx] = value;
            }
        }

        ImageBuffer::from_vec(v, width, height)
    }

    pub fn from_libraw(raw_image:&libraw::RawImage) -> Result<ImageBuffer, &str> {
        let top_margin:u32  = raw_image.sizes().top_margin as u32;
        let left_margin:u32 = raw_image.sizes().left_margin as u32;
        let h:u32 = raw_image.sizes().raw_height as u32;
        let w:u32 = raw_image.sizes().raw_width as u32;

        vprintln!("    Raw pixel buffer height: {}", h);
        vprintln!("    Raw pixel buffer width: {}", w);
        vprintln!("    Raw pixel buffer top margin: {}", top_margin);
        vprintln!("    Raw pixel buffer left margin: {}", left_margin);

        let need_len = (h - top_margin) as usize * (w - left_margin) as usize;
        vprintln!("    Creating vector of capacity {}", need_len);

        let mut v:Vec<f32> = Vec::with_capacity(need_len);
        v.resize(need_len, 0.0);

        for y in 0..h {
            for x in 0..w {
                if y >= top_margin && x >= left_margin {
                    let idx = y * w + x;
                    let val = raw_image[idx as usize] as f32;
                    
                    let put_idx = ((y - top_margin) * (w - left_margin) + (x - left_margin)) as usize;
                    v[put_idx] = val;
                }
            }
        }

        ImageBuffer::from_vec(v, (w-left_margin) as usize, (h-top_margin) as usize)
    }  

    pub fn from_cr2(raw_file:&str) -> Result<ImageBuffer, &str> {
        vprintln!("    Reading raw image file {}", raw_file);

        if !path::file_exists(raw_file) {
            return Err("File does not exist");
        }
        let buf = fs::read(raw_file).expect("read in");
    
        vprintln!("    Decoding for raw pixel values");
        let processor = libraw::Processor::new();
        let raw_image = processor.decode(&buf).unwrap();
    
        Ok(ImageBuffer::from_libraw(&raw_image).unwrap())
    }

    pub fn get(&self, x:usize, y:usize) -> Result<f32, &str> {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            return Ok(self.buffer[index]);
        } else {
            return Err(constants::status::INVALID_PIXEL_COORDINATES); // TODO: learn to throw exceptions
        }
    }

    pub fn is_empty(&self) -> bool {
        self.empty
    }

    pub fn put_u16(&mut self, x:usize, y:usize, val:u16) -> Result<&str, &str> {
        self.put(x, y, val as f32)
    }

    pub fn put(&mut self, x:usize, y:usize, val:f32) -> Result<&str, &str>{
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.buffer[index] = val;
            return Ok(constants::status::OK);
        } else {
            return Err(constants::status::INVALID_PIXEL_COORDINATES);
        }
    }

    fn put_to_index_u16(&mut self, index:usize, val:u16) -> Result<&str, &str> {
        self.put_to_index(index, val as f32)
    }

    fn put_to_index(&mut self, index:usize, val:f32) -> Result<&str, &str> {
        if index >= (self.width * self.height) {
            return Err(constants::status::INVALID_PIXEL_COORDINATES);
        }

        self.buffer[index] = val;

        Ok(constants::status::OK)
    }

    // Computes the mean of all pixel values
    pub fn mean(&self) -> f32 {

        let mut total:f32 = 0.0;
        let mut count:f32 = 0.0;

        // It is *soooo* inefficient to keep doing this...
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel_value = self.get(x, y).unwrap();
                if pixel_value > 0.0 {
                    total = total + pixel_value;
                    count = count + 1.0;
                }
            }
        }

        return total / count;
    }

    pub fn divide(&self, other:&ImageBuffer) -> Result<ImageBuffer, &str> {

        if self.width != other.width || self.height != other.height {
            return Err(constants::status::ARRAY_SIZE_MISMATCH);
        }

        let mut dest = ImageBuffer::new(self.width, self.height).unwrap();

        let need_len = self.width * self.height;

        for i in 0..need_len {
            let quotient = if other.buffer[i] != 0.0 { self.buffer[i] / other.buffer[i] } else { 0.0 };
            dest.put_to_index(i, quotient).unwrap();
        }

        Ok(dest)
    }

    pub fn divide_into(&self, divisor:f32) -> Result<ImageBuffer, &str> {
        let need_len = self.width * self.height;
        let mut dest = ImageBuffer::new(self.width, self.height).unwrap();

        for i in 0..need_len {
            let quotient = if self.buffer[i] != 0.0 { divisor / self.buffer[i] } else { 0.0 };
            dest.put_to_index(i, quotient).unwrap();
        }

        Ok(dest)
    }

    pub fn scale(&self, scalar:f32) -> Result<ImageBuffer, &str> {
        let need_len = self.width * self.height;
        let mut dest = ImageBuffer::new(self.width, self.height).unwrap();

        for i in 0..need_len {
            let product = self.buffer[i] * scalar;
            dest.put_to_index(i, product).unwrap();
            //v[i] = product;
        }

        Ok(dest)
    }

    pub fn multiply(&self, other:&ImageBuffer) -> Result<ImageBuffer, &str> {

        if self.width != other.width || self.height != other.height {
            return Err(constants::status::ARRAY_SIZE_MISMATCH);
        }

        let need_len = self.width * self.height;
        let mut dest = ImageBuffer::new(self.width, self.height).unwrap();

        for i in 0..need_len {
            let product = self.buffer[i] * other.buffer[i];
            dest.put_to_index(i, product).unwrap();
            //v[i] = product;
        }

        Ok(dest)
    }

    pub fn add(&self, other:&ImageBuffer) -> Result<ImageBuffer, &str> {

        if self.width != other.width || self.height != other.height {
            return Err(constants::status::ARRAY_SIZE_MISMATCH);
        }

        let need_len = self.width * self.height;
        let mut dest = ImageBuffer::new(self.width, self.height).unwrap();

        for i in 0..need_len {
            let result = self.buffer[i] + other.buffer[i];
            dest.put_to_index(i, result).unwrap();
            //v[i] = result;
        }

        Ok(dest)
    }

    pub fn subtract(&self, other:&ImageBuffer) -> Result<ImageBuffer, &str> {

        if self.width != other.width || self.height != other.height {
            return Err(constants::status::ARRAY_SIZE_MISMATCH);
        }

        let mut dest = ImageBuffer::new(self.width, self.height).unwrap();
        let need_len = self.width * self.height;

        for i in 0..need_len {
            let mut difference = self.buffer[i] - other.buffer[i];
            if difference < 0.0 {
                difference = 0.0;
            }
            dest.put_to_index(i, difference).unwrap();
        }

        Ok(dest)
    }


    pub fn shift_to_min_zero(&self) -> Result<ImageBuffer, &str> {

        let minmax = self.get_min_max(-1.0).unwrap();

        let need_len = self.width * self.height;
        let mut dest = ImageBuffer::new(self.width, self.height).unwrap();

        for i in 0..need_len {
            let value = self.buffer[i];
            if minmax.min < 0.0 {
                dest.put_to_index(i, value + minmax.min).unwrap();
            } else {
                dest.put_to_index(i, value - minmax.min).unwrap();
            }
        }

        Ok(dest)
    }

    pub fn normalize(&self, min:f32, max:f32) -> Result<ImageBuffer, &str> {

        let shifted = self.shift_to_min_zero().unwrap();

        let need_len = self.width * self.height;
        let mut dest = ImageBuffer::new(self.width, self.height).unwrap();

        let minmax = shifted.get_min_max(-1.0).unwrap();
        
        for i in 0..need_len {
            let value = ((shifted.buffer[i] - minmax.min) / (minmax.max - minmax.min)) * (max - min) + min;
            dest.put_to_index(i, value).unwrap();
        }
        Ok(dest)
    }

    pub fn red(&self) -> Result<ImageBuffer, &str> {
        let dest_height = self.height / 2;
        let dest_width = self.width / 2;

        let mut dest = ImageBuffer::new(dest_width, dest_height).unwrap();

        for y in (0..self.height).step_by(2) {
            for x in (0..self.width).step_by(2) {
                let put_x = x / 2;
                let put_y = y / 2;
                let put_idx = (put_y * dest_width) + put_x;

                let val_f32 :f32 = self.get(x, y).expect(constants::status::OK) as f32;
                dest.put_to_index(put_idx, val_f32).unwrap();
            }
        }
        Ok(dest)
    }


    pub fn crop(&self, height:usize, width:usize) -> Result<ImageBuffer, &str> {

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

        let h = self.height as i32;
        let w = self.width as i32;

        for y in 0..h {
            for x in 0..w {
                let shift_x = x as i32 + horiz;
                let shift_y = y as i32 + vert;
            
                if shift_x >= 0 && shift_y >= 0 && shift_x < w  && shift_y < h {
                    shifted_buffer.put(shift_x as usize, shift_y as usize, self.get(x as usize, y as usize).unwrap()).unwrap();
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
        
        let mut mx:f32 = std::f32::MIN;
        let mut mn:f32 = std::f32::MAX;

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
        let mut out_img = DynamicImage::new_rgb16(self.width as u32, self.height as u32).into_rgb16();
        
        for y in 0..self.height {
            for x in 0..self.width {
                let val = self.get(x, y).unwrap().round() as u16;
                out_img.put_pixel(x as u32, y as u32, Rgb([val, val, val]));
            }
        }

        vprintln!("    Writing image buffer to file at {}", to_file);
        if path::parent_exists_and_writable(&to_file) {
            out_img.save(to_file).unwrap();
            vprintln!("    File saved.");
            return Ok(constants::status::OK);
        } else {
            eprintln!("Parent does not exist or cannot be written: {}", path::get_parent(to_file));
            return Err(constants::status::PARENT_NOT_EXISTS_OR_UNWRITABLE);
        }
    }
}


