use image::{self, ImageError, ImageFormat};
use std::path::Path;

use crate::ray_const::{HEIGHT, WIDTH};

pub fn save_img(img: &[u8]) -> () {
    let res: Result<(), ImageError>;

    let mut path_i = 0;
    loop {
        let path_string = format!("img_{}.png", path_i);
        let path = Path::new(&path_string);
        path_i += 1;

        if !path.exists() {
            res = image::save_buffer_with_format(
                path,
                img,
                WIDTH,
                HEIGHT,
                image::ColorType::Rgb8,
                ImageFormat::Png,
            );
            break;
        }
    }

    match res {
        Err(err) => match err {
            _ => panic!("Error saving image"),
        },
        _ => (),
    }
}

pub fn save_img_override(img: &[u8]) -> () {
    let path = Path::new("override_img.png");
    let res = image::save_buffer_with_format(
        path,
        img,
        WIDTH,
        HEIGHT,
        image::ColorType::Rgb8,
        ImageFormat::Png,
    );

    match res {
        Err(err) => match err {
            _ => panic!("Error saving image"),
        },
        _ => (),
    }
}
