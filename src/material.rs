use vec3::Vec3;
use ray::Ray;
use random::*;
use hitable::HitRecord;

pub trait Material: Send + Sync {

    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)>;

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

    fn scatter(&self, _: &Ray, record: &HitRecord) -> Option<(Vec3, Ray)> {
        let scatter_direction = &(&record.p + &record.normal) + &random_in_unit_sphere();
        let scattered_ray = Ray::new(record.p, &scatter_direction - &record.p);
        Some((self.albedo, scattered_ray))
    }

}
