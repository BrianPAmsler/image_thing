extern crate png;

use std::fs::File;

fn as_mut_u8_slice(v: &mut [u32]) -> &mut [u8] {
    unsafe {
        std::slice::from_raw_parts_mut(
            v.as_ptr() as *mut u8,
            v.len() * std::mem::size_of::<i32>(),
        )
    }
}

fn as_u8_slice(v: &[u32]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            v.as_ptr() as *const u8,
            v.len() * std::mem::size_of::<i32>(),
        )
    }
}

#[derive(Clone)]
pub struct Image {
    w: u32,
    h: u32,
    pub(in crate::image) data: Vec<u32>
}

#[allow(dead_code)]
impl Image {
    pub fn create_empty_image(width: u32, height: u32) -> Image {
        let data = vec![0u32, width * height];

        Image { w: width, h: height, data }
    }

    pub fn create_image_from_file<P: AsRef<std::path::Path>>(filename: P) -> Image {
        let decoder = png::Decoder::new(File::open(filename).unwrap());
        let mut reader = decoder.read_info().unwrap();

        let ct = reader.output_color_type().0;

        // Allocate the output buffer.
        let mut buf_size = reader.output_buffer_size();

        if ct == png::ColorType::Rgb {
            buf_size /= 3;
        } else if ct == png::ColorType::Rgba {
            buf_size /= 4;
        } else {
            panic!("Invalid Color Format!");
        }
        
        let mut buf = vec![0u32; buf_size];

        let info: Option<png::OutputInfo>;

        if ct == png::ColorType::Rgba {
            // Read the next frame. An APNG might contain multiple frames.
            info = Some(reader.next_frame(as_mut_u8_slice(&mut buf)).unwrap());
        } else {
            // Convert to RGBA if it's in RGB
            let mut temp_buf = vec![0; reader.output_buffer_size()];
            info = Some(reader.next_frame(&mut temp_buf).unwrap());

            for i in 0..buf.len() {
                let r = temp_buf[i*3];
                let g = temp_buf[i*3 + 1];
                let b = temp_buf[i*3 + 2];

                let pixel = 0xFF000000 | ((b as u32) << 16) | ((g as u32) << 8) | (r as u32);

                buf[i] = pixel;
            }
        }
        
        match info {
            Some(x) => Image { w: x.width, h: x.height, data: buf },
            None => panic!("I don't know how this happened.")
        }
    }

    pub fn save_image_to_file<P: AsRef<std::path::Path>>(img: &Image, filename: P) {
        let f = std::fs::File::create(filename).unwrap();
        
        let ref mut w = std::io::BufWriter::new(f);

        let mut encoder = png::Encoder::new(w, img.w, img.h); // Width is 2 pixels and height is 1.
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        
        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(as_u8_slice(&img.data)).unwrap(); // Save
    }

    pub fn width(&self) -> u32 {
        self.w
    }

    pub fn height(&self) -> u32 {
        self.h
    }

    pub fn get_pixel_value(&self, x: u32, y: u32) -> (u8, u8, u8, u8) {
        let pixel: u32 = self.data[(x + y * self.w) as usize];

        let a: u8 = (pixel >> 24) as u8;
        let b: u8 = ((pixel >> 16) & 0xFF) as u8;
        let g: u8 = ((pixel >> 8) & 0xFF) as u8;
        let r: u8 = (pixel & 0xFF) as u8;

        (r, g, b, a)
    }

    pub fn set_pixel_value(&mut self, x: u32, y: u32, color: (u8, u8, u8, u8)) {
        let pixel: u32 = (color.3 as u32) << 24 | (color.2 as u32) << 16 | (color.1 as u32) << 8 | color.0 as u32;
        
        self.data[(x + y * self.w) as usize] = pixel;
    }

    pub fn get_raw_data(&self) -> &[u32] {
        &self.data
    }
    
    pub fn get_raw_data_mut(&mut self) -> &mut [u32] {
        &mut self.data
    }

    pub fn get_bytes(&self) -> &[u8] {
        as_u8_slice(&self.data[..])
    }
}