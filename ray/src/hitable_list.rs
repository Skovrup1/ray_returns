use std::rc::Rc;

use crate::hitable::*;
use crate::ray::*;

pub struct HitableList {
    objects: Vec<Rc<dyn Hitable>>,
}

impl HitableList {
    pub fn new() -> HitableList {
        HitableList {
            objects: Vec::new(),
        }
    }
    pub fn new_with_value(object: Rc<dyn Hitable>) -> HitableList {
        let mut list = HitableList::new();
        list.add(object);
        list
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hitable>) {
        self.objects.push(object);
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if object.hit(r, t_min, closest_so_far, rec) {
                closest_so_far = rec.t;
                hit_anything = true;
            }
        }

        hit_anything
    }
}
