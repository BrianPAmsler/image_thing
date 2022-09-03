mod image;

use image::Image;

fn main() {
    let mut img1 = Image::create_image_from_file("C:\\Users\\Brian\\Desktop\\test.png");
    let mut img2 = Image::create_image_from_file("C:\\Users\\Brian\\Desktop\\test2.png");

    println!("Resolution: ({}, {})", img1.width(), img1.height());

    let pixel1 = img1.get_pixel_value(908, 193);
    let pixel2 = img2.get_pixel_value(908, 193);

    println!("Pixel1 at 908, 193: ({}, {}, {}, {})", pixel1.0, pixel1.1, pixel1.2, pixel1.3);
    println!("Pixel2 at 908, 193: ({}, {}, {}, {})", pixel2.0, pixel2.1, pixel2.2, pixel2.3);

    for x in 0..img1.width() {
        for y in 0..img1.height() {
            let color = img1.get_pixel_value(x, y);

            let mut r = color.0 as u32;
            let mut g = color.1 as u32;
            let mut b = color.2 as u32;

            r = (r as f32 * 1.2f32) as u32;
            g = (g as f32 * 0.8f32) as u32;
            b = (b as f32 * 0.8f32) as u32;

            if r > 255 {
                r = 255;
            }

            img1.set_pixel_value(x, y, (r as u8, g as u8, b as u8, color.3));
        }
    }
    
    for x in 0..img2.width() {
        for y in 0..img2.height() {
            let color = img2.get_pixel_value(x, y);

            let mut r = color.0 as u32;
            let mut g = color.1 as u32;
            let mut b = color.2 as u32;

            r = (r as f32 * 1.2f32) as u32;
            g = (g as f32 * 0.8f32) as u32;
            b = (b as f32 * 0.8f32) as u32;

            if r > 255 {
                r = 255;
            }

            img2.set_pixel_value(x, y, (r as u8, g as u8, b as u8, color.3));
        }
    }

    Image::save_image_to_file(&img2, "C:\\Users\\Brian\\Desktop\\out2.png")
}
