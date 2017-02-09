use std::vec::Vec;
use std::boxed::Box;

use vec3::Vec3;
use vec3::Scalar;
use ray::Ray;
use hitable::HitRecord;

pub trait Material: Send + Sync {

    fn scatter(ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Vec3)>;

}

pub struct Lambertian {

    albedo: Vec3,
}

impl Lambertian {

    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian {
            albedo: albedo,
        }
    }
}

impl Material for Lambertian {

    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Vec3, Vec3)> {
        let scatter_direction = &(&record.p + &record.normal) + &random_in_unit_sphere();
        let scattered_ray = Ray::new(record.p, &target - &record.p);
        Some((self.center, scattered_ray))
    }

}
