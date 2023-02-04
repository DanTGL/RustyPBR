use std::{sync::Arc, cmp::Ordering};

use crate::math::{Real, ray::Ray, Vec3};

use self::material::{Material, MaterialResult};

pub mod scene;
pub mod sphere;
pub mod material;
pub mod camera;
pub mod bvh;

#[derive(Copy, Clone)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn hit(&self, ray: &Ray, mut t_min: Real, mut t_max: Real) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction()[a];
            let mut t0 = (self.min[a] - ray.origin()[a]) * inv_d;
            let mut t1 = (self.max[a] - ray.origin()[a]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t_min = {
                if t0 > t_min {
                    t0
                } else {
                    t_min
                }
            };
            t_max = {
                if t1 < t_max {
                    t1
                } else {
                    t_max
                }
            };

            if t_max <= t_min {
                return false;
            }
        }

        return true;
    }

}

pub fn box_compare(a: &dyn Hittable, b: &dyn Hittable, axis: u8) -> Option<Ordering> {
    let box_a = a.bounding_box(0.0, 0.0);
    let box_b = b.bounding_box(0.0, 0.0);

    if box_a.is_none() || box_b.is_none() {
        println!("No bounding box in BVHNode constructor.");
    }

    box_a.unwrap().min[axis as usize].partial_cmp(&box_b.unwrap().min[axis as usize])
}

pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
    let small: Vec3 = Vec3::from([
        box0.min.x.min(box1.min.x),
        box0.min.y.min(box1.min.y),
        box0.min.z.min(box1.min.z)
    ]);

    let big: Vec3 = Vec3::from([
        box0.max.x.max(box1.max.x),
        box0.max.y.max(box1.max.y),
        box0.max.z.max(box1.max.z)
    ]);

    AABB {
        min: small,
        max: big,
    }
}

pub struct HitRecord {
    pub pos: Vec3,
    pub normal: Vec3,
    pub t: Real,
    pub front_face: bool,
    pub mat: Arc<dyn Material>,
    #[cfg(feature = "debug")]
    pub hit_edge: bool,
}

impl HitRecord {
    
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: Real, t_max: Real) -> Option<HitRecord> {
        None
    }

    fn bounding_box(&self, t0: Real, t1: Real) -> Option<AABB> {
        None
    }
    
}