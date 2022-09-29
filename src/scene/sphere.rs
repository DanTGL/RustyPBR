use std::sync::Arc;

use super::*;

use material::Material;

pub struct Sphere {
    pub center: Vec3,
    pub radius: Real,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: Real, material: Arc<dyn Material>) -> Self {
        Sphere {
            center: center,
            radius: radius,
            material: material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: Real, t_max: Real) -> Option<HitRecord> {
        let oc: Vec3 = ray.origin() - self.center;
        let a = ray.direction().magnitude_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.magnitude_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            None
        } else {
            let t = ((-half_b - discriminant.sqrt()) / a);
            let hitPoint = ray.origin() + (t * ray.direction());
            Some(HitRecord {
                pos: hitPoint,
                normal: -(hitPoint - self.center) / self.radius,
                t: t,
            })
        }
    }
}

