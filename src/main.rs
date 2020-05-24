extern crate image;
extern crate rand;
extern crate rand_chacha;

use image::ImageBuffer;
use rand::{SeedableRng, seq::IteratorRandom};
use rand_chacha::ChaChaRng;

fn main() {
    println!("Hello, world!");

    let imgx = 600;
    let imgy = 480;

    let _scalex = 3.0 / imgx as f32;
    let _scaley = 3.0 / imgy as f32;

    let mut imgbuf = ImageBuffer::new(imgx, imgy);
    let mut rng: ChaChaRng = SeedableRng::from_seed([1,2,3,4,5,6,7,81,
                                                 9,8,7,6,5,4,3,2,
                                                 5,5,5,5,5,5,5,5,
                                                 1,2,3,4,5,6,7,8]);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    let (mut x, mut y, _) = imgbuf.enumerate_pixels_mut().choose(&mut rng).unwrap();
    for i in 0..400 {
        let p_norm = normalize(x, y, imgx, imgy);
        let p_norm = f0(p_norm.0, p_norm.1);
        let p = denormalize(p_norm.0, p_norm.1, imgx, imgy);
        x = p.0;
        y = p.1;

        if i < 20 { continue; }

        if x < imgx && y < imgy {
            let pixel = imgbuf.get_pixel_mut(x,y);
            pixel[1] = ((pixel[1] as u32 + 250) / 2) as u8;
        }
    }

    imgbuf.save("image.png").unwrap();
}

fn normalize(x: u32, y: u32, xmax: u32, ymax: u32) -> (f32, f32) {
    let max = if xmax > ymax {
        xmax as f32
    } else {
        ymax as f32
    };

    let x_center = x as f32 - (max / 2.0);
    let y_center = y as f32 - (max / 2.0);

    let x_scaled = x_center / max;
    let y_scaled = y_center / max;

    (x_scaled, y_scaled)
}

fn denormalize(x: f32, y: f32, xmax: u32, ymax: u32) -> (u32, u32) {
    let max = if xmax > ymax {
        xmax as f32
    } else {
        ymax as f32
    };

    let x_center = x * max;
    let y_center = y * max;

    let x_shifted = x_center + (max / 2.0);
    let y_shifted = y_center + (max / 2.0);

    (x_shifted as u32, y_shifted as u32)
}

fn r(x: f32, y: f32) -> f32 {
    (x.powi(2) + y.powi(2)).sqrt()
}

/// Linear
fn v0(x: f32, y: f32) -> (f32, f32) {
    (x, y)
}

/// Sinusoidal
fn v1(x: f32, y: f32) -> (f32, f32) {
    (x.sin(), y.sin())
}

/// Spherical
fn v2(x: f32, y: f32) -> (f32, f32) {
    let x_new = x / (x.powi(2) + y.powi(2));
    let y_new = y / (x.powi(2) + y.powi(2));
    (x_new, y_new)
}

fn f0(x: f32, y: f32) -> (f32, f32) {
    let v0 = v0(0.4*x + 0.5*y + 0.0,
                0.4*x + 0.5*y + 0.0);
    let v0 = (0.6*v0.0, 0.6*v0.1);
    let v1 = v1(0.4*x + 0.5*y + 0.1,
                0.5*x + 0.5*y + 0.0);
    let v1 = (0.1*v1.0, 0.1*v1.1);
    let v2 = v2(-0.1*x + 0.5*y + 0.1,
                0.9*x + 0.5*y + 2.0);
    let v2 = (0.3*v2.0, 0.3*v2.1);

    (v0.0 + v1.0 + v2.0, v0.1 + v1.1 + v2.1)
}
