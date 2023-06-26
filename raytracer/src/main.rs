mod aabb;
mod bvh;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod moving_sphere;
mod ray;
mod rtweekend;
mod sphere;
mod texture;
mod vec3;

use color::write_color;
use hittable::{HitRecord, Hittable};
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use moving_sphere::MovingSphere;
use rtweekend::random_double;
use std::{fs::File, sync::Arc};
use texture::{CheckerTexture, NoiseTexture};
pub use vec3::Vec3;
pub type Point3 = Vec3;
pub type Color3 = Vec3;
use bvh::BVH;
use hittable_list::HittableList;
use ray::Ray;
use std::f64::INFINITY;

use crate::{
    camera::Camera,
    material::{Dielectric, Lambertian, Metal},
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

    if world.hit(&r, 0.000001, INFINITY, &mut rec) {
        let mut scattered = Ray::new(Vec3::zero(), Vec3::zero(), 0.);
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

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::new_solid(
        Color3::new(0.2, 0.3, 0.1),
        Color3::new(0.9, 0.9, 0.9),
    ));
    let ground_material = Arc::new(Lambertian::new_texture(checker));
    world.add(Arc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double_unit();
            let center = Point3::new(
                a as f64 + 0.9 * random_double_unit(),
                0.2,
                b as f64 + 0.9 * random_double_unit(),
            );

            if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color3::elemul(Color3::random_unit(), Color3::random_unit());
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let center2 = center + Vec3::new(0., random_double(0., 0.5), 0.);
                    world.add(Arc::new(MovingSphere::new(
                        center,
                        center2,
                        0.,
                        1.,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color3::random(0.5, 1.);
                    let fuzz = random_double(0., 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }
    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0., 1., 0.),
        1.,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color3::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4., 1., 0.),
        1.,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4., 1., 0.),
        1.,
        material3,
    )));

    world
}

fn two_spheres() -> HittableList {
    let mut objects = HittableList::new();

    let checker = Arc::new(CheckerTexture::new_solid(
        Color3::new(0.2, 0.3, 0.1),
        Color3::new(0.9, 0.9, 0.9),
    ));

    objects.add(Arc::new(Sphere::new(
        Point3::new(0., -10., 0.),
        10.,
        Arc::new(Lambertian::new_texture(checker.clone())),
    )));

    objects.add(Arc::new(Sphere::new(
        Point3::new(0., 10., 0.),
        10.,
        Arc::new(Lambertian::new_texture(checker)),
    )));

    objects
}

fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new());

    objects.add(Arc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        Arc::new(Lambertian::new_texture(pertext.clone())),
    )));

    objects.add(Arc::new(Sphere::new(
        Point3::new(0., 2., 0.),
        2.,
        Arc::new(Lambertian::new_texture(pertext)),
    )));

    objects
}

fn main() {
    // get environment variable CI, which is true for GitHub Actions
    let is_ci = is_ci();

    println!("CI: {}", is_ci);

    // Image
    let aspect_ratio: f64 = 16. / 9.;
    let width: usize = 400;
    let height: usize = (width as f64 / aspect_ratio) as usize;
    let path = "output/test.jpg";
    let quality = 60; // From 0 to 100, suggested value: 60
    let samples_per_pixel: usize = 100;
    let max_depth = 50;

    // Create image data
    let mut img: RgbImage = ImageBuffer::new(width.try_into().unwrap(), height.try_into().unwrap());

    // World
    let world: BVH;

    let mut lookfrom = Point3::ones();
    let mut lookat = Point3::zero();
    let mut vfov = 40.;
    let mut aperture = 0.;

    match 3 {
        1 => {
            world = BVH::new(&random_scene(), 0., 1.);
            lookfrom = Point3::new(13., 2., 3.);
            lookat = Point3::new(0., 0., 0.);
            vfov = 20.0;
            aperture = 0.1;
        }
        2 => {
            world = BVH::new(&two_spheres(), 0., 0.);
            lookfrom = Point3::new(13., 2., 3.);
            lookat = Point3::new(0., 0., 0.);
            vfov = 20.0;
        }
        3 => {
            world = BVH::new(&&two_perlin_spheres(), 0., 0.);
            lookfrom = Point3::new(13., 2., 3.);
            lookat = Point3::new(0., 0., 0.);
            vfov = 20.0;
        }
        _ => {
            world = BVH::new(&HittableList::new(), 0., 0.);
        }
    }

    // Camera

    let vup = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.,
        1.,
    );

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

    //world.clear();

    // Output image to file
    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}
