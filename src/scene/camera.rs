use crate::math::{Vec3, Real, ray::Ray};


pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {

    pub fn new(orig: Vec3, aspect_ratio: Real, viewport_height: Real, focal_length: Real) -> Self {
        let viewport_width = aspect_ratio * viewport_height;

        //let orig = Vec3::zeros();
        let hori = Vec3::new(viewport_width, 0.0, 0.0);
        let vert = Vec3::new(0.0, viewport_height, 0.0);
        let llc = orig - hori / 2.0 - vert / 2.0 - Vec3::new(0.0, 0.0, focal_length);
        
        Self {
            origin: orig,
            horizontal: hori,
            vertical: vert,
            lower_left_corner: llc,
        }
    }

    pub fn get_ray(&self, u: Real, v: Real) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner +
                u * self.horizontal +
                v * self.vertical   -
                self.origin)
    }

}