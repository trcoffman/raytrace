use hitable::*;
use vec3::*;
use ray::*;
use material::Material;

pub struct Sphere<'a> {
    pub center: Vec3,
    pub radius: f32,
    pub material: &'a Material,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Vec3, radius: f32, material: &'a Material) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            material: material,
        }
    }
}

impl<'s> Hitable for Sphere<'s> {

    fn hit<'a, 'b>(&'a self, ray: &'b Ray, tMin: f32, tMax: f32) -> Option<HitRecord> {

        let oc = &ray.origin - &self.center;
        let a = dot(&ray.direction, &ray.direction);
        let b = 2.0 * dot(&oc, &ray.direction);
        let c = dot(&oc, &oc) - (self.radius * self.radius);
        let discriminant = (b * b) - (4.0 * a * c);

        if discriminant > 0.0 {
            // quadratic formula
            let tMinus = (-b - discriminant.sqrt()) / (2.0 * a);
            let tPlus: f32;
            if tMinus >= tMin && tMinus <= tMax {
                let t = tMinus;
                let p = ray.point_at(t);
                let normal = Scalar(1.0 / self.radius) * &(&p- &self.center);

                return Some(HitRecord::new(t, p, normal, self.material));
            }
            tPlus = (-b + discriminant.sqrt()) / (2.0 * a);
            if tPlus >= tMin && tPlus <= tMax {
                let t = tPlus;
                let p = ray.point_at(t);
                let normal = Scalar(1.0 / self.radius) * &(&p- &self.center);

                return Some(HitRecord::new(t, p, normal, self.material));
            }
        } 

        None
    }
}
