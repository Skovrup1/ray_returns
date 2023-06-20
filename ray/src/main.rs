use ray::{create_img, save_img};

fn main() {
    let img = create_img(); 

    save_img(img);

    println!("Hello, world!");
}
