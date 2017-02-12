#[macro_use] extern crate itertools;
extern crate image;
extern crate imageproc;
use image::{GenericImage, Pixel};

use std::path::Path;

struct Image {
    inner: image::DynamicImage,
}

type GradientBuffer = image::ImageBuffer<image::Luma<u16>, Vec<u16>>;

impl Image {
    pub fn load_image(path: &Path) -> Image {
        Image {
            inner: image::open(path).unwrap()
        }
    }

    fn gradient_magnitude(&self) -> GradientBuffer {
        let (red, green, blue) = decompose(&self.inner);
        let r_grad = imageproc::gradients::sobel_gradients(red.as_luma8().unwrap());
        let g_grad = imageproc::gradients::sobel_gradients(green.as_luma8().unwrap());
        let b_grad = imageproc::gradients::sobel_gradients(blue.as_luma8().unwrap());


        let (w, h) = r_grad.dimensions();
        let mut container = Vec::with_capacity((w * h) as usize);
        for (r, g, b) in izip!(r_grad.pixels(), g_grad.pixels(), b_grad.pixels()) {
            container.push(r[0] + g[0] + b[0]);
        }
        image::ImageBuffer::from_raw(w, h, container).unwrap()
    }
}

fn save_to_file(image: &GradientBuffer, file_path: &'static str) {
    let u8_container = image.pixels().map(|a| (a[0] / 2) as u8).collect::<Vec<_>>();
    let image: image::ImageBuffer<image::Luma<u8>, Vec<u8>> = image::ImageBuffer::from_raw(image.width(), image.height(), u8_container).unwrap();
    image.save(&Path::new(file_path)).unwrap();
}

fn decompose(image: &image::DynamicImage) -> (image::DynamicImage, image::DynamicImage, image::DynamicImage) {
    let w = image.width();
    let h = image.height();
    let mut red = image::DynamicImage::new_luma8(w, h);
    let mut green = image::DynamicImage::new_luma8(w, h);
    let mut blue = image::DynamicImage::new_luma8(w, h);
    for (x, y, pixel) in image.pixels() {
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];
        red.put_pixel(x, y, *image::Rgba::from_slice(&[r, r, r, 255]));
        green.put_pixel(x, y, *image::Rgba::from_slice(&[g, g, g, 255]));
        blue.put_pixel(x, y, *image::Rgba::from_slice(&[b, b, b, 255]));
    }
    (red, green, blue)
}

pub fn lib() {
    let image = Image::load_image(Path::new("sample-image.jpg"));
    let grad = image.gradient_magnitude();
    save_to_file(&grad, "gradient.jpeg");
}


#[test]
fn it_works() {
}
