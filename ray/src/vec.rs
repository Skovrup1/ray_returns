use nalgebra as na;

pub type Vect3 = na::Vector3<f32>;
pub type Poin3 = na::Vector3<f32>;
pub type Color = na::Vector3<f32>;

pub struct Ray {
    pub orig: Poin3,
    pub dir: Vect3,
}

impl Ray {
    pub fn new(orig: Poin3, dir: Vect3) -> Self {
        Ray { orig, dir }
    }

    pub fn at(&self, t: f32) -> Poin3 {
        self.orig + t * self.dir
    }
}

// first method of lambertian diffuse
pub fn random_in_unit_sphere() -> Vect3 {
    loop {
        let p = Vect3::new_random();
        if p.magnitude_squared() >= 1.0 {
            return p;
        }
    }
}
pub fn random_in_unit_vector() -> Vect3 {
    random_in_unit_sphere().normalize()
}

// second method of lembertian diffuse
pub fn random_in_hemisphere(normal: &Vect3) -> Vect3 {
    let unit_sphere = random_in_unit_sphere();
    if unit_sphere.dot(&normal) > 0.0 {
        unit_sphere
    } else {
        -unit_sphere
    }
}