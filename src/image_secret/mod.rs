mod secret_rw;
use std::{str::from_utf8, io::{Read, Write, BufReader}, convert::{TryFrom, TryInto}, fs::File};

use crate::image::Image;
use secret_rw::SecretWriter;

use std::path::Path;

const HEADER_STRING: &str = "Morkmessage Hider";
const HEADER_LENGTH: usize = HEADER_STRING.len() + 9;

#[derive(Clone, Copy, Debug)]
pub enum SecretType {
    String,
    File,
    NoSecret
}

impl From<u8> for SecretType {
    fn from(a: u8) -> SecretType {
        match a {
            0 => SecretType::String,
            1 => SecretType::File,
            _ => SecretType::NoSecret
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SecretInfo {
    pub num_bits: usize,
    pub secret_type: SecretType,
    pub secret_size: usize
}

pub fn get_secret_info(img: &mut Image) -> SecretInfo {
    let mut coder = SecretWriter::create(img);

    let mut header_slice = [0u8; HEADER_LENGTH];
    let mut header_found: bool;
    let mut num_bits = 0;
    while {
        num_bits += 1;

        coder.set_num_bits(num_bits);
        coder.read(&mut header_slice).unwrap();
        
        let test_string = from_utf8(&header_slice[..HEADER_STRING.len()]);
        header_found = test_string.is_ok() && test_string.unwrap() == HEADER_STRING;

        // Apparently this works like a do-while loop
        num_bits < 4 && !header_found
    } {}

    if header_found {
        let secret_type = SecretType::try_from(header_slice[HEADER_STRING.len()]).unwrap();
        let secret_size = usize::from_be_bytes(header_slice[HEADER_STRING.len() + 1..].try_into().unwrap());

        return SecretInfo { num_bits, secret_type, secret_size };
    }

    SecretInfo { num_bits: 0, secret_type: SecretType::NoSecret, secret_size: 0 }
}

fn generate_header(secret_type: SecretType, bytes: usize) -> [u8; HEADER_LENGTH] {
    let mut header = [0u8; HEADER_LENGTH];

    header[..HEADER_STRING.len()].copy_from_slice(HEADER_STRING.as_bytes());
    header[HEADER_STRING.len()] = secret_type as u8;

    let size = (bytes as u64).to_be_bytes();

    header[HEADER_STRING.len() + 1..].copy_from_slice(&size);

    header
}

fn encode_data(img: &mut Image, data: &[u8]) {
    let mut coder = SecretWriter::create(img);

    let mut num_bits = 1;
    while num_bits <= 4 && coder.get_capacity() < data.len() {
        num_bits += 1;
        coder.set_num_bits(num_bits);
    }

    if num_bits == 5 {
        println!("capacity: {}, data size: {}", coder.get_capacity(), data.len());
        panic!("Not enough image capacity!");
    }

    coder.write(data).unwrap();
}

pub fn encode_string(img: &mut Image, str: &str) {
    let header = generate_header(SecretType::String, str.len());

    let mut data = vec![0u8; header.len() + str.len()];
    data[..header.len()].copy_from_slice(&header);
    data[header.len()..].copy_from_slice(str.as_bytes());

    encode_data(img, &data);
}

pub fn encode_file<P: AsRef<Path>>(img: &mut Image, filename: P) {
    let file = filename.as_ref();

    if !file.is_file() {
        panic!("Must be a file.");
    } else if !file.exists() {
        panic!("File does not exist!");
    }

    let filename = file.file_name().unwrap().to_str().unwrap();
    let f = File::open(file).unwrap();

    let mut buf_reader = BufReader::new(f);
    let mut bytes = vec![0u8;0];

    buf_reader.read_to_end(&mut bytes).unwrap();
    
    
    let header = generate_header(SecretType::File, bytes.len());

    // This is definitely not the best way to do this, but I'm lazy
    bytes.splice(0..0, filename.as_bytes().iter().cloned());
    bytes.splice(0..0, (filename.len() as u64).to_be_bytes());
    bytes.splice(0..0, header);

    encode_data(img, &mut bytes);
}

pub fn decode_string(img: &mut Image, len: usize, num_bits: usize) -> String {
    let mut data = vec![0u8; HEADER_LENGTH + len];

    let mut coder = SecretWriter::create(img);

    coder.set_num_bits(num_bits);
    coder.read(&mut data[..]).unwrap();

    std::str::from_utf8(&data[HEADER_LENGTH..]).unwrap().to_string()
}

pub fn decode_file(img: &mut Image, buf: &mut [u8], num_bits: usize) -> String {
    let mut data = vec![0u8; HEADER_LENGTH + 8];

    let mut coder = SecretWriter::create(img);
    coder.set_num_bits(num_bits);
    coder.read(&mut data[..]).unwrap();

    let f_len = u64::from_be_bytes(data[HEADER_LENGTH..HEADER_LENGTH + 8].try_into().unwrap()) as usize;

    data.extend(vec![0u8; buf.len() + f_len]);
    coder.read(&mut data[..]).unwrap();

    let filename = std::str::from_utf8(&data[HEADER_LENGTH + 8..HEADER_LENGTH + 8 + f_len]).unwrap().to_string();

    buf.copy_from_slice(&data[HEADER_LENGTH + 8 + f_len..]);

    filename
}

pub fn get_img_capacity(img: &mut Image, num_bits: usize) -> usize {
    let mut coder = SecretWriter::create(img);
    coder.set_num_bits(num_bits);

    coder.get_capacity()
}