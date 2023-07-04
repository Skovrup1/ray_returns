use std::{rc::Rc, os::windows::thread};

use rand::{thread_rng, Rng};

use crate::{
    hittable::HitRecord,
    vec::{
        random_in_hemisphere, random_in_unit_sphere, Color, NearZeroExt, Ray, ReflectExt, Vect3,
    },
};

pub trait Material {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
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
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
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
    fuzzy: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzzy: f32) -> Self {
        Metal { albedo, fuzzy }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflect = r_in.dir.normalize().reflect(rec.normal);
        *scattered = Ray::new(rec.p, reflect + self.fuzzy * random_in_unit_sphere());
        *attenuation = self.albedo;

        scattered.dir.dot(&rec.normal) > 0.0
    }
}

pub struct Dielectric {
    //Index of refraction
    ir: f32,
}

impl Dielectric {
    pub fn new(ir: f32) -> Self {
        Dielectric { ir }
    }
}

fn refraction(uv: &Vect3, n: &Vect3, etai_etat: f32) -> Vect3 {
    let cos_theta = f32::min(n.dot(&-uv), 1.0);
    let r_out_perp = etai_etat * (uv + cos_theta * n);
    let r_out_parallel = -((1.0 - r_out_perp.magnitude_squared()).abs().sqrt()) * n;

    r_out_perp + r_out_parallel
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);

        let refraction_ratio = if rec.front_face {
            self.ir.recip()
        } else {
            self.ir
        };

        let unit_dir = r_in.dir.normalize();
        let cos_theta = ((-unit_dir).dot(&rec.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let should_reflect = reflectance(cos_theta, refraction_ratio) > thread_rng().gen();

        let dir = if cannot_refract || should_reflect {
            unit_dir.reflect(rec.normal)
        } else {
            refraction(&unit_dir, &rec.normal, refraction_ratio)
        };
        *scattered = Ray::new(rec.p, dir);

        true
    }
}

fn reflectance(cos_theta: f32, refraction_ratio: f32) -> f32 {
    //schlick approx
    let r = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
    let r = r * r;

    r + (1.0 - r) * (1.0 - cos_theta).powi(5)
}
