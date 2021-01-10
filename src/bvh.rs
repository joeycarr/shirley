use crate::aabb::{AABB, surrounding_box};
use crate::hit::{Hit, HitArc, hit_compare, HitList};
use crate::rand::randrange;
use std::cmp::Ordering;
use std::sync::Arc;

pub struct BVHNode {
    left: HitArc,
    right: HitArc,
    bounds: AABB,
}

impl BVHNode {
    pub fn from_hitlist(list: &HitList, time0: f64, time1: f64) -> BVHNode {
        BVHNode::from_vec(&list.objects, 0, list.objects.len(), time0, time1)
    }

    pub fn from_vec(src_objects: &Vec<HitArc>, start: usize, end: usize, time0: f64, time1: f64) -> BVHNode {
        let mut objects = src_objects.clone();
        let bounds = AABB::default();
        let axis = randrange(0.0, 3.0).trunc() as usize;

        let comparator = |a: &HitArc, b: &HitArc| hit_compare(Arc::clone(&a), Arc::clone(&b), axis);

        let object_span = end - start;

        let mut node = match object_span {
            1 => BVHNode{
                left: Arc::clone(&objects[start]),
                right: Arc::clone(&objects[start]),
                bounds,
            },
            2 => match comparator(&objects[start], &objects[start+1]) {
                Ordering::Greater => {
                    BVHNode{
                        left: Arc::clone(&objects[start]),
                        right: Arc::clone(&objects[start+1]),
                        bounds,
                    }
                },
                _ => {
                    BVHNode{
                        left: Arc::clone(&objects[start+1]),
                        right: Arc::clone(&objects[start]),
                        bounds,
                    }
                },
            },
            _ => {
                objects[start..end].sort_unstable_by(comparator);
                let mid = start + object_span / 2;
                BVHNode{
                    left: Arc::new(BVHNode::from_vec(&objects, start, mid, time0, time1)),
                    right: Arc::new(BVHNode::from_vec(&objects, mid, end, time0, time1)),
                    bounds,
                }
            },
        };

        let mut box_left = AABB::default();
        let mut box_right = AABB::default();

        if (! node.left.bounding_box(time0, time1, &mut box_left)) ||
        (! node.right.bounding_box(time0, time1, &mut box_right)) {
            // Is this a panic? How do we decide to skip objects without the box?
            panic!("Trying to add an object with no bounding box to a BVHNode");
        }

        node.bounds = surrounding_box(box_left, box_right);

        node
    }
}

impl Hit for BVHNode {
    fn hit(&self, ray: crate::ray::Ray, t_min: f64, t_max: f64, hitrec: &mut crate::hit::HitRecord) -> bool {
        if !self.bounds.hit(ray, t_min, t_max) {
            return false;
        }

        let hit_left = self.left.hit(ray, t_min, t_max, hitrec);
        let hit_right = self.right.hit(ray, t_min, if hit_left {hitrec.t} else {t_max}, hitrec);

        hit_left || hit_right
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, aabb: &mut AABB) -> bool {
        *aabb = self.bounds;
        true
    }
}
