use crate::vec::{Poin3, Ray, Vect3};

pub struct Camera {
    pub orig: Poin3,
    pub lower_left_corner: Poin3,
    pub horizontal: Vect3,
    pub vertical: Vect3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let orig = Poin3::new(0.0, 0.0, 0.0);
        let horizontal = Vect3::new(viewport_width, 0.0, 0.0);
        let vertical = Vect3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            orig - horizontal / 2.0 - vertical / 2.0 - Vect3::new(0.0, 0.0, focal_length);
        Camera {
            orig,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.orig,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.orig,
        )
    }
}
