use image::Rgb;

use crate::math::ray::Ray;

use super::HitRecord;

pub struct MaterialResult {
    attenuation: Rgb<u8>,
    scattered: Ray,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<MaterialResult>;
}