use std::vec::Vec;
use std::boxed::Box;

use vec3::Vec3;
use vec3::Scalar;
use ray::Ray;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

impl HitRecord {

    pub fn new(t: f32, p: Vec3, normal: Vec3) -> HitRecord {
        HitRecord {
            t: t,
            p: p,
            normal: normal,
        }
    }
}

pub trait Hitable {
    fn hit<'a, 'b>(&'a self, ray: &'b Ray, tMin: f32, tMax: f32) -> Option<HitRecord>;
}

impl Hitable for Vec<Box<Hitable>> {

    fn hit<'a, 'b>(&'a self, ray: &'b Ray, tMin: f32, tMax: f32) -> Option<HitRecord> {

        let mut closestSoFar = tMax;
        let mut hitAnything = false;
        let mut record: Option<HitRecord> = None;

        for hitableBox in self.iter() {
            match hitableBox.hit(ray, tMin, tMax) {
                Some(tempRecord) => {
                    // TODO: improve this code rather than blindly use what's in the book
                    let t = tempRecord.t;
                    if tMin <= t && t <= tMax {
                        hitAnything = true;
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
