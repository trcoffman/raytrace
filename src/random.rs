extern crate rand;

use self::rand::distributions::{IndependentSample, Range};

use vec3::*;

pub fn rand_between0and1() -> f32 {
    let between = Range::new(0.0, 1.0);
    let mut rng = rand::thread_rng();

    between.ind_sample(&mut rng)
}

pub fn random_in_unit_sphere_helper() -> Vec3 {
    &(Scalar(2.0)
        * &Vec3::new(
            rand_between0and1(),
            rand_between0and1(),
            rand_between0and1(),
        )) - &Vec3::new(1.0, 1.0, 1.0)
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut p = random_in_unit_sphere_helper();
    while dot(&p, &p) >= 1.0 {
        p = random_in_unit_sphere_helper();
    }
    p
}
