mod vector3;

use vector3::Vector3;

fn write_color(pixel_color: &Vector3) {
    println!(
        "{} {} {}",
        (255.999 * pixel_color.x) as i32,
        (255.999 * pixel_color.y) as i32,
        (255.999 * pixel_color.z) as i32
    )
}

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {:3}", (image_height - j));
        for i in 0..image_width {
            let pixel_color = Vector3 {
                x: i as f64 / (image_width - 1) as f64,
                y: j as f64 / (image_height - 1) as f64,
                z: 0.0,
            };

            write_color(&pixel_color);
        }
    }

    eprintln!("\rDone!                     ")
}
