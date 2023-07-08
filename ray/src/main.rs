pub mod camera;
pub mod hitable;
pub mod hitable_list;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod tracer;
pub mod utility;
pub mod vec;

fn main() {
    tracer::render();
}
