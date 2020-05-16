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

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

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
    for i in 0..100 {
        let p = dbg!(f0(x, y));
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

fn v0(x: f32, y: f32) -> (f32, f32) {
    (x, y)
}

fn v1(x: f32, y: f32) -> (f32, f32) {
    let xf = x;
    let yf = y;
    (xf.sin(), yf.sin())
}

fn f0(x: u32, y: u32) -> (u32, u32) {
    let v0 = v0(0.9*x as f32 + 0.75*y as f32 + 0.0,
                0.9*x as f32 + 0.5*y as f32 + 0.8);
    let v0 = (0.6*v0.0, 0.6*v0.1);
    let v1 = v1(1.0*x as f32 + 0.6*y as f32 + 0.0,
                0.9*x as f32 + 0.7*y as f32 + 0.0);
    let v1 = (0.4*v1.0, 0.4*v1.1);

    ((v0.0 + v1.0) as u32, (v0.1 + v1.1) as u32)
}
