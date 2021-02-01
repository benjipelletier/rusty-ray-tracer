use std::ops::*;


#[derive(Clone, Copy, Debug)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn x(&self) -> f32 { self.0 }
    pub fn y(&self) -> f32 { self.1 }
    pub fn z(&self) -> f32 { self.2 }

    pub fn r(&self) -> f32 { self.0 }
    pub fn g(&self) -> f32 { self.1 }
    pub fn b(&self) -> f32 { self.2 }

    pub fn dot(&self, other: Vec3) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3(self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0)
    }

    pub fn length(self) -> f32 { self.squared_length().sqrt() }
    pub fn squared_length(self) -> f32 { self.dot(self) }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn write_color(&self) {
        println!(
            "{} {} {}", 
            (255.999 * self.0) as i32,  
            (255.999 * self.1) as i32, 
            (255.999 * self.2) as i32
        );
    }
}

// Vec3 <-> Vec3 ops

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3(0.0, 0.0, 0.0) - self
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

// Vec3 <-> f32 ops

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, f: f32) -> Vec3 {
        Vec3(self.0 * f, self.1 * f, self.2 * f)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, f: f32) -> Vec3 {
        self * (1.0 / f)
    }
}