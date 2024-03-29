mod camera;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod vector3;

use std::rc::Rc;

use camera::Camera;
use material::{Lambertian, Metal};
use vector3::Vector3;

use crate::{hittable_list::HittableList, sphere::Sphere};
fn main() {
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Vector3::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(Vector3::new(0.8, 0.8, 0.8)));
    let material_right = Rc::new(Metal::new(Vector3::new(0.8, 0.6, 0.2)));

    world.add(Box::new(Sphere::new(
        Vector3::new(0.0, -100.5, -1.0),
        100.0,
        Some(material_ground),
    )));
    world.add(Box::new(Sphere::new(
        Vector3::new(0.0, 0.0, -1.0),
        0.5,
        Some(material_center),
    )));
    world.add(Box::new(Sphere::new(
        Vector3::new(-1.0, 0.0, -1.0),
        0.5,
        Some(material_left),
    )));
    world.add(Box::new(Sphere::new(
        Vector3::new(1.0, 0.0, -1.0),
        0.5,
        Some(material_right),
    )));

    let mut camera = Camera::default();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.render(&world);
}
