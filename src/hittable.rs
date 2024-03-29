use std::rc::Rc;

use crate::{interval::Interval, material::Material, ray::Ray, vector3::Vector3};

#[derive(Clone)]
pub struct HitRecord {
    pub point: Vector3,
    pub normal: Vector3,
    pub front_face: bool,
    pub t: f64,
    pub mat: Option<Rc<dyn Material>>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }

    pub fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vector3,
        scattered: &mut Ray,
    ) -> bool {
        if let Some(material) = &self.mat {
            material.scatter(r_in, self, attenuation, scattered)
        } else {
            false
        }
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            point: Vector3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            front_face: true,
            t: 0.0,
            mat: None,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool;
}
