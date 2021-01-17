use crate::aabb::AABB;
use crate::aarect::{XYRect, XZRect, YZRect};
use crate::hit::{Hit, HitArc, HitList, HitRecord};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Point3;

use std::sync::Arc;

pub struct Box3D {
    box_min: Point3,
    box_max: Point3,
    sides: HitList,
}

impl Box3D {
    pub fn new(p0: Point3, p1: Point3, material: Material) -> HitArc {
        let mut sides = HitList::default();

        sides.add(XYRect::new(p0.x, p1.x, p0.y, p1.y, p1.z, Arc::clone(&material)));
        sides.add(XYRect::new(p0.x, p1.x, p0.y, p1.y, p0.z, Arc::clone(&material)));

        sides.add(XZRect::new(p0.x, p1.x, p0.z, p1.z, p1.y, Arc::clone(&material)));
        sides.add(XZRect::new(p0.x, p1.x, p0.z, p1.z, p0.y, Arc::clone(&material)));

        sides.add(YZRect::new(p0.y, p1.y, p0.z, p1.z, p1.x, Arc::clone(&material)));
        sides.add(YZRect::new(p0.y, p1.y, p0.z, p1.z, p0.x, Arc::clone(&material)));

        Arc::new(Box3D{ box_min: p0, box_max: p1, sides })
    }
}

impl Hit for Box3D {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, hitrec: &mut HitRecord) -> bool {
        self.sides.hit(ray, t_min, t_max, hitrec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, aabb: &mut AABB) -> bool {
        *aabb = AABB::new(self.box_min, self.box_max);
        true
    }
}
