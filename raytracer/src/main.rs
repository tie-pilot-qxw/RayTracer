mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod rtweekend;
mod vec3;

use color::write_color;
use hittable::{
    aarect::{XyRect, XzRect, YzRect},
    bvh,
    constant_medium::ConstantMedium,
    hittable_list, moving_sphere, HitRecord, Hittable, RotateY, Translate,
};
use image::{ImageBuffer, RgbImage};
use indicatif::{MultiProgress, ProgressBar};
use material::{
    texture::{CheckerTexture, ImageTexture, NoiseTexture},
    DiffuseLight,
};
use moving_sphere::MovingSphere;
use rtweekend::random_double;
use std::{
    fs::File,
    sync::{mpsc, Arc},
    thread,
};
pub use vec3::Vec3;
pub type Point3 = Vec3;
pub type Color3 = Vec3;
use bvh::BVH;
use hittable::boxes::Boxes;
use hittable_list::HittableList;
use ray::Ray;
use std::f64::INFINITY;

use crate::{
    camera::Camera,
    hittable::sphere::Sphere,
    material::{Dielectric, Lambertian, Metal},
    rtweekend::random_double_unit,
};

const AUTHOR: &str = "Xinwei Qiang";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn ray_color(
    r: Ray,
    world: Arc<dyn Hittable + Send + Sync>,
    background: Color3,
    depth: isize,
) -> Color3 {
    let mut rec = HitRecord::new();

    if depth <= 0 {
        return Color3::new(0., 0., 0.);
    }

    if !world.hit(&r, 0.000001, INFINITY, &mut rec) {
        return background;
    }

    let mut scattered = Ray::new(Vec3::zero(), Vec3::zero(), 0.);
    let mut attenuation = Color3::zero();
    let emitted = rec.mat_ptr.clone().unwrap().emitted(rec.u, rec.v, &rec.p);

    if rec
        .mat_ptr
        .clone()
        .unwrap()
        .scatter(&r, &rec, &mut attenuation, &mut scattered)
    {
        emitted
            + Vec3::elemul(
                attenuation,
                ray_color(scattered, world, background, depth - 1),
            )
    } else {
        emitted
    }
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

    let pertext = Arc::new(NoiseTexture::new(4.));

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

fn earth() -> HittableList {
    let mut objects = HittableList::new();
    let earth_texture = Arc::new(ImageTexture::new(&"raytracer/res/earthmap.jpg".to_string()));
    let earth_surface = Arc::new(Lambertian::new_texture(earth_texture));
    let globe = Arc::new(Sphere::new(Point3::new(0., 0., 0.), 2., earth_surface));
    objects.add(globe);
    objects
}

fn simple_light() -> HittableList {
    let mut objects = HittableList::new();

    let pretext = Arc::new(NoiseTexture::new(4.));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        Arc::new(Lambertian::new_texture(pretext.clone())),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0., 2., 0.),
        2.,
        Arc::new(Lambertian::new_texture(pretext)),
    )));

    let difflight = Arc::new(DiffuseLight::new_color(Color3::ones() * 4.));
    objects.add(Arc::new(XyRect::new(
        3.,
        5.,
        1.,
        3.,
        -2.,
        difflight.clone(),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0., 7., 0.),
        2.,
        difflight,
    )));
    objects
}

fn cornell_box() -> HittableList {
    let mut objects = HittableList::new();

    let red = Arc::new(Lambertian::new(Color3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_color(Color3::new(15., 15., 15.)));

    objects.add(Arc::new(YzRect::new(0., 555., 0., 555., 555., green)));
    objects.add(Arc::new(YzRect::new(0., 555., 0., 555., 0., red)));
    objects.add(Arc::new(XzRect::new(213., 343., 227., 332., 554., light)));
    objects.add(Arc::new(XzRect::new(0., 555., 0., 555., 0., white.clone())));
    objects.add(Arc::new(XzRect::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));
    objects.add(Arc::new(XyRect::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));

    let mut box1: Arc<dyn Hittable + Send + Sync> = Arc::new(Boxes::new(
        &Point3::zero(),
        &Point3::new(165., 330., 165.),
        white.clone(),
    ));
    box1 = Arc::new(RotateY::new(box1, 15.));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265., 0., 295.)));
    objects.add(box1);

    let mut box2: Arc<dyn Hittable + Send + Sync> = Arc::new(Boxes::new(
        &Point3::zero(),
        &Point3::new(165., 165., 165.),
        white,
    ));
    box2 = Arc::new(RotateY::new(box2, -18.));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130., 0., 65.)));
    objects.add(box2);

    objects
}

