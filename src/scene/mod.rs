use std::sync::Arc;

use crate::math::{Real, ray::Ray, Vec3};

use self::material::{Material, MaterialResult};

pub mod scene;
pub mod sphere;
pub mod material;
pub mod camera;

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