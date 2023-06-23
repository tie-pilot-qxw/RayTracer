mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use color::write_color;
use hittable::{HitRecord, Hittable};
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::{fs::File, sync::Arc};
pub use vec3::Vec3;
pub type Point3 = Vec3;
pub type Color3 = Vec3;
use hittable_list::HittableList;
use ray::Ray;
use std::f64::INFINITY;

use crate::{
    camera::Camera,
    material::{Lambertian, Metal},
    rtweekend::random_double_unit,
    sphere::Sphere,
};

const AUTHOR: &str = "Xinwei Qiang";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn ray_color(r: Ray, world: &impl Hittable, depth: isize) -> Color3 {
    let mut rec = HitRecord::new();

    if depth <= 0 {
        return Color3::new(0., 0., 0.);
    }

    if world.hit(r, 0.000001, INFINITY, &mut rec) {
        let mut scattered = Ray::new(Vec3::zero(), Vec3::zero());
        let mut attenuation = Color3::zero();
        if rec
            .mat_ptr
            .clone()
            .unwrap()
            .scatter(&r, &rec, &mut attenuation, &mut scattered)
        {
            return Vec3::elemul(attenuation, ray_color(scattered, world, depth - 1));
        } else {
            return Color3::zero();
        }
    }

    let unit_direction = r.direction().unit();
    let t: f64 = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color3::ones() + t * Color3::new(0.5, 0.7, 1.0)
}

fn main() {
    // get environment variable CI, which is true for GitHub Actions
    let is_ci = is_ci();

    println!("CI: {}", is_ci);

    //Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let width: usize = 800;
    let height: usize = (width as f64 / aspect_ratio) as usize;
    let path = "output/test.jpg";
    let quality = 60; // From 0 to 100, suggested value: 60
    let samples_per_pixel: usize = 100;
    let max_depth = 50;

    // Create image data
    let mut img: RgbImage = ImageBuffer::new(width.try_into().unwrap(), height.try_into().unwrap());

    //Create camera
    let cam = Camera::new();

    //World
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(Color3::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color3::new(0.7, 0.3, 0.3)));
    let material_left = Arc::new(Metal::new(Color3::new(0.8, 0.8, 0.8)));
    let material_right = Arc::new(Metal::new(Color3::new(0.8, 0.6, 0.2)));

    world.add(Box::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        material_right,
    )));

    // Progress bar UI powered by library `indicatif`
    // You can use indicatif::ProgressStyle to make it more beautiful
    // You can also use indicatif::MultiProgress in multi-threading to show progress of each thread
    let bar = if is_ci {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    for j in 0..height {
        for i in 0..width {
            let mut color = Color3::zero();
            for _s in 0..samples_per_pixel {
                let u = (i as f64 + random_double_unit()) / (width - 1) as f64;
                let v = (j as f64 + random_double_unit()) / (height - 1) as f64;
                let r = cam.get_ray(u, v);
                color += ray_color(r, &world, max_depth);
            }
            write_color(color, samples_per_pixel, &mut img, i, height - j - 1);
            bar.inc(1);
        }
    }

    // Finish progress bar
    bar.finish();

    world.clear();

    // Output image to file
    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}
