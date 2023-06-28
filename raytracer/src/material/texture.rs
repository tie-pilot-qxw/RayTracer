use std::sync::Arc;

use crate::{rtweekend::clamp, Color3, Point3};
use image::{GenericImageView, Pixel, Rgb};

use self::perlin::Perlin;

mod perlin;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color3;
}

pub struct SolidColor {
    color_value: Color3,
}

impl SolidColor {
    pub fn new(color_value: Color3) -> Self {
        Self { color_value }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color3 {
        self.color_value
    }
}

pub struct CheckerTexture {
    odd: Arc<dyn Texture + Send + Sync>,
    even: Arc<dyn Texture + Send + Sync>,
}

impl CheckerTexture {
    #[allow(dead_code)]
    pub fn new(even: Arc<dyn Texture + Send + Sync>, odd: Arc<dyn Texture + Send + Sync>) -> Self {
        Self { odd, even }
    }

    pub fn new_solid(c1: Color3, c2: Color3) -> Self {
        Self {
            odd: (Arc::new(SolidColor::new(c1))),
            even: (Arc::new(SolidColor::new(c2))),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color3 {
        let sines = (10. * p.x()).sin() * (10. * p.y()).sin() * (10. * p.z()).sin();
        if sines < 0. {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: (Perlin::new()),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color3 {
        // Color3::ones() * 0.5 * (1.0 + self.noise.noise(&(*p * self.scale))) // perlin noise
        // Color3::ones() * self.noise.turb(&(*p * self.scale), 7) // perlin noise with turbulence
        Color3::ones() * 0.5 * (1. + (self.scale * p.z() + 10. * self.noise.turb(p, 7)).sin())
        // marble-like texture
    }
}

pub struct ImageTexture {
    width: usize,
    height: usize,
    data: Vec<Vec<Rgb<u8>>>,
}

impl ImageTexture {
    pub fn new(filename: &String) -> Self {
        let img = image::open(filename).unwrap();
        let (width, height) = img.dimensions();
        let mut data: Vec<Vec<Rgb<u8>>> = Vec::new();
        data.resize(width.try_into().unwrap(), Vec::new());
        for i in 0..width {
            data[i as usize].resize(height.try_into().unwrap(), Rgb([0_u8; 3]));
            for j in 0..height {
                data[i as usize][j as usize] = img.get_pixel(i, j).clone().to_rgb();
            }
        }

        Self {
            width: width as usize,
            height: height as usize,
            data,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color3 {
        let u = clamp(u, 0., 1.);
        let v = 1. - clamp(v, 0., 1.);

        let mut i = (u * self.width as f64) as usize;
        let mut j = (v * self.height as f64) as usize;

        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }

        let color_scale = 1. / 255.;
        Color3::new(
            self.data[i][j][0] as f64 * color_scale,
            self.data[i][j][1] as f64 * color_scale,
            self.data[i][j][2] as f64 * color_scale,
        )
    }
}

#[test]
fn test_image() {
    let filename: String = "./res/earthmap.jpg".to_string();
    let _a = ImageTexture::new(&filename);
}
