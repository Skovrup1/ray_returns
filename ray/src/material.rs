use std::rc::Rc;

use crate::{hittable::HitRecord, vec::{Ray, Color, random_in_hemisphere, NearZeroExt, ReflectExt, Vect3}};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        false
    }
}


impl dyn Material {
    pub fn default() -> Rc<Self> {
        Rc::new(Lambertian::new(Default::default()))
    }
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_dir = random_in_hemisphere(&rec.normal);

        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_dir);
        *attenuation = self.albedo;

        true
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflect =  r_in.dir.normalize().reflect(rec.normal);
        *scattered = Ray::new(rec.p, reflect); 
        *attenuation = self.albedo;

        scattered.dir.dot(&rec.normal) > 0.0
    }
}