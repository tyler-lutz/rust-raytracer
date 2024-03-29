use crate::{hittable::HitRecord, ray::Ray, vector3::Vector3};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vector3,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    pub albedo: Vector3,
}

impl Lambertian {
    pub fn new(albedo: Vector3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vector3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vector3::random_in_unit_sphere();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.point, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    pub albedo: Vector3,
}

impl Metal {
    pub fn new(albedo: Vector3) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vector3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = r_in.direction.normalized().reflect(rec.normal);
        *scattered = Ray::new(rec.point, reflected);
        *attenuation = self.albedo;
        true
    }
}
