use std::cmp;

use nalgebra::Vector3;

use crate::ray::Ray;

#[derive(Copy, Clone, Default)]
pub struct AABB {
    pub min: Vector3<f32>,
    pub max: Vector3<f32>,
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

    pub fn new_from_aabb(first_box: AABB, other_box: AABB) -> AABB {
        let x = first_box.min[0]
            .min(other_box.min[0])
            .min(first_box.max[0].max(other_box.max[0]));

        let y = first_box.min[1]
            .min(other_box.min[1])
            .min(first_box.max[1].max(other_box.max[1]));

        let z = first_box.min[2]
            .min(other_box.min[2])
            .min(first_box.max[2].max(other_box.max[2]));

        AABB {}
    }

    pub fn surrounding_box(&self, output_box: AABB) -> AABB {}
}
