use crate::{rtweekend::clamp, Color3};
use image::RgbImage;

/// the multi-sample write_color() function
pub fn write_color(
    mut color: Color3,
    samples_per_pixel: usize,
    img: &mut RgbImage,
    i: usize,
    j: usize,
) {
    // Clear NaNs
    for i in 0..2 {
        if color[i].is_nan() || color[i].is_infinite() {
            color[i] = 0.;
        }
    }

    //Divide the color by the number of samples.
    let scale = 1. / samples_per_pixel as f64;
    color *= scale;
    let r = color.x().powf(0.5);
    let g = color.y().powf(0.5);
    let b = color.z().powf(0.5);

    let pixel_color = [
        (clamp(r, 0., 0.999) * 256.).floor() as u8,
        (clamp(g, 0., 0.999) * 255.).floor() as u8,
        (clamp(b, 0., 0.999) * 255.).floor() as u8,
    ];
    let pixel = img.get_pixel_mut(i.try_into().unwrap(), j.try_into().unwrap());
    *pixel = image::Rgb(pixel_color);
    // Write the translated [0,255] value of each color component.
}
