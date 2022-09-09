mod image;
mod image_secret;

use std::{io::{Read, Write}, convert::TryInto, path::{Path, PathBuf}, fs::File};

use image::Image;

fn main() {
    let mut img = Image::create_image_from_file("C:\\Users\\Brian\\Desktop\\hidden_file.png");

    //program::encode_file(&mut img, "C:\\Users\\Brian\\Pictures\\test.jpg");

    //Image::save_image_to_file(&mut img, "C:\\Users\\Brian\\Desktop\\hidden_file.png");
    
    let info = image_secret::get_secret_info(&mut img);
    let mut data = vec![0u8; info.secret_size];

    println!("info: {:?}", info);

    let mut path = PathBuf::new();
    path.push("C:\\Users\\Brian\\Desktop");

    let filename = image_secret::decode_file(&mut img, &mut data, info.num_bits);
    path.push(&filename);

    println!("filename: {}, Path: {:?}", filename, path);

    let mut file = File::create(path).unwrap();
    file.write(&mut data).unwrap();
    
}
