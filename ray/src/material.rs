use rand::{Rng, thread_rng};

use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::vec::{random_in_hemisphere, random_in_unit_sphere, Color, ReflectExt, RefractExt};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
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
        let scatter_dir = random_in_hemisphere(rec.normal);

        let eps = f32::EPSILON;
        let is_zero = (scatter_dir.x.abs() < eps)
            && (scatter_dir.y.abs() < eps)
            && (scatter_dir.z.abs() < eps);

        let scatter_dir = if is_zero { rec.normal } else { scatter_dir };

        *scattered = Ray::new(rec.p, scatter_dir);
        *attenuation = self.albedo;

        true
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Metal {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
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
        let reflected = r_in.dir().normalize().reflect(rec.normal);

        *scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere());
        *attenuation = self.albedo;

        scattered.dir().dot(&rec.normal) > 0.0
    }
}

pub struct Dielectric {
    ir: f32,
}

impl Dielectric {
    pub fn new(ir: f32) -> Dielectric {
        Dielectric { ir }
    }
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

        let unit_direction = r_in.dir().normalize();

        let cos_theta = -unit_direction.dot(&rec.normal).min(1.0);
        let sin_theta = (1.0f32 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let should_relect = reflectance(cos_theta, refraction_ratio) > thread_rng().gen();

        let direction = if cannot_refract || should_relect {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, refraction_ratio)
        };

        *scattered = Ray::new(rec.p, direction);

        true
    }
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0.powi(2);

    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
