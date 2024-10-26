use image::{DynamicImage, GenericImageView, Pixel, RgbImage};

pub fn average_section(
    image: &DynamicImage,
    section_bounds: (u32, u32),
    start: (u32, u32),
) -> [f32; 3] {
    let s_img = image.view(start.0, start.1, section_bounds.0, section_bounds.1);
    let mut r_sum: u32 = 0;
    let mut g_sum: u32 = 0;
    let mut b_sum: u32 = 0;
    for pxl in s_img.pixels() {
        r_sum += pxl.2.channels()[0] as u32;
        g_sum += pxl.2.channels()[1] as u32;
        b_sum += pxl.2.channels()[2] as u32;
    }
    let pixels = (s_img.width() * s_img.height()) as f32;
    [
        r_sum as f32 / pixels,
        g_sum as f32 / pixels,
        b_sum as f32 / pixels,
    ]
}
pub fn average_sections(image: &DynamicImage, section_splits: u32) -> Vec<[f32; 3]> {
    todo!("implement sections");
    let section_count = (section_splits + 1).pow(2);
    let section_width = image.width() / (section_splits + 1);
    let section_height = image.height() / (section_splits + 1);
    let mut sections: Vec<[f32; 3]> = Vec::with_capacity(section_count as usize);
    for _ in 0..section_count {
        sections.push([0.0; 3]);
    }
    for (x, y, pixel) in image.pixels() {
        todo!("You need to take x and y and somehow get in which segment that is");
    }
    dbg!(&sections);
    sections
        .iter()
        .map(|section| {
            [
                section[0] / (section_width * section_height) as f32,
                section[1] / (section_width * section_height) as f32,
                section[2] / (section_width * section_height) as f32,
            ]
        })
        .collect()
}

pub fn determine_segment_dimensions(
    total_width: u32,
    total_height: u32,
    segments: u8,
) -> (u32, u32) {
    (
        total_width / segments as u32,
        total_height / segments as u32,
    )
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
pub fn get_min_image_sizes(images: Vec<&DynamicImage>) -> (u32, u32) {
    let mut min_width = u32::MAX;
    let mut min_height = u32::MAX;
    let mut min_pixels = u32::MAX;
    for image in images {
        let p = image.width() * image.height();
        if p < min_pixels {
            min_pixels = p;
            min_width = image.width();
            min_height = image.height();
        }
    }
    (min_width, min_height)
}
pub fn save_img_from_arr(pixels: &[[u8; 3]], dimensions: (u32, u32), name: &str) {
    if pixels.len() < (dimensions.0 as usize * dimensions.1 as usize) {
        return;
    }
    let mut img = RgbImage::new(dimensions.0, dimensions.1);
    for (value, pixel) in pixels.iter().zip(img.pixels_mut()) {
        pixel.0 = *value;
    }
    img.save(name).unwrap();
}
