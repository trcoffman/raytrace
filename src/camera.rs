
use ray::*;
use vec3::*;
use random::*;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(origin: Vec3, lower_left_corner: Vec3, horizontal: Vec3, vertical: Vec3) -> Camera {

        Camera {
            origin: origin,
            lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {

        let direction = &(&self.lower_left_corner + &(Scalar(u) * &self.horizontal)) +
                        &(&(Scalar(v) * &self.vertical) - &self.origin);
        Ray::new(self.origin, direction)
    }

    pub fn get_randomized_ray(&self, x: u32, nx: u32, y: u32, ny: u32) -> Ray {
        self.get_randomized_ray_impl(x as f32, nx as f32, y as f32, ny as f32)
    }

    pub fn get_randomized_ray_impl(&self, x: f32, nx: f32, y: f32, ny: f32) -> Ray {

        let u = (x + rand_between0and1()) / nx;
        let v = (y + rand_between0and1()) / ny;
        self.get_ray(u, v)
    }
}
