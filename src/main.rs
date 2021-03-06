extern crate image;
extern crate rand;
extern crate rand_chacha;

use image::ImageBuffer;
use rand::{SeedableRng, seq::IteratorRandom};
use rand::distributions::{Distribution, WeightedIndex};
use rand_chacha::ChaChaRng;

fn main() {
    println!("Hello, world!");

    let imgx = 640;
    let imgy = 480;

    let _scalex = 3.0 / imgx as f32;
    let _scaley = 3.0 / imgy as f32;

    let mut imgbuf = ImageBuffer::new(imgx, imgy);
    let mut rng: ChaChaRng = SeedableRng::from_seed([1,2,3,4,5,6,7,81,
                                                 9,8,7,6,5,4,3,2,
                                                 5,5,5,5,5,5,5,5,
                                                 1,2,3,4,5,6,7,8]);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        //let r = (0.3 * x as f32) as u8;
        //let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([0, 0, 0]);
    }

    let (mut x, mut y, _) = imgbuf.enumerate_pixels_mut().choose(&mut rng).unwrap();
    let fs_vec: Vec<(fn(f32, f32) -> (f32, f32), [u8; 3])> = vec![
        (f0, [0, 250, 0]),
        (f1, [250, 0, 0]),
        (f2, [0, 0, 250]),
    ];
    let fs_weights = vec![
        1,
        10,
        10,
    ];
    let fs = WeightedIndex::new(&fs_weights).unwrap();
    for i in 0..400000 {
        let p_norm = normalize(x, y, imgx, imgy);

        let (f, color) = fs_vec[fs.sample(&mut rng)];

        let p_norm = f(p_norm.0, p_norm.1);
        let p = denormalize(p_norm.0, p_norm.1, imgx, imgy);
        //let p = dbg!(denormalize(p_norm.0, p_norm.1, imgx, imgy));
        /*
        if f == f1 {
            println!("p: {:?}", p);
        }
        */
        x = p.0;
        y = p.1;

        if i < 20 { continue; }

        if x < imgx && y < imgy {
            let pixel = imgbuf.get_pixel_mut(x,y);
            pixel[0] = ((pixel[0] as u32 + color[0] as u32) / 2) as u8;
            pixel[1] = ((pixel[1] as u32 + color[1] as u32) / 2) as u8;
            pixel[2] = ((pixel[2] as u32 + color[2] as u32) / 2) as u8;
        }
    }

    imgbuf.save("image.png").unwrap();
}

fn normalize(x: u32, y: u32, xmax: u32, ymax: u32) -> (f32, f32) {
    let (max, x_shift, y_shift) = if xmax > ymax {
        (xmax as f32, 0.0, (xmax - ymax) as f32 / 2.0)
    } else {
        (ymax as f32, (ymax - xmax) as f32 / 2.0, 0.0)
    };

    let x_center = x as f32 - (max / 2.0) + x_shift;
    let y_center = y as f32 - (max / 2.0) + y_shift;

    let x_scaled = x_center / max;
    let y_scaled = y_center / max;

    (x_scaled, y_scaled)
}

fn denormalize(x: f32, y: f32, xmax: u32, ymax: u32) -> (u32, u32) {
    let (max, x_shift, y_shift) = if xmax > ymax {
        (xmax as f32, 0.0, (xmax - ymax) as f32 / 2.0)
    } else {
        (ymax as f32, (ymax - xmax) as f32 / 2.0, 0.0)
    };

    let x_center = x * max;
    let y_center = y * max;

    let x_decentered = x_center + (max / 2.0) - x_shift;
    let y_decentered = y_center + (max / 2.0) - y_shift;

    (x_decentered as u32, y_decentered as u32)
}

fn r(x: f32, y: f32) -> f32 {
    (x.powi(2) + y.powi(2)).sqrt()
}

fn theta(x: f32, y: f32) -> f32 {
    (x / y).atan()
}

fn phi(x: f32, y: f32) -> f32 {
    (y / x).atan()
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
    let mut x_new = x / (x.powi(2) + y.powi(2));
    if x_new.is_nan() {
        x_new = 0.0;
    }
    let mut y_new = y / (x.powi(2) + y.powi(2));
    if y_new.is_nan() {
        y_new = 0.0
    }
    (x_new, y_new)
}

/// Swirl
fn v3(x: f32, y: f32) -> (f32, f32) {
    (x * r(x,y).powi(2).sin() - y * r(x,y).powi(2).cos(),
     x * r(x,y).powi(2).cos() + y * r(x,y).powi(2).sin())
}

/// Horseshoe
fn v4(x: f32, y: f32) -> (f32, f32) {
    (((x-y) * (x+y)) / r(x,y),
     2.0 * x * y / r(x,y))
}

fn f0(x: f32, y: f32) -> (f32, f32) {
    let v0 = v0(1.0*x + 0.0*y + 0.0,
                0.0*x + 1.0*y + 0.0);
    let v0 = (1.0*v0.0, 1.0*v0.1);
    let v1 = v1(0.0*x + 0.0*y + 0.0,
                0.0*x + 0.0*y + 0.0);
    let v1 = (0.0*v1.0, 0.0*v1.1);
    let v2 = v2(0.0*x + 0.0*y + 0.0,
                0.0*x + 0.0*y + 0.0);
    let v2 = (0.0*v2.0, 0.0*v2.1);

    (v0.0 + v1.0 + v2.0, v0.1 + v1.1 + v2.1)
}

fn f1(x: f32, y: f32) -> (f32, f32) {
    let v3 = v3(0.35*x + 0.35*y + 0.1,
                0.0*x + 0.35*y + 0.1);
    (v3.0, v3.1)
}

fn f2(x: f32, y: f32) -> (f32, f32) {
    let v4 = v4(0.1*x + -0.1*y + -0.1,
                0.1*x + 0.1*y + 0.0);
    (v4.0, v4.1)
}
