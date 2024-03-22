use rand::Rng;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    vector3::Vector3,
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    image_height: i32,
    center: Vector3,
    pixel00_loc: Vector3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
}

impl Camera {
    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        if self.image_height < 1 {
            self.image_height = 1;
        }

        self.center = Vector3::new(0.0, 0.0, 0.0);

        // Determine viewport dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left = self.center
            - Vector3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {:3}", (self.image_height - j));
            for i in 0..self.image_width {
                let mut pixel_color = Vector3::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color = pixel_color + self.ray_color(&ray, self.max_depth, world)
                }

                self.write_color(&pixel_color);
            }
        }

        eprintln!("\rDone!                     ")
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center =
            self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vector3 {
        let mut rng = rand::thread_rng();
        let px = -0.5 + rng.gen::<f64>();
        let py = -0.5 + rng.gen::<f64>();
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    fn write_color(&self, pixel_color: &Vector3) {
        let mut r = pixel_color.x;
        let mut g = pixel_color.y;
        let mut b = pixel_color.z;

        let scale = 1.0 / self.samples_per_pixel as f64;
        r *= scale;
        g *= scale;
        b *= scale;

        let intensity = Interval::new(0.000, 0.999);

        println!(
            "{} {} {}",
            (255.999 * intensity.clamp(r)) as i32,
            (255.999 * intensity.clamp(g)) as i32,
            (255.999 * intensity.clamp(b)) as i32
        )
    }

    fn ray_color(&self, ray: &Ray, depth: i32, world: &dyn Hittable) -> Vector3 {
        if depth < 0 {
            return Vector3::new(0.0, 0.0, 0.0);
        }

        let mut hit_record = HitRecord {
            point: Vector3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            t: 0.0,
        };

        if world.hit(ray, Interval::new(0.001, f64::INFINITY), &mut hit_record) {
            let direction = Vector3::random_on_hemisphere(hit_record.normal);
            return 0.5 * self.ray_color(&Ray::new(hit_record.point, direction), depth - 1, world);
        }

        let unit_direction = ray.direction.normalized();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Vector3::new(1.0, 1.0, 1.0) + a * Vector3::new(0.5, 0.7, 1.0)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            image_height: 0,
            center: Vector3::new(0.0, 0.0, 0.0),
            pixel00_loc: Vector3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vector3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}
