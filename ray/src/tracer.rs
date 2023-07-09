use crate::buf::Buf;
use crate::camera::Camera;
use crate::hitable::HitRecord;
use crate::hitable_list::HitableList;
use crate::material::*;
use crate::save::save_img;
use crate::sphere::Sphere;
use crate::utility::*;
use crate::vec::{Color, RandomExt};
use nalgebra::{Point3, Vector3};
use std::io::{stderr, Result, Write};

use std::rc::Rc;

pub fn random_scene() -> HitableList {
    let mut world = HitableList::new();

    let mat_ground = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        mat_ground,
    )));

    for a in (-11..11).rev() {
        for b in (-11..11).rev() {
            let choose_mat = random_float();
            let center = Vector3::new(
                a as f32 + 0.9 * random_float(),
                0.2,
                b as f32 + 0.9 * random_float(),
            );

            if (center - Vector3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                let sphere_mat: Rc<dyn Material> = if choose_mat < 0.8 {
                    let albedo = Color::random().component_mul(&Color::random());
                    Rc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_between(0.5, 1.0);
                    let fuzz = between_float(0.0, 0.5);
                    Rc::new(Metal::new(albedo, fuzz))
                } else {
                    Rc::new(Dielectric::new(1.5))
                };

                world.add(Rc::new(Sphere::new(center, 0.2, sphere_mat)));
            }
        }
    }
    let mat1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, mat1)));

    let mat2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, mat2)));

    let mat3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, mat3)));

    world
}

pub fn render() -> () {
    //world
    let world = random_scene();

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vector3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let vfov = 20.0;
    let apeture = 0.1;

    //camera
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        ASPECT_RATIO,
        apeture,
        dist_to_focus,
    );

    //image
    let mut buffer = Buf::default();

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:3}", j);
        stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES {
                let u = (i as f32 + random_float()) / (IMAGE_WIDTH as f32 - 1.0);
                let v = (j as f32 + random_float()) / (IMAGE_HEIGHT as f32 - 1.0);

                let ray = cam.get_ray(u, v);

                let mut rec = HitRecord::zero();

                pixel_color += ray.color(&mut rec, &world, MAX_DEPTH as i32);
            }

            let x = (i) as u32;
            let y = (IMAGE_HEIGHT - j) as u32;
            
            buffer.write_color(x, y - 1, pixel_color, SAMPLES);
        }
    }
    save_img(&buffer.data);

    eprintln!("\nDone.");
}
