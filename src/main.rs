mod image;

use std::{io::{Read, Write}, convert::TryInto};

use image::{Image, image_coder::ImageCoder};
    
static HEADER_STRING: &str = "Morkmessage Hider v1.0";

fn check_for_header(coder: &mut ImageCoder) -> u64 {
    let header_len = HEADER_STRING.len();
    let mut header = vec![0u8; header_len + 8];

    if coder.read(&mut header).is_err() {
        return 0;
    }

    let h_string = std::str::from_utf8(&header[..header_len]).unwrap();

    if h_string == HEADER_STRING {
        return u64::from_be_bytes(header[header_len..].try_into().unwrap());
    }

    0
}

fn write_with_header(coder: &mut ImageCoder, bytes: &mut [u8]) {
    let header_len = HEADER_STRING.len();
    let mut header = vec![0u8; header_len + 8];

}

fn main() {
    let emoji = std::fs::read("C:\\Users\\Brian\\Pictures\\lcemoji.webp").unwrap();

    let mut img = Image::create_image_from_file("C:\\Users\\Brian\\Desktop\\test.png");

    {
        let mut coder = ImageCoder::create(&mut img);
        coder.num_bits(4);

        println!("capacity: {}mb", (coder.get_capacity() as f64) / 1048576f64);
        if coder.write(&emoji[..]).is_err() {
            panic!("Write failed!");
        }
    }

    Image::save_image_to_file(&img, "C:\\Users\\Brian\\Desktop\\hidden_emoji.png");
    
    let mut img2 = Image::create_image_from_file("C:\\Users\\Brian\\Desktop\\hidden_emoji.png");

    let mut coder = ImageCoder::create(&mut img2);
    let mut bytes = vec![0u8; emoji.len()];
    if coder.read(&mut bytes).is_err() {
        panic!("Read failed!");
    }

    std::fs::write("C:\\Users\\Brian\\Desktop\\extracted_emoji.png", bytes).unwrap();
}
