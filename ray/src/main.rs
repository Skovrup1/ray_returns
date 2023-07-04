use std::{f32::INFINITY, rc::Rc, ops::Mul};

use nalgebra::Vector3;
use rand::{Rng};
use ray::{
    buf::Buf,
    camera::Camera,
    framebuffer::save_img,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    sphere::Sphere,
    utility::{HEIGHT, WIDTH},
    vec::{Color, Poin3, Ray, Vect3}, material::{Lambertian, Metal},
};

fn ray_color(r: &Ray, world: &HittableList, depth: u32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    let mut rec: HitRecord = Default::default();
    
    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let mut scatter_r: Ray = Ray::new(Poin3::zeros(), Vect3::zeros());
        let mut attenuation: Color = Color::zeros();
        if rec.mat.scatter(r, &rec, &mut attenuation, &mut scatter_r) {
            return attenuation.component_mul(&ray_color(&scatter_r, &world, depth - 1))
        }

        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_dir = r.dir.normalize();
    let t = 0.5 * (unit_dir.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let mut img_buf = Buf::default();
    let _aspect_ratio = WIDTH as f32 / HEIGHT as f32;
    let samples_per_pixel = 50;
    let max_depth = 50;

    // Camera
    let cam = Camera::new();

    // World
    let mut world: HittableList = Default::default();

    let mat_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.0)));
    let mat_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let mat_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

    world.add(Rc::new(Sphere::new(Poin3::new(0.0, -100.5, -1.0), 100.0, mat_ground)));
    world.add(Rc::new(Sphere::new(Poin3::new(0.0, 0.0, -1.0), 0.5, mat_center)));
    world.add(Rc::new(Sphere::new(Poin3::new(-1.0, 0.0, -1.0), 0.5, mat_left)));
    world.add(Rc::new(Sphere::new(Poin3::new(1.0, 0.0, -1.0), 0.5, mat_right)));

    // Render
    let mut rng = rand::thread_rng();

    for j in (0..HEIGHT).rev() {
        eprint!("\rLines done {}/{}", HEIGHT - j, HEIGHT);
        for i in 0..WIDTH {
            let mut pix_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f32 + rng.gen::<f32>()) / (WIDTH - 1) as f32;
                let v = (j as f32 + rng.gen::<f32>()) / (HEIGHT - 1) as f32;

                let r = cam.get_ray(u, v);
                pix_color += ray_color(&r, &world, max_depth);
            }
            img_buf.write_color(i, HEIGHT - j - 1, pix_color, samples_per_pixel);
        }
    }
    save_img(img_buf.data.as_slice());
}
