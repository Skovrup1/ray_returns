use crate::hitable::*;
use crate::ray::*;
use crate::sphere::Sphere;

pub struct HitableList {
    pub objects: Vec<Sphere>,
}

impl HitableList {
    pub fn new() -> HitableList {
        HitableList {
            objects: Vec::new(),
        }
    }
}

impl HitableList {
    pub fn add(&mut self, obj: Sphere) {
        self.objects.push(obj);
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_anything = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(hit) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }

        hit_anything
    }
}
