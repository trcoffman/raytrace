use std::ops::*;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Copy, Clone)]
pub struct Scalar(pub f32);

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn origin() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn make_unit(&self) -> Vec3 {
        let norm = self.norm();
        Vec3::new(self.x / norm, self.y / norm, self.z / norm)
    }
}

impl<'a> Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

pub fn dot(v1: Vec3, v2: Vec3) -> f32 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

impl Add<Vec3> for Scalar {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        let Scalar(value) = self;
        Vec3::new(value + other.x, value + other.y, value + other.z)
    }
}

impl Mul<Vec3> for Scalar {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        let Scalar(value) = self;
        Vec3::new(value * other.x, value * other.y, value * other.z)
    }
}
