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
