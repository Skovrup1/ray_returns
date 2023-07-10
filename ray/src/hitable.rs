use nalgebra::Vector3;

use crate::material::*;
use crate::ray::*;

#[derive(Default)]
pub struct HitRecord {
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub mat: Material,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vector3<f32>) {
        self.front_face = Vector3::dot(&r.dir(), &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            outward_normal * -1.0
        };
    }
}

pub trait Hitable {
    fn hit(&self, _r: &Ray, _t_min: f32, _t_max: f32) -> Option<HitRecord> {
        None
    }
}
