extern crate content_aware_resize;
use content_aware_resize as car;

use std::path::Path;

fn main() {
    let mut image = car::Image::load_image(Path::new("sample-image.jpeg"));
    image.resize_to(car::Dimensions::Relative(-1, 0));
    let data = image.get_image_data();
}