fn cornell_smoke() -> HittableList {
    let mut objects = HittableList::new();

    let red = Arc::new(Lambertian::new(Color3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_color(Color3::new(15., 15., 15.)));

    objects.add(Arc::new(YzRect::new(0., 555., 0., 555., 555., green)));
    objects.add(Arc::new(YzRect::new(0., 555., 0., 555., 0., red)));
    objects.add(Arc::new(XzRect::new(213., 343., 227., 332., 554., light)));
    objects.add(Arc::new(XzRect::new(0., 555., 0., 555., 0., white.clone())));
    objects.add(Arc::new(XzRect::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));
    objects.add(Arc::new(XyRect::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));

    let mut box1: Arc<dyn Hittable + Send + Sync> = Arc::new(Boxes::new(
        &Point3::zero(),
        &Point3::new(165., 330., 165.),
        white.clone(),
    ));
    box1 = Arc::new(RotateY::new(box1, 15.));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265., 0., 295.)));

    let mut box2: Arc<dyn Hittable + Send + Sync> = Arc::new(Boxes::new(
        &Point3::zero(),
        &Point3::new(165., 165., 165.),
        white,
    ));
    box2 = Arc::new(RotateY::new(box2, -18.));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130., 0., 65.)));

    objects.add(Arc::new(ConstantMedium::new_color(
        box1,
        0.01,
        Color3::zero(),
    )));
    objects.add(Arc::new(ConstantMedium::new_color(
        box2,
        0.01,
        Color3::ones(),
    )));

    objects
}

