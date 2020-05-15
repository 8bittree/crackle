extern crate image;

use image::ImageBuffer;

fn main() {
    println!("Hello, world!");

    let imgx = 400;
    let imgy = 300;

    let mut imgbuf = ImageBuffer::new(imgx, imgy);

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
