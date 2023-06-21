mod color;
mod ray;
mod vec3;

use color::write_color;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::fs::File;
pub use vec3::Vec3;
pub type Point3 = Vec3;
pub type Color3 = Vec3;
use ray::Ray;

const AUTHOR: &str = "Xinwei Qiang";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn hit_sphere(center: Point3, radius: f64, r: Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().squared_length();
    let half_b = oc * r.direction();
    let c = oc.squared_length() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0. {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(r: Ray) -> Color3 {
    let t = hit_sphere(Point3::new(0., 0., -1.), 0.5, r);
    if t > 0.0 {
        let n = (r.at(t) - Vec3::new(0., 0., -1.)).unit();
        return 0.5 * Color3::new(n.x() + 1., n.y() + 1., n.z() + 1.);
    }
    let unit_direction = r.direction().unit();
    let t: f64 = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color3::ones() + t * Color3::new(0.5, 0.7, 1.0)
}

fn main() {
    // get environment variable CI, which is true for GitHub Actions
    let is_ci = is_ci();

    println!("CI: {}", is_ci);

    let aspect_ratio: f64 = 16.0 / 9.0;
    let width: usize = 800;
    let height: usize = (width as f64 / aspect_ratio) as usize;
    let path = "output/test.jpg";
    let quality = 60; // From 0 to 100, suggested value: 60

    // Create image data
    let mut img: RgbImage = ImageBuffer::new(width.try_into().unwrap(), height.try_into().unwrap());

    //Create camera
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = aspect_ratio * viewport_height;
    let focal_length: f64 = 1.0;

    let origin = Point3::zero();
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0., 0., focal_length);

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
            let u = i as f64 / (width - 1) as f64;
            let v = j as f64 / (height - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let color = ray_color(r);
            let pixel_color = [
                (color.x() * 255.).floor() as u8,
                (color.y() * 255.).floor() as u8,
                (color.z() * 255.).floor() as u8,
            ];
            write_color(pixel_color, &mut img, i, height - j - 1);
            bar.inc(1);
        }
    }

    // Finish progress bar
    bar.finish();

    // Output image to file
    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}
