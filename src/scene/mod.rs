use image::Rgb;

use crate::math::{Real, ray::Ray, Vec3};

use self::material::Material;

pub mod scene;
pub mod sphere;
pub mod material;

pub struct HitRecord {
    pub pos: Vec3,
    pub normal: Vec3,
    pub t: Real,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: Real, t_max: Real) -> Option<HitRecord> {
        None
    }
}