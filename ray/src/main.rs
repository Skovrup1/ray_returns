use ray::{
    buf::Buf,
    ray_const::{HEIGHT, WIDTH},
    ray_img::save_img,
    vec::{Color, Poin3, Ray, Vect3},
};

fn ray_color(r: &Ray) -> Color {
    let unit_dir = r.dir.normalize();
    let t = 0.5 * (unit_dir.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let mut img_buf = Buf::default();

    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Poin3::new(0.0, 0.0, 0.0);
    let horizontal = Vect3::new(viewport_width, 0.0, 0.0);
    let vertical = Vect3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vect3::new(0.0, 0.0, focal_length);

    for j in (0..HEIGHT).rev() {
        for i in 0..WIDTH {
            let u = i as f32 / (WIDTH - 1) as f32;
            let v = j as f32 / (HEIGHT - 1) as f32;

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pix_color = ray_color(&r);

            img_buf.change_pix(i, HEIGHT - j - 1, pix_color);
        }
    }
    save_img(img_buf.data.as_slice());
}
