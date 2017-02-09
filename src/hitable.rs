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
    fn hit<'a, 'b>(&'a self, ray: &'b Ray, tMin: f32, tMax: f32) -> Option<HitRecord<'a>>;
}

impl<'c> Hitable for Vec<Box<Hitable + 'c>> {

    fn hit<'a, 'b>(&'a self, ray: &'b Ray, tMin: f32, tMax: f32) -> Option<HitRecord> {

        let mut closestSoFar = tMax;
        let mut record: Option<HitRecord> = None;

        for hitableBox in self.iter() {
            match hitableBox.hit(ray, tMin, tMax) {
                Some(tempRecord) => {
                    // TODO: improve this code rather than blindly use what's in the book
                    let t = tempRecord.t;
                    if tMin <= t && t <= tMax {
                        if t < closestSoFar {
                            closestSoFar = t;
                            record = Some(tempRecord);
                        }
                    }
                },
                None => (),
            }
        }

        record
    }
}
