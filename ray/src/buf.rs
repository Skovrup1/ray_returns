use crate::{
    utility::{IMAGE_HEIGHT, IMAGE_WIDTH},
    vec::Color,
};

pub struct Buf {
    pub data: Vec<u8>,
    width: u32,
    height: u32,
}

impl Buf {
    pub fn default() -> Self {
        let data = vec![0; (IMAGE_WIDTH * IMAGE_HEIGHT * 3) as usize];
        let width = IMAGE_WIDTH;
        let height = IMAGE_HEIGHT;
        Buf {
            data,
            width,
            height,
        }
    }

    pub fn write_color(&mut self, x: u32, y: u32, color: Color, samples: u32) -> () {
        let scale = (samples as f32).recip();
        //adjust for gamma = 2
        let r = (color.x * scale).sqrt();
        let g = (color.y * scale).sqrt();
        let b = (color.z * scale).sqrt();

        let index = ((x + y * self.width) * 3) as usize;
        self.data[index] = (256.0 * r.clamp(0.0, 0.999)) as u8;
        self.data[index + 1] = (256.0 * g.clamp(0.0, 0.999)) as u8;
        self.data[index + 2] = (256.0 * b.clamp(0.0, 0.999)) as u8;
    }
}
