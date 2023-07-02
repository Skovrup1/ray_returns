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
