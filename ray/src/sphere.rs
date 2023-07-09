use nalgebra::Vector3;

use crate::hitable::*;
use crate::material::*;
use crate::ray::*;
use std::rc::Rc;

pub struct Sphere {
    center: Vector3<f32>,
    radius: f32,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32, mat: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            mat,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.dir().norm_squared();
        let half_b = oc.dot(&r.dir());
        let c = oc.norm_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;

            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        rec.mat = self.mat.clone();

        true
    }
}
