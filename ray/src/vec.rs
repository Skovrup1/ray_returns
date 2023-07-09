#![allow(dead_code)]

use nalgebra::Vector3;
use rand::{prelude::Distribution, thread_rng, Rng};

pub type Color = Vector3<f32>;

pub trait RefractExt {
    fn refract(&self, n: Vector3<f32>, etai_etat: f32) -> Vector3<f32>;
}

impl RefractExt for Vector3<f32> {
    fn refract(&self, n: Vector3<f32>, etai_etat: f32) -> Vector3<f32> {
        let cos_theta = (-self).dot(&n).min(1.0);
        let out_perp = etai_etat * (self + cos_theta * n);
        let out_parallel = -((1.0 - out_perp.norm_squared()).abs().sqrt()) * n;

        out_perp + out_parallel
    }
}

pub trait ReflectExt {
    fn reflect(&self, n: Vector3<f32>) -> Vector3<f32>;
}

impl ReflectExt for Vector3<f32> {
    fn reflect(&self, n: Vector3<f32>) -> Vector3<f32> {
        self - n * self.dot(&n) * 2.0
    }
}
pub trait RandomExt {
    fn random_between(min: f32, max: f32) -> Self;
    fn random() -> Self;
}

impl RandomExt for Vector3<f32> {
    fn random_between(min: f32, max: f32) -> Self {
        let mut rng = thread_rng();
        let between = rand::distributions::Uniform::from(min..max);

        Vector3::new(
            between.sample(&mut rng),
            between.sample(&mut rng),
            between.sample(&mut rng),
        )
    }

    fn random() -> Self {
        let mut rng = thread_rng();

        Vector3::new(rng.gen(), rng.gen(), rng.gen())
    }
}

pub fn random_in_hemisphere(n: Vector3<f32>) -> Vector3<f32> {
    let unit_sphere = random_in_unit_sphere();

    if unit_sphere.dot(&n) > 0.0 {
        return unit_sphere;
    } else {
        return -unit_sphere;
    }
}

pub fn random_unit_vector() -> Vector3<f32> {
    random_in_unit_sphere().normalize()
}

pub fn random_in_unit_sphere() -> Vector3<f32> {
    loop {
        let p = Vector3::random_between(-1.0, 1.0);

        if p.norm_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_in_unit_disk() -> Vector3<f32> {
    loop {
        let mut rng = thread_rng();
        let mut between = rand::distributions::Uniform::from(-1.0..1.0).sample_iter(&mut rng);

        let p = Vector3::new(between.next().unwrap(), between.next().unwrap(), 0.0);

        if p.norm_squared() < 1.0 {
            return p;
        }
    }
}
