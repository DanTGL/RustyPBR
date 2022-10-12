use crate::*;
use crate::math::*;
use crate::math::ray::Ray;

use super::HitRecord;

const PORTAL_RADIUS: Real = 0.5;

struct PortalMaterial {
    other_portal_pos: Vec3
}

impl Material for PortalMaterial {
    fn scatter(&self, rng: &mut dyn RngCore, ray: &Ray, hit_record: &HitRecord) -> Option<MaterialResult> {
        let transported_origin = self.other_portal_pos - hit_record.normal;
        Some(MaterialResult {
            attenuation: Vec3::new(1.0, 1.0, 1.0),
            scattered: Ray {
                origin: transported_origin,
                dir: ray.direction()
            }
        })
    }
}

pub struct Portal {
    portal1: Sphere,
    portal2: Sphere,
}

impl Portal {
    pub fn new(portal_1_pos: Vec3, portal_2_pos: Vec3) -> Self {
        Self {
            portal1: Sphere {
                center: portal_1_pos,
                radius: PORTAL_RADIUS,
                material: Arc::new(PortalMaterial {
                    other_portal_pos: portal_2_pos
                })
            },
            portal2: Sphere {
                center: portal_1_pos,
                radius: PORTAL_RADIUS,
                material: Arc::new(PortalMaterial {
                    other_portal_pos: portal_1_pos
                })
            }
        }
    }
}

impl Hittable for Portal {
    fn hit(&self, ray: &math::ray::Ray, t_min: Real, t_max: Real) -> Option<scene::HitRecord> {
        let mut hit_record: Option<HitRecord> = None;

        let mut closest: Real = t_max;

        if let Some(rec) = self.portal1.hit(ray, t_min, closest) {
            closest = rec.t;
            hit_record = Some(rec);
        }

        if let Some(rec) = self.portal2.hit(ray, t_min, closest) {
            hit_record = Some(rec);
        }

        hit_record
    }
}