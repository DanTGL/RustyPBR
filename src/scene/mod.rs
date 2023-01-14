use std::sync::Arc;

use crate::math::{Real, ray::Ray, Vec3};

use self::material::{Material, MaterialResult};

pub mod scene;
pub mod sphere;
pub mod material;
pub mod camera;

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
}