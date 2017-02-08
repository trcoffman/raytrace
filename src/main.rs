pub mod vec3;
pub mod ray;
pub mod sphere;
pub mod hitable;
pub mod camera;
pub mod random;

use vec3::*;
use sphere::*;
use hitable::*;
use ray::*;
use camera::*;
use random::*;

use std::vec::Vec;
use std::boxed::Box;

fn color(ray: &Ray, world: &Hitable) -> Vec3 {
    color_limited(ray, world, 0)
}

fn color_limited(ray: &Ray, world: &Hitable, depth: u32) -> Vec3 {

    let hitResult = if depth < 20 {
        world.hit(ray, 0.001, std::f32::MAX)
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

fn raytrace<'a, 'b, 'c, 'd>(
        world: &Hitable,
        nx: u32,
        ny: u32,
        ns: u32,
        camera: &Camera) {

    println!("P3\n{} {}\n255", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {

            // Multi sample anti aliasing, this time with iterators
            let colSum: Vec3 = (0..ns).fold(Vec3::new(0.0, 0.0, 0.0), |sum, elem| {
                let ray = camera.get_randomized_ray(i, nx, j, ny);
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
    let ns = 200; // number of samples to take per pixel
    
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
