mod image;
mod image_secret;

extern crate const_format;

use std::{env, io::{Write}, path::{Path, PathBuf}, fs::File, collections::HashMap};
use const_format::formatcp;

use image::Image;

const EXE_NAME: &str = "idk";

fn get_args(args: &[String]) -> HashMap<String, String> {
    let mut map = HashMap::new();

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        let mut arg_string = String::from("");

        if arg.starts_with('-') && arg.len() > 1 {
            if args.len() > i + 1 {
                let temp = &args[i + 1];

                if !arg_string.starts_with('-') {
                    arg_string = temp.to_owned();

                    i += 1;
                }
            }
            
            map.insert(arg[1..].to_string(), arg_string);
        }

        i += 1;
    }

    map
}

fn encode(input_file: &str, output_file: &str, args: HashMap<String, String>) -> Result<(), &'static str>{
    if args.contains_key("s") && args.contains_key("f") {
        return Err("Must provide a file or a string, not both!");
    }

    if args.contains_key("s") {
        let encode_string = args["s"].to_owned();

        let mut img = Image::create_image_from_file(input_file);
        image_secret::encode_string(&mut img, &encode_string);

        Image::save_image_to_file(&mut img, output_file);
    } else if args.contains_key("f") {
        let filename = args["f"].to_owned();

        let mut img = Image::create_image_from_file(input_file);
        image_secret::encode_file(&mut img, filename);

        Image::save_image_to_file(&mut img, output_file);
    }

    Ok(())
}

fn decode(input_file: &str, args: HashMap<String, String>) -> Result<(), &'static str> {
    let mut img = Image::create_image_from_file(input_file);

    let info = image_secret::get_secret_info(&mut img);

    match info.secret_type {
        image_secret::SecretType::String => {
            let secret = image_secret::decode_string(&mut img, info.secret_size, info.num_bits);

            println!("Image contains the string: {}", &secret);

            if args.contains_key("o") {
                let filename = &args["o"];
                let mut f = File::create(filename).unwrap();

                f.write(secret.as_bytes()).unwrap();
            }

            Ok(())
        },
        image_secret::SecretType::File => {
            let mut secret = vec![0u8; info.secret_size];
            let filename = image_secret::decode_file(&mut img, &mut secret, info.num_bits);

            let mut output_file: PathBuf;
            if args.contains_key("o") {
                output_file = PathBuf::new();
                output_file.push(args["o"].as_str());
            } else {
                let p = Path::new(input_file).parent().unwrap();

                output_file = p.join(filename);
            }

            println!("Decoded file saved to {}", output_file.to_str().unwrap());

            let mut f = File::create(output_file).unwrap();

            f.write(&secret).unwrap();

            Ok(())
        },
        image_secret::SecretType::NoSecret => {
            Err("Image does not contain a secret!")
        }
    }
}

pub fn capacity(input_file: &str) -> Result<(), &'static str> {
    let mut img = Image::create_image_from_file(input_file);

    println!("Capacity for num_bits:");
    for i in 1..5 {
        let cap = image_secret::get_img_capacity(&mut img, i);

        println!("\t{}: {}mb", i, cap / 1000000);
    }

    Ok(())
}

fn main() {
    let raw_args: Vec<String> = env::args().collect();

    if raw_args.len() < 3 {
        println!("Usage: {} [encode, decode] <args>", EXE_NAME);
        return;
    }

    let command = raw_args[1].to_owned();

    let result = match &command[..] {
        "encode" => encode(&raw_args[2], &raw_args[3], get_args(&raw_args[4..])),
        "decode" => decode(&raw_args[2], get_args(&raw_args[3..])),
        "capacity" => capacity(&raw_args[2]),
        _ => Err(formatcp!("Usagee: {} [encode, decode] <args>", EXE_NAME))
    };

    if result.is_err() {
        println!("{}", result.err().unwrap());
    }
}
