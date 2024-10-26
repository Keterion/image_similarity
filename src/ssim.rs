use image::{DynamicImage, GenericImageView};

fn compute_ssim(image1: &DynamicImage, image2: &DynamicImage) -> f32 {
    // compare luminance
    let mu = average_luminance(image1) - average_luminance(image2);
    
}

fn average_luminance(image: &DynamicImage) -> f32 {
    let n = image.width() * image.height();
    let mut avg = 0.0;

    for p in image.pixels() {
        avg += luminance([p.2.0[0], p.2.0[1], p.2.0[2]]) / n as f32;
    }
    return avg;
}
fn luminance(pixel: [u8; 3]) -> f32 {
    let r = pixel[0] as f32 / 255.0;
    let g = pixel[1] as f32 / 255.0;
    let b = pixel[2] as f32 / 255.0;
    let luminance = 
          0.2126 * srgb_to_lin(r)
        + 0.7152 * srgb_to_lin(g)
        + 0.0722 * srgb_to_lin(b)
    ;
    return luminance;
}
fn srgb_to_lin(value: f32) -> f32 {
    if value <= 0.04045 {
        return value / 12.92;
    } else {
        return ((value + 0.055)/1.055).powf(2.4);
    }
}

//fn average_lightness(image: &DynamicImage) {
//
//}
//fn lightness(luminance: f32) -> f32 {
//    if luminance <= (216.0 / 24389.0) {
//        return luminance * (24389.0 / 27.0);
//    } else {
//        return luminance.powf(1.0/3.0) * 116.0 - 16.0;
//    }
//}
