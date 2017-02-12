#[macro_use] extern crate itertools;
extern crate image;
extern crate imageproc;
use image::{GenericImage, Pixel};

use std::path;

struct Image {
    inner: image::DynamicImage,
}

type GradientBuffer = image::ImageBuffer<image::Luma<u16>, Vec<u16>>;

impl Image {
    pub fn load_image(path: &path::Path) -> Image {
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
    let u8_container = image.pixels().map(|a| (a[0] / 128) as u8).collect::<Vec<_>>();
    let image: image::ImageBuffer<image::Luma<u8>, Vec<u8>> = image::ImageBuffer::from_raw(image.width(), image.height(), u8_container).unwrap();
    image.save(&path::Path::new(file_path)).unwrap();
}

fn decompose(image: &image::DynamicImage) -> 
        (image::DynamicImage, image::DynamicImage, image::DynamicImage) {
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

struct DPTable {
    width: usize,
    height: usize,
    table: Vec<u16>,
}

// TODO: horizontal
impl DPTable {
    fn get(&self, w: usize, h: usize) -> u16 {
        let i = self.width * h + w;
        self.table[i]
    }

    fn set(&mut self, w: usize, h: usize, v: u16) {
        let i = self.width * h + w;
        self.table[i] = v;
    }

    fn to_gradient_buffer(self) -> GradientBuffer {
        GradientBuffer::from_raw(self.width as u32, self.height as u32, self.table).unwrap()
    }

    fn start_index(&self) -> usize {
        self.table.iter()
            .take(self.width)
            .enumerate()
            .map(|(i, n)| (n, i))
            .min()
            .map(|(_, i)| i)
            .unwrap_or(0)

    }

    fn from_gradient_buffer(gradient: &GradientBuffer) -> Self {
        let dims = gradient.dimensions();
        let w = dims.0 as usize;
        let h = dims.1 as usize;
        let mut table = DPTable {
            width: w,
            height: h,
            table: vec![0; w * h],
        };
        // return gradient[h][w]
        let get = |w, h| gradient.get_pixel(w as u32, h as u32)[0];

        // Initialize bottom row
        for i in 0..w {
            let px = get(i, h - 1);
            table.set(i, h - 1, px)
        }
        // For each cell in row j, select the smaller of the cells in the 
        // row above. Special case the end rows
        use std::cmp::min;
        for row in (0..h - 1).rev() {
            for col in 1..w - 1 {
                let l = table.get(col - 1, row + 1);
                let m = table.get(col    , row + 1);
                let r = table.get(col + 1, row + 1);
                table.set(col, row, get(col, row) + min(min(l, m), r));
            }
            // special case far left and far right:
            let left = get(0, row) + min(table.get(0, row + 1), table.get(1, row + 1));
            table.set(0, row, left);
            let right = get(0, row) + min(table.get(w - 1, row + 1), table.get(w - 2, row + 1));
            table.set(w - 1, row, right);
        }
        table
    }
}

/*fn shortest_path_vertical(gradient: &GradientBuffer) -> DPTable<u16> {
    let (w, h) = gradient.dimensions();
    let mut dp_table = DPTable::new(w as usize, h as usize);
    dp_table
}*/

pub fn lib() {
    let image = Image::load_image(path::Path::new("sample-image.jpg"));
    let grad = image.gradient_magnitude();
    let table = DPTable::from_gradient_buffer(&grad);
    println!("{}", table.start_index());
    let grad = table.to_gradient_buffer();
    save_to_file(&grad, "gradient_buffer.jpeg");
}


#[test]
fn it_works() {
}
