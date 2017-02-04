use vec3::Vec3;
use vec3::Scalar;

#[derive(Copy,Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin: origin,
            direction: direction.make_unit(),
        }
    }

    pub fn point_at(&self, t: f32) -> Vec3 {
        &(self.origin) + &(Scalar(t) * &(self.direction))
    }
}
