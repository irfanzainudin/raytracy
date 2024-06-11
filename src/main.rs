use std::fs::{File, OpenOptions};
use std::io::prelude::*;

use vec3::mul_scalar;
mod helloworld;
mod vec3;
mod ray;

fn ray_color(r: &ray::Ray) -> vec3::Colour {
    let unit_direction: vec3::Vec3 = vec3::unit_vector(&r.direction(), &r.direction().length());
    let a = 0.5 * (unit_direction.y() + 1.0);
    return vec3::mul_scalar(&vec3::Colour::create(1.0, 1.0, 1.0), &(1.0 - a)) + vec3::mul_scalar(&vec3::Colour::create(0.5, 0.7, 1.0), &a);
}

fn main() {
    // Output a PPM file
    // Create a file
    let _ = File::create("data.ppm").expect("creation of PPM file failed");
    let mut data_file = OpenOptions::new()
        .append(true)
        .open("data.ppm")
        .expect("cannot open PPM file");

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Calculate the image height, and ensure that it's at least 1.
    let mut image_height = ((image_width as f64) / aspect_ratio) as i64;
    image_height = if image_height < 1 { 1 } else { image_height };

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ((image_width as f64) / (image_height as f64));
    let camera_center = vec3::Point3::create(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = vec3::Vec3::create(viewport_width, 0.0, 0.0);
    let viewport_v = vec3::Vec3::create(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = vec3::div_scalar(&viewport_u, &(image_width as f64));
    let pixel_delta_v = vec3::div_scalar(&viewport_v, &(image_height as f64));

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center
                             - vec3::Vec3::create(0.0, 0.0, focal_length) - vec3::div_scalar(&viewport_u, &2.0) - vec3::div_scalar(&viewport_v, &2.0);
    let pixel00_loc = viewport_upper_left + vec3::mul_scalar(&(pixel_delta_u + pixel_delta_v), &0.5);

    // Viewport widths less than one are ok since they are real valued.
    // let viewport_height = 2.0;
    // let viewport_width = viewport_height * ((image_width as f64) / (image_height as f64));
    
    // Render
    let file_data = format!("P3\n{} {}\n255\n", image_width, image_height);
    let _ = data_file
        .write(file_data.as_bytes())
        .expect("writing file_data failed");

    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {:?}", (image_height - j));
        std::io::stderr().flush().expect("flush failed");
        
        for i in 0..image_width {
            let pixel_center = pixel00_loc + vec3::mul_scalar(&pixel_delta_u, &(i as f64)) + mul_scalar(&pixel_delta_v, &(j as f64));
            let ray_direction = pixel_center - camera_center;
            let r = ray::Ray::create(camera_center, ray_direction);

            let pixel_colour = ray_color(&r);
            
            // TODO: Once the function signature is changed, change this too
            // write_color(std::cout, pixel_color);
            vec3::write_colour(&mut data_file, &pixel_colour);
        }
    }

    eprintln!("\rDone.                 \n");

    // TODO: move these lines of code into tests
    let v3 = vec3::Vec3::new();
    let ve3 = vec3::Vec3::create(1.0, 2.0, 0.0);
    println!("{:?}", v3);
    // println!("{:?}", -ve3);
    // println!("{:?}", ve3.x());
}
