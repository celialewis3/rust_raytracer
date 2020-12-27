mod vec3;
mod ray;

use ray::Ray;
use vec3::Vec3;



fn color(r: &Ray) {
    let unit_direction: Vec3 = Vec3::unit_vector(&r.direction());
}

fn main() {
    println!("Hello, raytracing!");
}

