use std::{f64::consts::PI, sync::Arc};

pub mod texture;

use crate::{hittable::HitRecord, ray::Ray, rtweekend::random_double_unit, Color3, Point3, Vec3};

use texture::{SolidColor, Texture};

pub trait Material {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _albedo: &mut Color3,
        _scattered: &mut Ray,
        _pdf: &mut f64,
    ) -> bool {
        false
    }

    fn scattering_pdf(&self, _ray_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.
    }

    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color3 {
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
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        albedo: &mut Color3,
        scattered: &mut Ray,
        pdf: &mut f64,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        //let direction = Vec3::random_in_hemisphere(rec.normal);

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction.unit(), r_in.time());
        //*scattered = Ray::new(rec.p, direction.unit(), r_in.time());

        *albedo = self.albedo.value(rec.u, rec.v, &rec.p);
        *pdf = rec.normal * scattered.direction() / PI;
        //*pdf = 0.5 / PI;
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
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color3,
        scattered: &mut Ray,
        _pdf: &mut f64,
    ) -> bool {
        let reflected = Vec3::reflect(r_in.direction().unit(), rec.normal);
        *scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
            r_in.time(),
        );
        *attenuation = self.albedo;
        scattered.direction() * rec.normal > 0.
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
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color3,
        scattered: &mut Ray,
        _pdf: &mut f64,
    ) -> bool {
        *attenuation = Color3::new(1., 1., 1.);
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
        *scattered = Ray::new(rec.p, direction, r_in.time());
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
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color3,
        _scattered: &mut Ray,
        _pdf: &mut f64,
    ) -> bool {
        false
    }

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color3 {
        self.emit.value(u, v, p)
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
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color3,
        scattered: &mut Ray,
        _pdf: &mut f64,
    ) -> bool {
        *scattered = Ray::new(rec.p, Vec3::random_in_unit_sphere(), r_in.time());
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        true
    }
}
