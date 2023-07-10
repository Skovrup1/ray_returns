use crate::buf::Buf;
use crate::camera::Camera;
use crate::hitable_list::HitableList;
use crate::material::*;
use crate::save::save_img;
use crate::sphere::Sphere;
use crate::utility::*;
use crate::vec::{Color, RandomExt};

use nalgebra::Vector3;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::io::{stderr, Write};

pub fn random_scene() -> HitableList {
    let mut world = HitableList::new();

    let mut rng = thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = rng.gen();

            let center = Vector3::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (center - Vector3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                let sphere_mat: Material;

                if choose_mat < 0.7 {
                    let albedo = Color::random();

                    sphere_mat = Material::Lambertian(Lambertian::new(albedo));
                } else if choose_mat < 0.8 {
                    let albedo = Color::random();
                    let fuzz = rng.gen_range(0.0..0.5);

                    sphere_mat = Material::Metal(Metal::new(albedo, fuzz));
                } else {
                    sphere_mat = Material::Dielectric(Dielectric::new(1.5));
                }

                world.add(Sphere::new(center, 0.2, sphere_mat));
            }
        }
    }
    let mat_ground = Material::Lambertian(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        mat_ground,
    ));

    let mat1 = Material::Dielectric(Dielectric::new(1.5));
    world.add(Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, mat1));

    let mat2 = Material::Lambertian(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, mat2));

    let mat3 = Material::Metal(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, mat3));

    world
}

pub fn render() -> () {
    //world
    let world = random_scene();

    let lookfrom = Vector3::new(13.0, 2.0, 3.0);
    let lookat = Vector3::new(0.0, 0.0, 0.0);
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
    let mut rng = thread_rng();

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rlines: {}/{}", IMAGE_HEIGHT - j, IMAGE_HEIGHT);
        stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES {
                let u = (i as f32 + rng.gen::<f32>()) / (IMAGE_WIDTH as f32 - 1.0);
                let v = (j as f32 + rng.gen::<f32>()) / (IMAGE_HEIGHT as f32 - 1.0);

                let ray = cam.get_ray(u, v);

                pixel_color += ray.color(&world, MAX_DEPTH as i32);
            }

            let x = (i) as u32;
            let y = (IMAGE_HEIGHT - j) as u32;

            buffer.write_color(x, y - 1, pixel_color, SAMPLES);
        }
    }
    save_img(&buffer.data);

    eprintln!("\nDone.");
}

pub fn par_render() {
    //world
    let world = random_scene();

    let lookfrom = Vector3::new(13.0, 2.0, 3.0);
    let lookat = Vector3::new(0.0, 0.0, 0.0);
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
    let h = IMAGE_HEIGHT;
    let w = IMAGE_WIDTH;
    let s = SAMPLES;

    let buffer = (0..h)
        .into_par_iter()
        .rev()
        .flat_map(|y| {
            (0..w)
                .flat_map(|x| {
                    let color: Color = (0..s)
                        .map(|_| {
                            let mut rng = thread_rng();
                            let u = (x as f32 + rng.gen::<f32>()) / (w as f32 - 1.0);
                            let v = (y as f32 + rng.gen::<f32>()) / (h as f32 - 1.0);

                            let ray = cam.get_ray(u, v);

                            ray.color(&world, MAX_DEPTH as i32)
                        })
                        .sum();
                    let scale = (SAMPLES as f32).recip();
                    color
                        .iter()
                        .map(|c| {
                            let c = (c * scale).sqrt();

                            (256.0 * c.clamp(0.0, 0.999)) as u8
                        })
                        .collect::<Vec<u8>>()
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<u8>>();

    save_img(&buffer);
    eprintln!("\nDone.");
}
