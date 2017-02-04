
use ray::*;
use vec3::*;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {

    pub fn new( origin: Vec3,
            lower_left_corner: Vec3,
            horizontal: Vec3,
            vertical: Vec3) -> Camera {
        
        Camera {
            origin: origin,
            lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {

        let direction = &(&self.lower_left_corner + &(Scalar(u) * &self.horizontal)) + &(&(Scalar(v) * &self.vertical) - &self.origin);
        Ray::new(self.origin, direction)
    }
   
}

