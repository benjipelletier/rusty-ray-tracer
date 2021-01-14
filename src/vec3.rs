struct Vec3 {
    e: [f32; 3];
}

impl Vec3 {
    fn new (x: f32, y: f32, z:f32) {
        Vec3 { e: [x, y, z] }
    }

    fn x(&self) -> f32 {
        self.e[0]
    }

    fn y(&self) -> f32 {
        self.e[1]
    }

    fn z(&self) -> f32 {
        self.e[2]
    }

    fn r(&self) -> f32 {
        self.e[0]
    }

    fn g(&self) -> f32 {
        self.e[1]
    }

    fn b(&self) -> f32 {
        self.e[2]
    }

    fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    fn squared_length(&self) -> f32 {
        e[0]*e[0] + e[1]*e[1] + e[2]*e[2] 
    }
}