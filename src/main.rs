extern crate image;
extern crate rand;
extern crate rand_chacha;

use image::ImageBuffer;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;

fn main() {
    println!("Hello, world!");

    let imgx = 400;
    let imgy = 300;

    let mut imgbuf = ImageBuffer::new(imgx, imgy);
    let rng: ChaChaRng = SeedableRng::from_seed([1,2,3,4,5,6,7,81,
                                                 9,8,7,6,5,4,3,2,
                                                 5,5,5,5,5,5,5,5,
                                                 1,2,3,4,5,6,7,8]);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([x as u8, 0, y as u8]);
    }

    imgbuf.save("image.png").unwrap();
}

fn v0(x: u32, y: u32) -> (u32, u32) {
    (x, y)
}

fn v1(x: u32, y: u32) -> (u32, u32) {
    let xf = f64::from(x);
    let yf = f64::from(y);
    (xf.sin() as u32, yf.sin() as u32)
}
