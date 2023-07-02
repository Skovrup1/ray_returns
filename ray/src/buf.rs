use crate::{
    ray_const::{HEIGHT, WIDTH},
    vec::Vect3,
};

pub struct Buf {
    pub data: Vec<u8>,
    width: u32,
    height: u32,
}

impl Buf {
    pub fn default() -> Self {
        let data = vec![0; (WIDTH * HEIGHT * 3) as usize];
        let width = WIDTH;
        let height = HEIGHT;
        Buf {
            data,
            width,
            height,
        }
    }

    pub fn change_pix(&mut self, x: u32, y: u32, color: Vect3) -> () {
        let index = ((x + y * self.width) * 3) as usize;
        self.data[index] = (color.x * 255.999) as u8;
        self.data[index + 1] = (color.y * 255.999) as u8;
        self.data[index + 2] = (color.z * 255.999) as u8;
    }
}
