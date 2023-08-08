use crate::aabb::AABB;
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

    fn bounding_box(&self) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let mut is_first = true;
        let mut output_box = AABB::default();

        for (i, obj) in self.objects.iter().enumerate() {
            if let Some(bounding) = obj.bounding_box() {
                if is_first {
                    output_box = bounding;
                } else {
                    output_box = bounding.surrounding_box(output_box);
                }
            } else {
                return None;
            }

            is_first = false;
        }
        None
    }
}
