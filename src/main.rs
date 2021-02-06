mod vec3;
mod ray;

use std::io::{self, Write};
use vec3::Vec3;
use ray::Ray;

fn ray_color(ray: &Ray) -> Vec3 {
    if hit_sphere(Vec3(0.0, 0.0, -1.0), 0.5, &ray) {
        return Vec3(1.0, 0.0, 0.0);
    }
    let unit_direction: Vec3 = ray.direction.unit_vector();
    let t: f32 = 0.5 * (unit_direction.y() + 1.0);
    (1.0-t)*Vec3(1.0, 1.0, 1.0) + t*Vec3(0.5, 0.7, 1.0)
}

fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> bool {
    let oc: Vec3 = ray.origin - center;
    let a: f32 = ray.direction.dot(ray.direction);
    let b: f32 = 2.0 * oc.dot(ray.direction);
    let c: f32 = oc.dot(oc) - radius*radius;
    let discriminant: f32 = b*b - 4.0*a*c;
    discriminant > 0.0
}


fn main() {

    // Image

    const aspect_ratio: f32 = 16.0 / 9.0;
    const image_width: i32 = 400;
    const image_height: i32 = (image_width as f32 / aspect_ratio) as i32;

    // Camera

    let viewport_height: f32 = 2.0;

    let viewport_width: f32 = aspect_ratio * viewport_height;
    let focal_length: f32 = 1.0;

    let origin: Vec3 = Vec3(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3(viewport_width, 0.0, 0.0);
    let vertical: Vec3 = Vec3(0.0, viewport_height, 0.0);
    let lower_left_corner: Vec3 = origin - horizontal/2.0 - vertical/2.0 - Vec3(0.0, 0.0, focal_length);

    // render

    println!("P3\n{} {}\n255", image_width, image_height);
    for j in (0..image_height).rev() {
        eprint!("\rPercent complete: {}%", ((image_height - j) * 100 / (image_height)));
        for i in 0..image_width {
            let u: f32 = i as f32 / (image_width-1) as f32;
            let v: f32 = j as f32 / (image_height-1) as f32;
            let r: Ray = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            let pixel_color: Vec3 = ray_color(&r);
            pixel_color.write_color();
        }
    }
}
