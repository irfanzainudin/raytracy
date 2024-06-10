mod helloworld;
mod vec3;

fn main() {
    helloworld::helloworld();

    let v3 = vec3::Vec3::new();
    let ve3 = vec3::Vec3::create(1.0, 2.0, 0.0);
    println!("{:?}", v3);
    // println!("{:?}", -ve3);
    // println!("{:?}", ve3.x());
}
