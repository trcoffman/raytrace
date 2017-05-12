use std::vec::Vec;
use std::boxed::Box;

use vec3::Vec3;
use ray::Ray;
use material::Material;

pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f32, p: Vec3, normal: Vec3, material: &'a Material) -> HitRecord<'a> {
        HitRecord {
            t: t,
            p: p,
            normal: normal,
            material: material,
        }
    }
}

pub trait Hitable: Send + Sync {
    fn hit<'a, 'b>(&'a self, ray: &'b Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'a>>;
}

impl<'c> Hitable for Vec<Box<Hitable + 'c>> {
    fn hit<'a, 'b>(&'a self, ray: &'b Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {

        let mut closest_so_far = t_max;
        let mut record: Option<HitRecord> = None;

        for hitable_box in self.iter() {
            match hitable_box.hit(ray, t_min, t_max) {
                Some(temp_record) => {
                    // TODO: improve this code rather than blindly use what's in the book
                    let t = temp_record.t;
                    if t_min <= t && t <= t_max {
                        if t < closest_so_far {
                            closest_so_far = t;
                            record = Some(temp_record);
                        }
                    }
                }
                None => (),
            }
        }

        record
    }
}
