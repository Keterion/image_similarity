use image::DynamicImage;

pub fn segment(img: &DynamicImage) -> Vec<Vec<[u8; 3]>> {
    todo!()
}

pub fn get_max_image_sizes(images: Vec<&DynamicImage>) -> (u32, u32) {
    let mut max_width = 0;
    let mut max_height = 0;
    let mut max_pixels = 0;
    for image in images {
        let p = image.width() * image.height();
        if p > max_pixels {
            max_pixels = p;
            max_width = image.width();
            max_height = image.height();
        }
    }
    (max_width, max_height)
}
