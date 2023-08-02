use nalgebra::Vector3;

use crate::ray::Ray;

pub struct AABB {
    min: Vector3<f32>,
    max: Vector3<f32>,
}

impl AABB {
    #[inline]
    pub fn hit(self, r: &Ray, mut t_min: f32, mut t_max: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.dir()[a];
            let mut t0 = (self.min[a] - r.origin()[a]) * inv_d;
            let mut t1 = (self.max[a] - r.origin()[a]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            if t0 > t_min {
                t_min = t0;
            }
            if t1 < t_max {
                t_max = t1;
            }

            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}
