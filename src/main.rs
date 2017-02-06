extern crate rand;

pub mod vec3;
pub mod ray;
pub mod sphere;
pub mod hitable;
pub mod camera;

use rand::distributions::{IndependentSample, Range};

use vec3::*;
use sphere::*;
use hitable::*;
use ray::*;
use camera::*;

use std::vec::Vec;
use std::boxed::Box;

fn random_in_unit_sphere_helper() -> Vec3 {

    &(Scalar(2.0) * &Vec3::new(randBetween0and1(), randBetween0and1(), randBetween0and1())) - &Vec3::new(1.0, 1.0, 1.0)
}

fn random_in_unit_sphere() -> Vec3 {

    let mut done = false;
    let mut p = random_in_unit_sphere_helper();
    while dot(&p, &p) >= 1.0 {
        p = random_in_unit_sphere_helper();
    }
    p
}

fn color(ray: &Ray, world: &Hitable) -> Vec3 {
    color_limited(ray, world, 0)
}

fn color_limited(ray: &Ray, world: &Hitable, depth: u32) -> Vec3 {

    let hitResult = if depth < 100 {
        world.hit(ray, 0.0, std::f32::MAX)
    } else {
        None
    };
    match hitResult {
        Some(record) => {
            let target = &(&record.p + &record.normal) + &random_in_unit_sphere();
            Scalar(0.5) * &color_limited(&Ray::new(record.p, &target - &record.p), world, depth+1) 
        },
        None => {
            // Background
            let t = 0.5 * (ray.direction.y + 1.0);
            let white = Vec3::new(1.0, 1.0, 1.0);
            let blue = Vec3::new(0.5, 0.7, 1.0);
            // Linear interpolation between white and blue
            &(Scalar(1.0 - t) * &white) + &(Scalar(t) * &blue)
        },
    }
}

fn randBetween0and1() -> f32 {

    let between = Range::new(0.0, 1.0);
    let mut rng = rand::thread_rng();

    between.ind_sample(&mut rng)
}

fn raytrace<'a, 'b, 'c, 'd>(
        world: &Hitable,
        nx: u32,
        ny: u32,
        ns: u32,
        camera: &Camera) {

    println!("P3\n{} {}\n255", nx, ny);
    for j in (0..ny).rev() {
        let j = j as f32; // Shadow j as f32
        let ny = ny as f32; // Shadow ny as f32

        for i in 0..nx {
            let i = i as f32; // Shadow i as f32
            let nx = nx as f32; // Shadow nx as f32

            // Multi sample anti aliasing, this time with iterators
            let colSum: Vec3 = (0..ns).fold(Vec3::new(0.0, 0.0, 0.0), |sum, elem| {
                let u = (i + randBetween0and1()) / nx;
                let v = (j + randBetween0and1()) / ny;
                let ray = camera.get_ray(u, v);
                let col = color(&ray, world); 
                &sum + &col
            });
            let colAvg = Scalar(1.0 / (ns as f32)) * &colSum;

            // Gamma correction.
            let col = Vec3::new(colAvg.x.sqrt(), colAvg.y.sqrt(), colAvg.z.sqrt());

            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn main() {

    let nx = 1000;
    let ny = 500;
    let ns = 20; // number of samples to take per pixel
    
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);

    let camera = Camera::new(origin, lower_left_corner, horizontal, vertical);

    let world: Vec<Box<Hitable>> = vec! [
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ];
    
    raytrace(&world, nx, ny, ns, &camera);
}
