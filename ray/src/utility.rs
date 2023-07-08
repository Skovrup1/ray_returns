use rand::distributions::{Distribution, Uniform};

pub const IMAGE_WIDTH: u32 = 1200 / 5;
pub const ASPECT_RATIO: f32 = 3.0 / 2.0;
pub const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
pub const SAMPLES: u32 = 500 / 5;
pub const MAX_DEPTH: u32 = 50;

pub fn random_float() -> f32 {
    let between = Uniform::from(0.0..1.0);
    between.sample(&mut rand::thread_rng())
}

pub fn between_float(min: f32, max: f32) -> f32 {
    let between = Uniform::from(min..max);
    between.sample(&mut rand::thread_rng())
}
