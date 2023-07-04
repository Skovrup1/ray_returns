use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    vec::{Poin3, Ray},
};

pub struct Sphere {
    pub center: Poin3,
    pub radius: f32,
    pub mat: Rc<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.orig - self.center;
        let a = r.dir.magnitude_squared();
        let half_b = oc.dot(&r.dir);
        let c = oc.magnitude_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            let root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat = self.mat.clone();

        true
    }
}

impl Sphere {
    pub fn new(center: Poin3, radius: f32, mat: Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            mat,
        }
    }
}
