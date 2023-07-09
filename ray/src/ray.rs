#![allow(dead_code)]

use nalgebra::Vector3;

use crate::hitable::{HitRecord, Hitable};
use crate::hitable_list::HitableList;
use crate::vec::Color;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    origin: Vector3<f32>,
    dir: Vector3<f32>,
}

impl Ray {
    pub fn zero() -> Ray {
        Ray {
            origin: Vector3::zeros(),
            dir: Vector3::zeros(),
        }
    }

    pub fn new(origin: Vector3<f32>, dir: Vector3<f32>) -> Ray {
        Ray { origin, dir }
    }

    pub fn origin(&self) -> Vector3<f32> {
        self.origin
    }

    pub fn dir(&self) -> Vector3<f32> {
        self.dir
    }

    pub fn at(&self, t: f32) -> Vector3<f32> {
        self.origin() + self.dir() * t
    }

    pub fn color(&self, rec: &mut HitRecord, world: &HitableList, depth: i32) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if world.hit(self, 0.001, f32::INFINITY, rec) {
            let mut scattered = Ray::zero();
            let mut attenuation = Color::zeros();

            let is_scattered = rec.mat.scatter(self, rec, &mut attenuation, &mut scattered);

            if is_scattered {
                return attenuation.component_mul(&scattered.color(rec, world, depth - 1));
            } else {
                return Color::new(0.0, 0.0, 0.0);
            }
        }

        let unit_direction = self.dir().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);

        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
