use std::fs::{File, OpenOptions};
use std::io::prelude::*;

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
            let r = (i as f64) / (image_width as f64 - 1.0);
            // println!("r: {:?}", r);
            let g = (j as f64) / (image_height as f64 - 1.0);
            // println!("g: {:?}", g);
            let b = 0.0;

            let ir = (255.999 * r) as i64;
            // println!("ir: {:?}", ir);
            let ig = (255.999 * g) as i64;
            let ib = (255.999 * b) as i64;

            let img_data = format!("{} {} {}\n", ir, ig, ib);
            let _ = data_file
                .write(img_data.as_bytes())
                .expect("writing img_data failed");
        }
    }

    eprintln!("\rDone.                 \n");
}