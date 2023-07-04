use std::{rc::Rc};

use crate::{vec::{Poin3, Ray, Vect3}, material::{Material}};

pub struct HitRecord {
    pub p: Poin3,
    pub normal: Vect3,
    pub mat: Rc<dyn Material>,
    pub t: f32,
    pub front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self { p: Default::default(), normal: Default::default(), mat: <dyn Material>::default(), t: Default::default(), front_face: Default::default() }
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vect3) {
        self.front_face = r.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -(*outward_normal)
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}
