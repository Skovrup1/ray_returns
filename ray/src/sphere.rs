use nalgebra::Vector3;

use crate::aabb::AABB;
use crate::hitable::*;
use crate::material::*;
use crate::ray::*;

pub struct Sphere {
    center: Vector3<f32>,
    radius: f32,
    mat: Material,
    bbox: AABB,
}

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32, mat: Material) -> Sphere {
        let rvec = Vector3::new(radius, radius, radius);
        let bbox = AABB {
            min: center - rvec,
            max: center + rvec,
        };

        Sphere {
            center,
            radius,
            mat,
            bbox,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.dir().norm_squared();
        let half_b = oc.dot(&r.dir());
        let c = oc.norm_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;

            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut hit = HitRecord::default();
        hit.t = root;
        hit.p = r.at(hit.t);
        let outward_normal = (hit.p - self.center) / self.radius;
        hit.set_face_normal(r, outward_normal);
        hit.mat = self.mat;

        Some(hit)
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(self.bbox)
    }
}
