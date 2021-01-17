
use crate::ray::Ray;
use crate::vec3::Point3;
use std::cmp::Ordering;

#[derive(Debug, Default, Copy, Clone)]
pub struct AABB {
    pub min: Point3,
    pub max: Point3,
}

impl AABB {
    pub fn new(min: Point3, max: Point3) -> AABB {
        AABB { min, max }
    }

    pub fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> bool {
        let min = self.min.as_array();
        let max = self.max.as_array();
        let direction = ray.direction.as_array();
        let origin = ray.origin.as_array();

        for a in 0..3 {
            let inv_d = 1.0 / direction[a];
            let mut t0 = (min[a] - origin[a]) * inv_d;
            let mut t1 = (max[a] - origin[a]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };

            if t_max <= t_min {
                return false;
            }
        }
        return true;
    }
}

pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
    let small = Point3::new(
        box0.min.x.min(box1.min.x),
        box0.min.y.min(box1.min.y),
        box0.min.z.min(box1.min.z),
    );
    let big = Point3::new(
        box0.max.x.max(box1.max.x),
        box0.max.y.max(box1.max.y),
        box0.max.z.max(box1.max.z),
    );
    AABB::new(small, big)
}

pub fn aabb_compare(a: AABB, b: AABB, axis: usize) -> Ordering {
        let a = a.min.as_array();
        let b = b.min.as_array();

        if a[axis] < b[axis] {
            Ordering::Less
        } else if a[axis] > b[axis] {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
}
