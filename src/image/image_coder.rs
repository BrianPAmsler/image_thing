use crate::image::Image;

use std::io::{Read, Write, Error};

use super::as_u8_slice;

pub struct ImageCoder<'a> {
    img: &'a mut Image,
    num_bits: usize,
    use_alpha_channel: bool
}

#[allow(dead_code)]
impl ImageCoder<'_> {
    pub fn create(img: &mut Image) -> ImageCoder {
        ImageCoder { img, num_bits: 1, use_alpha_channel: false }
    }

    pub fn num_bits(&mut self, num_bits: usize) {
        if num_bits > 4 {
            panic!("num_bits cannot be > 4!");
        }

        self.num_bits = num_bits;
    }

    pub fn use_alpha_channel(&mut self, use_alpha_channel: bool) {
        self.use_alpha_channel = use_alpha_channel;
    }

    pub fn get_capacity(&self) -> u32 {
        let total_bits: usize;

        if self.use_alpha_channel {
            total_bits = self.img.data.len() * self.num_bits * 4;
        } else {
            total_bits = self.img.data.len() * self.num_bits * 3;
        }

        (total_bits as u32) / 8
    }
}

struct BitQueue {
    i: usize,
    bytes: Vec<u8>
}

impl BitQueue {
    pub fn new() -> BitQueue {
        BitQueue { i: 0, bytes: Vec::new() }
    }

    pub fn push_bits(&mut self, bits: u64, count: usize) {
        let mut bits_left = count;
        while bits_left > 0 {
            // add 8 bits at a time
            let bits_to_add = std::cmp::min(8, bits_left);

            let mut current_bits = (((bits >> (bits_left - bits_to_add)) & !(0xFF << bits_to_add)) as u8) << (8 - bits_to_add);

            let mut bits_not_added = bits_to_add;
            while bits_not_added > 0 {
                let current_byte_index = self.i / 8;
                let bit_in_byte = self.i % 8;

                // Add a new byte if necessary
                if self.bytes.len() == current_byte_index {
                    self.bytes.push(0);
                }

                let bits_added = std::cmp::min(8 - bit_in_byte, bits_not_added);

                self.bytes[current_byte_index] = self.bytes[current_byte_index] | current_bits >> bit_in_byte;

                current_bits = (((current_bits as u32) << bits_added) & 0xFF) as u8;
                bits_not_added -= bits_added;

                self.i += bits_added;
            }

            bits_left -= bits_to_add;
        }
    }

    pub fn push_byte(&mut self, byte: u8) {
        self.push_bits(byte as u64, 8);
    }

    pub fn pop_bits(&mut self, num_bits: usize) -> u8 {
        if num_bits > 8 {
            panic!("Can't pop more than 8 bits at a time!");
        }

        if num_bits > self.i {
            panic!("Not enough bits to pop!");
        }

        let mut overflow: u8 = 0;
        let mut i = (self.i as i32 - 1) / 8;
        while i >= 0 {
            let byte_index = i as usize;
            let mut shift = self.bytes[byte_index] as u16;
            shift = shift << num_bits;

            self.bytes[byte_index] = (overflow as u16 | shift) as u8;
            overflow = (shift >> 8) as u8;

            i -= 1;
        }

        self.i -= num_bits;

        overflow
    }

    pub fn get_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn total_bits(&self) -> usize {
        self.i
    }

    pub fn total_filled_bytes(&self) -> usize {
        self.i / 8
    }
}

impl Read for ImageCoder<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let bytes_per_pixel;

        if self.use_alpha_channel {
            bytes_per_pixel = 4;
        } else {
            bytes_per_pixel = 3;
        }

        let data_size = std::cmp::min(buf.len(), (self.img.data.len() * bytes_per_pixel * self.num_bits) / 8);

        let mut q = BitQueue::new();
        
        let mut i = 0;
        while q.total_filled_bytes() < data_size && i < self.img.data.len() {
            let bitmask = (!(0xFFu8 << self.num_bits)) as u64;

            let pixel = self.img.data[i];

            let a = (pixel >> 24) as u64;
            let b = ((pixel >> 16) & 0xFF) as u64;
            let g = ((pixel >> 8) & 0xFF) as u64;
            let r = (pixel & 0xFF) as u64;

            q.push_bits(r & bitmask, self.num_bits);
            q.push_bits(g & bitmask, self.num_bits);
            q.push_bits(b & bitmask, self.num_bits);
            
            if self.use_alpha_channel {
                q.push_bits(a & bitmask, self.num_bits);
            }

            i += 1;
        }

        for (d, s) in buf.iter_mut().zip(q.get_bytes().iter()) {
            *d = *s;
        }

        Ok(data_size)
    }
}

impl Write for ImageCoder<'_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let num_bytes = (buf.len() * 8 + self.num_bits - 1) / self.num_bits;

        if num_bytes > self.get_capacity() as usize {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Not enough Image capacity!"));
        }

        let bytes_per_pixel;
        if self.use_alpha_channel {
            bytes_per_pixel = 4;
        } else {
            bytes_per_pixel = 3;
        }

        let mut bits = BitQueue::new();
        let mut i = 0;
        for byte in buf {
            bits.push_byte(*byte);

            while bits.total_bits() >= self.num_bits {
                let data = bits.pop_bits(self.num_bits);

                let pixel = i / bytes_per_pixel;
                let subpixel = i % bytes_per_pixel;

                let pixel_data = self.img.data[pixel];
                let mut a = (pixel_data >> 24) as u8;
                let mut b = ((pixel_data >> 16) & 0xFF) as u8;
                let mut g = ((pixel_data >> 8) & 0xFF) as u8;
                let mut r = (pixel_data & 0xFF) as u8;

                let bitmask = 0xFFu8 << self.num_bits;
                match subpixel {
                    0 => { r = (r & bitmask) | data; }
                    1 => { g = (g & bitmask) | data; }
                    2 => { b = (b & bitmask) | data; }
                    3 => { a = (a & bitmask) | data; }
                    _ => { panic!("How did this happen? I don't know..."); }
                }
                
                self.img.data[pixel] = ((a as u32) << 24) | ((b as u32) << 16) | ((g as u32) << 8) | (r as u32);

                i += 1;
            }
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}