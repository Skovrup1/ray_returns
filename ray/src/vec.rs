#![allow(dead_code)]

use nalgebra::{Vector3, Point3};
use rand::{thread_rng, prelude::Distribution, Rng};

pub type Color = Vector3<f32>;

pub trait RandomExt {
    fn random_between(min: f32, max: f32) -> Self;
    fn random() -> Self;
}

impl RandomExt for Vector3<f32> {
    fn random_between(min: f32, max: f32) -> Self {
        let mut rng = thread_rng();
        let between = rand::distributions::Uniform::from(min..max);

        Vector3::new(between.sample(&mut rng), between.sample(&mut rng), between.sample(&mut rng))
    }

    fn random() -> Self {
        let mut rng = thread_rng();

        Vector3::new(rng.gen(), rng.gen(), rng.gen())
    }
}

pub fn random_in_unit_sphere() {
    todo!()
}

pub fn random_unit_vector() {
    todo!()
}