fn final_scene() -> HittableList {
    let mut boxes1 = HittableList::new();
    let ground = Arc::new(Lambertian::new(Color3::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.;
            let x0 = -1000. + i as f64 * w;
            let z0 = -1000. + j as f64 * w;
            let y0 = 0.;
            let x1 = x0 + w;
            let y1 = random_double(1., 101.);
            let z1 = z0 + w;

            boxes1.add(Arc::new(Boxes::new(
                &Point3::new(x0, y0, z0),
                &Point3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut objects = HittableList::new();

    objects.add(Arc::new(BVH::new(&boxes1, 0., 1.)));

    let light = Arc::new(DiffuseLight::new_color(Color3::ones() * 7.));
    objects.add(Arc::new(XzRect::new(123., 423., 147., 412., 554., light)));

    let center1 = Point3::new(400., 400., 200.);
    let center2 = center1 + Vec3::new(30., 0., 0.);
    let moving_sphere_material = Arc::new(Lambertian::new(Color3::new(0.7, 0.3, 0.1)));
    objects.add(Arc::new(MovingSphere::new(
        center1,
        center2,
        0.,
        1.,
        50.,
        moving_sphere_material,
    )));

    objects.add(Arc::new(Sphere::new(
        Point3::new(260., 150., 45.),
        50.,
        Arc::new(Dielectric::new(1.5)),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0., 150., 145.),
        50.,
        Arc::new(Metal::new(Color3::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let mut boundary = Arc::new(Sphere::new(
        Point3::new(360., 150., 145.),
        70.,
        Arc::new(Dielectric::new(1.5)),
    ));
    objects.add(boundary.clone());
    objects.add(Arc::new(ConstantMedium::new_color(
        boundary,
        0.2,
        Color3::new(0.2, 0.4, 0.9),
    )));
    boundary = Arc::new(Sphere::new(
        Point3::zero(),
        5000.,
        Arc::new(Dielectric::new(1.5)),
    ));
    objects.add(Arc::new(ConstantMedium::new_color(
        boundary,
        0.0001,
        Color3::ones(),
    )));

    let emat = Arc::new(Lambertian::new_texture(Arc::new(ImageTexture::new(
        &"raytracer/res/earthmap.jpg".to_string(),
    ))));
    objects.add(Arc::new(Sphere::new(
        Point3::new(400., 200., 400.),
        100.,
        emat,
    )));

    let pertext = Arc::new(NoiseTexture::new(0.1));
    objects.add(Arc::new(Sphere::new(
        Point3::new(220., 280., 300.),
        80.,
        Arc::new(Lambertian::new_texture(pertext)),
    )));

    let mut boxes2 = HittableList::new();
    let white = Arc::new(Lambertian::new(Color3::ones() * 0.73));
    let ns = 1000;
    for _j in 0..ns {
        boxes2.add(Arc::new(Sphere::new(
            Point3::random(0., 165.),
            10.,
            white.clone(),
        )));
    }

    objects.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(Arc::new(BVH::new(&boxes2, 0.0, 1.0)), 15.)),
        Vec3::new(-100., 270., 395.),
    )));

    objects
}

fn main() {
    const THREAD_NUM: usize = 5;

    // get environment variable CI, which is true for GitHub Actions
    let is_ci = is_ci();

    println!("CI: {}", is_ci);

    // Image
    let mut aspect_ratio: f64 = 16. / 9.;
    let mut width: usize = 400;
    let path = "output/test.jpg";
    let quality = 60; // From 0 to 100, suggested value: 60
    let mut samples_per_pixel: usize = 100;
    let max_depth = 50;

    // World
    let world: BVH;

    let mut lookfrom = Point3::ones();
    let mut lookat = Point3::zero();
    let mut vfov = 40.;
    let mut aperture = 0.;
    let mut background = Color3::zero();

    match 1 {
        1 => {
            world = BVH::new(&random_scene(), 0., 1.);
            lookfrom = Point3::new(13., 2., 3.);
            lookat = Point3::new(0., 0., 0.);
            vfov = 20.0;
            aperture = 0.1;
            background = Color3::new(0.7, 0.8, 1.);
        }
        2 => {
            world = BVH::new(&two_spheres(), 0., 0.);
            lookfrom = Point3::new(13., 2., 3.);
            lookat = Point3::new(0., 0., 0.);
            vfov = 20.0;
            background = Color3::new(0.7, 0.8, 1.);
        }
        3 => {
            world = BVH::new(&two_perlin_spheres(), 0., 0.);
            lookfrom = Point3::new(13., 2., 3.);
            lookat = Point3::new(0., 0., 0.);
            vfov = 20.0;
            background = Color3::new(0.7, 0.8, 1.);
        }
        4 => {
            world = BVH::new(&earth(), 0., 0.);
            lookfrom = Point3::new(13., 2., 3.);
            lookat = Point3::new(0., 0., 0.);
            vfov = 20.0;
            background = Color3::new(0.7, 0.8, 1.);
        }
        5 => {
            world = BVH::new(&simple_light(), 0., 0.);
            samples_per_pixel = 400;
            background = Color3::zero();
            lookfrom = Point3::new(26., 3., 6.);
            lookat = Point3::new(0., 2., 0.);
            vfov = 20.0
        }
        6 => {
            world = BVH::new(&cornell_box(), 0., 0.);
            aspect_ratio = 1.;
            width = 600;
            samples_per_pixel = 200;
            background = Color3::zero();
            lookfrom = Point3::new(278., 278., -800.);
            lookat = Point3::new(278., 278., 0.);
            vfov = 40.;
        }
        7 => {
            world = BVH::new(&cornell_smoke(), 0., 0.);
            aspect_ratio = 1.;
            width = 600;
            samples_per_pixel = 200;
            lookfrom = Point3::new(278., 278., -800.);
            lookat = Point3::new(278., 278., 0.);
            vfov = 40.;
        }
        8 => {
            world = BVH::new(&final_scene(), 0., 1.);
            aspect_ratio = 1.;
            width = 800;
            samples_per_pixel = 10000;
            background = Color3::zero();
            lookfrom = Point3::new(478., 278., -600.);
            lookat = Point3::new(278., 278., 0.);
            vfov = 40.;
        }
        _ => {
            world = BVH::new(&HittableList::new(), 0., 0.);
        }
    }
    let height: usize = (width as f64 / aspect_ratio) as usize;

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

    samples_per_pixel = (samples_per_pixel / THREAD_NUM + 1) * THREAD_NUM;
    let world = Arc::new(world);

    // Create image data
    let mut img: RgbImage = ImageBuffer::new(width.try_into().unwrap(), height.try_into().unwrap());

    // Progress bar UI powered by library `indicatif`
    // You can use indicatif::ProgressStyle to make it more beautiful
    // You can also use indicatif::MultiProgress in multi-threading to show progress of each thread
    let bar_collection = MultiProgress::new();

    let mut ans = Vec::new();
    ans.resize(width, Vec::new());
    for iter in &mut ans {
        iter.resize(height, Color3::zero());
    }

    let (tx, rx) = mpsc::channel();
    for _k in 0..THREAD_NUM {
        let camm = cam.clone();
        let world_t = world.clone();
        let tx_k = tx.clone();
        let mut ans_t = ans.clone();
        let bar = bar_collection.add(if is_ci {
            ProgressBar::hidden()
        } else {
            ProgressBar::new((height * width) as u64)
        });
        thread::spawn(move || {
            for (i, iter) in ans_t.iter_mut().enumerate().take(width) {
                for (j, jter) in iter.iter_mut().enumerate().take(height) {
                    for _s in 0..samples_per_pixel / THREAD_NUM {
                        let u = (i as f64 + random_double_unit()) / (width - 1) as f64;
                        let v = (j as f64 + random_double_unit()) / (height - 1) as f64;
                        let r = camm.get_ray(u, v);
                        *jter += ray_color(r, world_t.clone(), background, max_depth);
                    }
                    bar.inc(1);
                }
            }
            // Finish progress bar
            bar.finish();
            tx_k.send(ans_t).unwrap();
        });
    }
    drop(tx);
    for received in rx {
        for i in 0..width {
            for j in 0..height {
                ans[i][j] += received[i][j];
            }
        }
    }

    for (i, iter) in ans.iter().enumerate().take(width) {
        for (j, jter) in iter.iter().enumerate().take(height) {
            write_color(*jter, samples_per_pixel, &mut img, i, height - j - 1);
        }
    }

    // Output image to file
    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}
