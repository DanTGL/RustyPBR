
use image::{ImageBuffer, RgbImage, Rgb};
use math::{Vec3, Real};
use na::{Point3, Vector3, Const, OPoint};
use scene::{sphere::{self, Sphere}, Hittable, scene::Scene};

use crate::{scene::scene::ray_color, math::ray::Ray};

extern crate nalgebra as na;
extern crate image;

mod math;
mod scene;

const WIDTH: u32 = 256;
const ASPECT_RATIO: Real = 16.0 / 9.0;
const HEIGHT: u32 = (WIDTH as Real / ASPECT_RATIO) as u32;

fn gen_image(scene: &Scene, width: u32, height: u32, f: fn(r: &Ray) -> Rgb<u8>) -> RgbImage {
    let viewport_height: Real = 2.0;
    let viewport_width: Real = ASPECT_RATIO * viewport_height;
    let focal_length: Real = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let mut imgbuf: RgbImage = image::ImageBuffer::new(width, height);

    //let s1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let u = (x as Real) / ((WIDTH - 1) as Real);
        let v = (y as Real) / ((HEIGHT - 1) as Real);

        let r = Ray::new(origin, lower_left + u * horizontal + v * vertical - origin);

        if let Some(rec) = scene.raytrace(&r) {
            *pixel = Rgb([255, 255, 0]);
        } else {
            *pixel = Rgb([0, 0, 0]);
        //*pixel = f(&r);
        }
        
    }

    imgbuf
}

fn main() {
    use std::time::Instant;
    println!("Hello, world!");
    
    let scene = Scene::new(vec![
        Box::new(Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
        }),
        Box::new(Sphere {
            center: Vec3::new(-2.0, 0.0, -3.0),
            radius: 0.25,
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, 4.0, -1.0),
            radius: 0.35,
        }),
        ]);
        
        
    let now = Instant::now();
    {
        let img = gen_image(&scene, 256, 256, |r: &Ray| ray_color(r));
        img.save("image.bmp").unwrap();
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    
}
