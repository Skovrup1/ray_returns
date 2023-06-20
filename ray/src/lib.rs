use image::{self, RgbImage, ImageFormat, Rgb, ImageError};
use std::path::Path;

mod vec;

pub fn save_img(img: RgbImage) -> () {
    let mut res: Result<(), ImageError>;

    let mut path_i = 0;    
    loop {
        let path_string = format!("img_{}.png", path_i);
        let path = Path::new(&path_string);
        path_i += 1;
        
        if path.exists() {
            res = img.save_with_format(path, ImageFormat::Png);
            break;
        }
    } 

    match res {
        Err(err) => { 
            match err {
                _ => panic!()
            }
        }
        _ => ()
    }
}

pub fn save_img_override(img: RgbImage) -> () {
    let path = Path::new("override_img.png");
    let res = img.save_with_format(path, ImageFormat::Png);

    match res {
        Err(err) => { 
            match err {
                _ => panic!()
            }
        }
        _ => ()
    }
}

pub fn create_img() -> RgbImage {
    let mut img = RgbImage::new(32, 32);

    for (_x, _y, pix) in img.enumerate_pixels_mut() {
        let red_pix = Rgb([255, 255, 0]); 

        *pix = red_pix;
    }

    return img;
}