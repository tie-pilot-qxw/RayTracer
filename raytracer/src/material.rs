use std::{f64::consts::PI, sync::Arc};

pub mod texture;

use crate::{
    hittable::HitRecord,
    pdf::{CosinePdf, Pdf},
    ray::Ray,
    rtweekend::random_double_unit,
    Color3, Point3, Vec3,
};

use texture::{SolidColor, Texture};

pub struct ScatterRecord {
    pub specular_ray: Ray,
    pub is_specular: bool,
    pub attenuation: Color3,
    pub pdf_ptr: Option<Arc<dyn Pdf + Send + Sync>>,
}

impl ScatterRecord {
    pub fn new() -> Self {
        Self {
            specular_ray: (Ray::new(Vec3::zero(), Vec3::zero(), 0.)),
            is_specular: (false),
            attenuation: (Color3::zero()),
            pdf_ptr: (None),
        }
    }
}
pub trait Material {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, _srec: &mut ScatterRecord) -> bool {
        false
    }

    fn scattering_pdf(&self, _ray_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.
    }

    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: &Point3) -> Color3 {
        Color3::zero()
    }
}

pub struct Lambertian {
    albedo: Arc<dyn Texture + Send + Sync>,
}

impl Lambertian {
    pub fn new(albedo: Color3) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(albedo)),
        }
    }

    pub fn new_texture(albedo: Arc<dyn Texture + Send + Sync>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.is_specular = false;
        srec.attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        srec.pdf_ptr = Some(Arc::new(CosinePdf::new(&rec.normal)));
        true
    }

    fn scattering_pdf(&self, _ray_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = rec.normal * scattered.direction().unit();
        if cosine < 0. {
            0.
        } else {
            cosine / PI
        }
    }
}

pub struct Metal {
    albedo: Color3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color3, f: f64) -> Self {
        Self {
            albedo,
            fuzz: f.min(1.),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        let reflected = Vec3::reflect(r_in.direction().unit(), rec.normal);
        srec.specular_ray = Ray::new(
            rec.p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
            r_in.time(),
        );
        srec.attenuation = self.albedo;
        srec.is_specular = true;
        srec.pdf_ptr = None;
        true
    }
}

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1. - ref_idx) / (1. + ref_idx);
        r0 = r0 * r0;
        r0 + (1. - r0) * (1. - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.is_specular = true;
        srec.pdf_ptr = None;
        srec.attenuation = Color3::ones();

        let refraction_ratio = if rec.front_face {
            1. / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction().unit();
        let cos_theta = (1.0_f64).min((-unit_direction) * rec.normal);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.;
        let direction: Vec3 = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > random_double_unit()
        {
            Vec3::reflect(unit_direction, rec.normal)
        } else {
            Vec3::refract(unit_direction, rec.normal, refraction_ratio)
        };
        srec.specular_ray = Ray::new(rec.p, direction, r_in.time());
        true
    }
}

pub struct DiffuseLight {
    emit: Arc<dyn Texture + Send + Sync>,
}

impl DiffuseLight {
    #[allow(dead_code)]
    pub fn new(emit: Arc<dyn Texture + Send + Sync>) -> Self {
        Self { emit }
    }

    pub fn new_color(c: Color3) -> Self {
        Self {
            emit: Arc::new(SolidColor::new(c)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, _srec: &mut ScatterRecord) -> bool {
        false
    }

    fn emitted(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color3 {
        if rec.front_face {
            self.emit.value(u, v, p)
        } else {
            Color3::zero()
        }
    }
}

pub struct Isotropic {
    albedo: Arc<dyn Texture + Send + Sync>,
}

impl Isotropic {
    pub fn new_color(c: Color3) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(c)),
        }
    }

    pub fn new_texture(albedo: Arc<dyn Texture + Send + Sync>) -> Self {
        Self { albedo }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.is_specular = true;
        srec.pdf_ptr = None;
        srec.specular_ray = Ray::new(rec.p, Vec3::random_in_unit_sphere(), r_in.time());
        srec.attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        true
    }
}
