use std::{mem, rc::Rc};

use crate::hittable::{HitRecord, Hittable};

#[derive(Default)]
pub struct HittableList {
    //consider a refactor
    objs: Vec<Rc<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::vec::Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = Default::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for i in &self.objs {
            if i.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                mem::swap(rec, &mut temp_rec);
            }
        }

        hit_anything
    }
}

impl HittableList {
    pub fn add(&mut self, obj: Rc<dyn Hittable>) {
        self.objs.push(obj);
    }
}
