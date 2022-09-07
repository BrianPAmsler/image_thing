mod image;

use std::io::{Read, Write};

use image::{Image, image_coder::ImageCoder};

fn print_bytes(bytes: &[u8]) {
    print!("[ ");
    for b in bytes {
        print!("{:#010b}, ", b);
    }
    println!("]");
}
    
static TEST_STRING: &str = "Hello World!";

fn main() {
    let mut img = Image::create_image_from_file("C:\\Users\\Brian\\Desktop\\test.png");

    let mut coder = ImageCoder::create(&mut img);
    let mut data = vec![0u8; TEST_STRING.len()];

    coder.read(&mut data[..]);
    print!("Original Data: \t");
    print_bytes(&data[..]);

    coder.write(TEST_STRING.as_bytes());
    
    print!("String Data: \t");
    print_bytes(TEST_STRING.as_bytes());

    coder.read(&mut data[..]);
    print!("Read Data: \t");
    print_bytes(&data[..]);

    println!("String read from image: {}", std::str::from_utf8(&data[..]).unwrap());        
}
