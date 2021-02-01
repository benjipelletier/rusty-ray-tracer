mod vec3;

use vec3::Vec3;

fn main() {
    let nx: i32 = 200;
    let ny: i32 = 200;
    println!("P3\n{} {}\n255", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let pixel_color = Vec3(
                i as f32 / (nx-1) as f32,
                j as f32 / (ny-1) as f32,
                0.25,
            );
            pixel_color.write_color();
        }
    }
}
