extern crate image;
extern crate imageproc;
use image::{GenericImage, Pixel};

use std::path::Path;

struct Image {
    inner: image::DynamicImage,
}

impl Image {
    pub fn load_image(path: &Path) -> Image {
        Image {
            inner: image::open(path).unwrap()
        }
    }

    fn gradient_magnitude(&self) {//-> image::ImageBuffer<image::Luma<u16>, Vec<u16>> {
        let (red, green, blue) = decompose(&self.inner);
        let r_grad = imageproc::gradients::sobel_gradients(red.as_luma8().unwrap());
        let g_grad = imageproc::gradients::sobel_gradients(green.as_luma8().unwrap());
        let b_grad = imageproc::gradients::sobel_gradients(blue.as_luma8().unwrap());
        // imageproc::gradients::sobel_gradients(&self.inner)
        save_to_file(&r_grad, "red_grad.jpeg");
        save_to_file(&g_grad, "green_grad.jpeg");
        save_to_file(&b_grad, "blue_grad.jpeg");
    }
}

fn save_to_file(image: &image::ImageBuffer<image::Luma<u16>, Vec<u16>>, file_path: &'static str) {
    // ugh
    let u8_container = image.pixels().map(|a| a[0] as u8).collect::<Vec<_>>();
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
    image.gradient_magnitude();

}


#[test]
fn it_works() {
}
