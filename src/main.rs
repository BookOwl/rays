use image::{RgbImage, Rgb};

fn main() {
    let width = 256;
    let height = 256;
    let mut img = RgbImage::new(width, height);
    for (y, j) in (0..height).rev().enumerate() {
        for (x, i) in (0..width).enumerate() {
            let r = (i as f32) / (width as f32 - 1.0);
            let g = (j as f32) / (height as f32 - 1.0);
            let b = 0.25;
            let pixel = Rgb([(255.999 * r) as u8, (255.999 * g) as u8, (255.999 * b) as u8]);
            img.put_pixel(x as u32, y as u32, pixel);
        }
    }
    img.save("out.png").unwrap();
}
