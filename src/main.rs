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


fn color(ray: &Ray, world: &Hitable) -> Vec3 {

    match world.hit(ray, 0.0, std::f32::MAX) {
        Some(record) => Scalar(0.5) * &(Scalar(1.0) + &record.normal),
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

            // Multi sample anti aliasing
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for s in (0..ns) {
                let u = (i + randBetween0and1()) / nx;
                let v = (j + randBetween0and1()) / ny;
                let ray = camera.get_ray(u, v);
                col = &col + &color(&ray, world); 
            }
            col = Scalar(1.0 / (ns as f32)) * &col;

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