mod vec3;
mod ray;
mod objects;

use vec3::Vec3;
use ray::Ray;
use objects::{Hittable, HitRecord, HittableList, Sphere};

fn ray_color(ray: &Ray, world: &HittableList) -> Vec3 {
    let mut rec = HitRecord::new();
    if world.hit(&ray, 0.0001, f32::MAX, &mut rec) {
        return 0.5 * (rec.normal + Vec3(1.0, 1.0, 1.0));
    }
    let unit_direction: Vec3 = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0-t)*Vec3(1.0, 1.0, 1.0) + t*Vec3(0.5, 0.7, 1.0)
}


fn main() {
    // Image

    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;

    // World
    let sphere_1 = Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5);
    let sphere_2 = Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0);

    let mut world = HittableList::new();
    world.add(sphere_1);
    world.add(sphere_2);

    // Camera

    let viewport_height: f32 = 2.0;

    let viewport_width: f32 = ASPECT_RATIO * viewport_height;
    let focal_length: f32 = 1.0;

    let origin: Vec3 = Vec3(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3(viewport_width, 0.0, 0.0);
    let vertical: Vec3 = Vec3(0.0, viewport_height, 0.0);
    let lower_left_corner: Vec3 = origin - horizontal/2.0 - vertical/2.0 - Vec3(0.0, 0.0, focal_length);

    // render

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rPercent complete: {}%", ((IMAGE_HEIGHT - j) * 100 / (IMAGE_HEIGHT)));
        for i in 0..IMAGE_WIDTH {
            let u: f32 = i as f32 / (IMAGE_WIDTH-1) as f32;
            let v: f32 = j as f32 / (IMAGE_HEIGHT-1) as f32;
            let r: Ray = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            let pixel_color: Vec3 = ray_color(&r, &world);
            pixel_color.write_color();
        }
    }
}
