use crate::{interval::Interval, ray::Ray, vector3::Vector3};

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub point: Vector3,
    pub normal: Vector3,
    pub t: f64,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            point: Vector3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            t: 0.0,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool;
}
