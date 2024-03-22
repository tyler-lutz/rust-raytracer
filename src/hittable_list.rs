use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            let mut temp_hit_record = HitRecord::default();
            if object.hit(
                &ray,
                Interval {
                    min: ray_t.min,
                    max: closest_so_far,
                },
                &mut temp_hit_record,
            ) {
                hit_anything = true;
                closest_so_far = temp_hit_record.t;
                *hit_record = temp_hit_record;
            }
        }

        hit_anything
    }
}
