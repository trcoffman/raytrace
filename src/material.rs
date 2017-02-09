use vec3::*;
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

pub struct Metal {

    albedo: Vec3,
}

impl Metal {

    pub fn new(albedo: Vec3) -> Metal {
        Metal {
            albedo: albedo,
        }
    }

    fn reflect(&self, v: &Vec3, n: &Vec3) -> Vec3 {
        v - &(Scalar(2.0 * dot(v, n))*n)
    }

}

impl Material for Metal {

    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = self.reflect(&ray_in.direction.make_unit(), &record.normal.make_unit());
        let scattered_ray = Ray::new(record.p, reflected);
        if dot(&scattered_ray.direction, &record.normal) > 0.0 {
            Some((self.albedo, scattered_ray))
        } else {
            None
        }
    }

}
