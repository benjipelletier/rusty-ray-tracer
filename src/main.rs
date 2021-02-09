mod vec3;
mod ray;
mod objects;
mod camera;

use vec3::Vec3;
use ray::Ray;
use objects::{Hittable, HitRecord, HittableList, Sphere};
use rand::Rng;
use camera::Camera;



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
    let samples_per_pixel = 1000;
    let mut rng = rand::thread_rng();

    // World
    let sphere_1 = Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5);
    let sphere_2 = Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0);

    let mut world = HittableList::new();
    world.add(sphere_1);
    world.add(sphere_2);

    // Camera
    let origin: Vec3 = Vec3(0.0, 0.0, 0.0);
    let cam = Camera::new(origin);

    // render

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rPercent complete: {}%", ((IMAGE_HEIGHT - j) * 100 / (IMAGE_HEIGHT)));
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Vec3(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u: f32 = (i as f32 + rng.gen::<f32>()) / (IMAGE_WIDTH-1) as f32;
                let v: f32 = (j as f32 + rng.gen::<f32>()) / (IMAGE_HEIGHT-1) as f32;
                let r: Ray = cam.get_ray(u, v);
                pixel_color = ray_color(&r, &world) + pixel_color;
            }
            pixel_color.write_color(samples_per_pixel);
        }
    }
}
