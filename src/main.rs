extern crate rayon;

pub mod vec3;
pub mod ray;
pub mod sphere;
pub mod hitable;
pub mod camera;
pub mod random;
pub mod material;

use vec3::*;
use sphere::*;
use hitable::*;
use ray::*;
use camera::*;
use random::*;
use material::*;

use rayon::prelude::*;

use std::vec::Vec;
use std::boxed::Box;

fn color(ray: &Ray, world: &Hitable) -> Vec3 {
    color_limited(ray, world, 0)
}

fn color_limited(ray: &Ray, world: &Hitable, depth: u32) -> Vec3 {

    if depth > 50 {
        return Vec3::new(0.0, 0.0, 0.0)
    }
    match world.hit(ray, 0.001, std::f32::MAX) {
        Some(record) => {
            // TODO: could this be refactored to be part of the hit record? just an option with attenuation
            // and a ray?
            match record.material.scatter(ray, &record) {
                Some((attenuation, target)) => &attenuation * &color_limited(&target, world, depth+1),
                None => Vec3::new(0.0, 0.0, 0.0)
            }
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

fn raytrace(
        world: &Hitable,
        nx: u32,
        ny: u32,
        ns: u32,
        camera: &Camera) {

    println!("P3\n{} {}\n255", nx, ny);
    let rows: Vec<Vec<(i32, i32, i32)>> = (0..ny).into_par_iter().weight_max().map(|j| {
        (0..nx).map(|i| -> (i32, i32, i32) {

            // Multi sample anti aliasing, this time with iterators
            let colSum: Vec3 = (0..ns).fold(Vec3::new(0.0, 0.0, 0.0), |sum, _| {
                let ray = camera.get_randomized_ray(i, nx, j, ny);
                let col = color(&ray, world); 
                &sum + &col
            });
            let colAvg = Scalar(1.0 / (ns as f32)) * &colSum;

            // Gamma correction.
            let col = Vec3::new(colAvg.x.sqrt(), colAvg.y.sqrt(), colAvg.z.sqrt());

            ((255.99 * col.x) as i32, (255.99 * col.y) as i32, (255.99 * col.z) as i32)
        }).collect()
    }).collect();
    for row in rows.iter().rev() {
        for pixel in row.iter() {
            println!("{} {} {}", pixel.0, pixel.1, pixel.2);
        }
    }

}

fn main() {

    let nx = 1000;
    let ny = 500;
    let ns = 200; // number of samples to take per pixel
    
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);

    let camera = Camera::new(origin, lower_left_corner, horizontal, vertical);

    let pink = Lambertian::new(Vec3::new(0.8, 0.3, 0.3));
    let yellow_green = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
    let shiny_metal = Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.1);
    let dull_metal = Metal::new(Vec3::new(0.8, 0.8, 0.8), 1.0);
    let world: Vec<Box<Hitable>> = vec! [
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, &yellow_green)),
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, &pink)),
        Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, &shiny_metal)),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, &dull_metal)),
    ];
    
    raytrace(&world, nx, ny, ns, &camera);
}
