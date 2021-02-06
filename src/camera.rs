mod ray;

use ray::Ray;

pub struct Camera {
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup) -> Camera
}