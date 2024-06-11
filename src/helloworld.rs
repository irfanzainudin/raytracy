use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use crate::vec3;

pub fn helloworld() {
    // Output a PPM file
    // Create a file
    let _ = File::create("data.ppm").expect("creation of PPM file failed");
    let mut data_file = OpenOptions::new()
        .append(true)
        .open("data.ppm")
        .expect("cannot open PPM file");
    
    // Image specifications
    let image_width: i32 = 256;
    let image_height: i32 = 256;

    // Render
    let file_data = format!("P3\n{} {}\n255\n", image_width, image_height);
    let _ = data_file
        .write(file_data.as_bytes())
        .expect("writing file_data failed");

    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {:?}", (image_height - j));
        std::io::stderr().flush().expect("flush failed");
        
        for i in 0..image_width {
            let pixel_colour = vec3::Colour {
                e: [(i as f64)/(image_width as f64 - 1.0), (j as f64)/(image_height as f64 - 1.0), 0.0]
            };
            
            // TODO: Once the function signature is changed, change this too
            // write_color(std::cout, pixel_color);
            vec3::write_colour(&mut data_file, &pixel_colour);
        }
    }

    eprintln!("\rDone.                 \n");
}
