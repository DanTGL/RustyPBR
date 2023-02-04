use super::*;

use rand::{Rng, RngCore};

enum BVHNode {
    Branch { left: Box<BVH>, right: Box<BVH> },
    Leaf(Box<dyn Hittable>)
}

pub struct BVH {
    node: BVHNode,
    pub aabb: AABB,
}

impl Hittable for BVH {
    fn hit(&self, ray: &Ray, t_min: Real, t_max: Real) -> Option<HitRecord> {
        if !self.aabb.hit(ray, t_min, t_max) {
            None
        } else {
            match &self.node {
                BVHNode::Branch { left, right } => {
                    let hit_left = left.hit(ray, t_min, t_max);
                    let t = {
                        if let Some(ref hit) = hit_left {
                            hit.t
                        } else {
                            t_max
                        }
                    };

                    let hit_right = {
                        right.hit(ray, t_min, t)
                    };
                    
        
                    hit_right.or(hit_left)
            
                },
                BVHNode::Leaf(hittable) => hittable.hit(ray, t_min, t_max),
            }

            
        }
    }
    fn bounding_box(&self, t0: Real, t1: Real) -> Option<AABB> {
        Some(self.aabb)
    }
}

impl BVH {
    
    pub fn new(rng: &mut dyn RngCore, list: Vec<Box<dyn Hittable>>, time0: Real, time1: Real) -> Option<Self> {
        Self::new_recur(rng, list, time0, time1)
    }

    fn new_recur(rng: &mut dyn RngCore, mut objects: Vec<Box<dyn Hittable>>, time0: Real, time1: Real) -> Option<Self> {

        let axis = rng.gen_range(0..3);

        let comparator = box_compare;

        let len = objects.len();

        match len {
            0 => None,

            1 => {
                let leaf = objects.pop().unwrap();
                let aabb = leaf.bounding_box(time0, time1)?;
                Some(BVH { node: BVHNode::Leaf(leaf), aabb: aabb })
            },

            /*2 => {
                if comparator(objects[start].as_ref(), objects[start+1].as_ref(), axis) == Some(Ordering::Less) {
                    (objects[start].clone(), objects[start+1].clone())
                } else {
                    (objects[start+1].clone(), objects[start].clone())
                }
            },*/

            _ => {
                objects.sort_by(|a, b| comparator(a.as_ref(), b.as_ref(), axis).unwrap() );

                let mid = objects.len() / 2;
                let right = Self::new_recur(rng, objects.drain(len / 2..).collect(), time0, time1).unwrap();
                let left = Self::new_recur(rng, objects, time0, time1).unwrap();

                let aabb = surrounding_box(left.aabb, right.aabb);
                Some(BVH {
                    node: BVHNode::Branch {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    
                    aabb: aabb
                })
            }
        }/*;

        let box_left = left.bounding_box(time0, time1)?;
        let box_right = right.bounding_box(time0, time1)?;

        Some(Self {
            left: left,
            right: right,
            aabb: surrounding_box(box_left, box_right)
        })*/
    }
}

