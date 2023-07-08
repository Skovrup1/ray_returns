#![allow(dead_code)]

use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use crate::utility::{between_float, random_float};

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    arr: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { arr: [x, y, z] }
    }

    pub fn zero() -> Vec3 {
        Vec3 {
            arr: [0.0, 0.0, 0.0],
        }
    }

    pub fn x(&self) -> f32 {
        self.arr[0]
    }

    pub fn y(&self) -> f32 {
        self.arr[1]
    }

    pub fn z(&self) -> f32 {
        self.arr[2]
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.arr[0] * self.arr[0] + self.arr[1] * self.arr[1] + self.arr[2] * self.arr[2]
    }

    // dot product
    pub fn dot(&self, other: &Vec3) -> f32 {
        self.arr[0] * other.arr[0] + self.arr[1] * other.arr[1] + self.arr[2] * other.arr[2]
    }

    // cross product
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.arr[1] * other.arr[2] - self.arr[2] * other.arr[1],
            self.arr[2] * other.arr[0] - self.arr[0] * other.arr[2],
            self.arr[0] * other.arr[1] - self.arr[1] * other.arr[0],
        )
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn random() -> Vec3 {
        Vec3::new(random_float(), random_float(), random_float())
    }

    pub fn random_between(min: f32, max: f32) -> Vec3 {
        Vec3::new(
            between_float(min, max),
            between_float(min, max),
            between_float(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_between(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    pub fn near_zero(&self) -> bool {
        const S: f32 = 1e-8;
        (self.arr[0].abs() < S) && (self.arr[1].abs() < S) && (self.arr[2].abs() < S)
    }

    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - 2.0 * n * self.dot(&n)
    }

    pub fn refract(self, n: Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = n.dot(&-self).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * n;

        r_out_perp + r_out_parallel
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(between_float(-1.0, 1.0), between_float(-1.0, 1.0), 0.0);
            if p.length_squared() >= 1.0 {
                return p;
            }
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Vec3::new(
            self.x() * other.x(),
            self.y() * other.y(),
            self.z() * other.z(),
        )
    }
}

// Scalar multiplication
impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Vec3::new(self.x() * other, self.y() * other, self.z() * other)
    }
}

// Scalar multiplication
impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self * other.x(), self * other.y(), self * other.z())
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Vec3::new(
            self.x() / other.x(),
            self.y() / other.y(),
            self.z() / other.z(),
        )
    }
}

// Scalar division
impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Vec3::new(self.x() / other, self.y() / other, self.z() / other)
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Self) -> () {
        self.arr[0] += other.x();
        self.arr[1] += other.y();
        self.arr[2] += other.z();
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, other: Self) -> () {
        self.arr[0] -= other.x();
        self.arr[1] -= other.y();
        self.arr[2] -= other.z();
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Self) -> () {
        self.arr[0] *= other.x();
        self.arr[1] *= other.y();
        self.arr[2] *= other.z();
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) -> () {
        self.arr[0] *= other;
        self.arr[1] *= other;
        self.arr[2] *= other;
    }
}

impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, other: Self) -> () {
        self.arr[0] /= other.x();
        self.arr[1] /= other.y();
        self.arr[2] /= other.z();
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, i: usize) -> &Self::Output {
        &self.arr[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.arr[i]
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;
