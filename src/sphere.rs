
use hitable::*;
use vec3::*;
use ray::*;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}

impl Hitable for Sphere {
    fn hit<'a, 'b>(&'a self, ray: &'b Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {

        let oc = &ray.origin - &self.center;
        let a = dot(&ray.direction, &ray.direction);
        let b = 2.0 * dot(&oc, &ray.direction);
        let c = dot(&oc, &oc) - (self.radius * self.radius);
        let discriminant = (b * b) - (4.0 * a * c);

        if discriminant > 0.0 {
            // quadratic formula
            let t_minus = (-b - discriminant.sqrt()) / (2.0 * a);
            let t_plus: f32;
            if t_minus >= t_min && t_minus <= t_max {
                let t = t_minus;
                let p = ray.point_at(t);
                let normal = Scalar(1.0 / self.radius) * &(&p - &self.center);

                return Some(HitRecord::new(t, p, normal));
            }
            t_plus = (-b + discriminant.sqrt()) / (2.0 * a);
            if t_plus >= t_min && t_plus <= t_max {
                let t = t_plus;
                let p = ray.point_at(t);
                let normal = Scalar(1.0 / self.radius) * &(&p - &self.center);

                return Some(HitRecord::new(t, p, normal));
            }
        }

        None
    }
}
