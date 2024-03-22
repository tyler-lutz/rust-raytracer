mod camera;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod vector3;

use camera::Camera;
use vector3::Vector3;

use crate::{hittable_list::HittableList, sphere::Sphere};
fn main() {
    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(
        Vector3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    let mut camera = Camera::default();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;

    camera.render(&world);
}
