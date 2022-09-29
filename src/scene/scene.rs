use image::Rgb;
use na::{Vector3, UnitVector3, Unit, base};

use crate::*;

use math::{Vec3, ray::Ray};

use super::HitRecord;

pub struct Scene {
    objects: Vec<Box<dyn Hittable>>,
}

impl Scene {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
        Scene {
            objects: objects
        }
    }

    pub fn raytrace(&self, r: &Ray) -> Option<HitRecord>  {
        let mut hit_record: Option<HitRecord> = None;

        let mut t_min: Real = 0.0;
        let mut t_max: Real = Real::INFINITY;

        for obj in self.objects.iter() {
            if let Some(rec) = obj.hit(r, t_min, t_max) {
                t_max = rec.t;
                hit_record = Some(rec);
            }
        }

        hit_record
    }
}

pub fn ray_color(r: &Ray) -> Rgb<u8> {
    let unit = r.direction();
    let t = 0.5 * (unit.y + 1.0);
    let color = (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);

    Rgb([(color.x * 255.999) as u8, (color.y * 255.999) as u8, (color.z * 255.999) as u8])
}