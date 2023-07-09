pub mod camera;
pub mod hitable;
pub mod hitable_list;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod tracer;
pub mod utility;
pub mod vec;
pub mod buf;
pub mod save;

fn main() {
    tracer::render();
}
