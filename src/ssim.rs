use image::{DynamicImage, GenericImage, GenericImageView, Pixel};

fn compute_ssim(image1: &DynamicImage, image2: &DynamicImage) -> f32 {
    // compare luminance

    todo!();
    let mu_x: f32;
    let mu_y: f32;
    let sigma_x: f32;
    let sigma_y: f32;
    let sigma_xy: f32;
    let dynamic_range: f32;
    let k_1 = 0.01_f32;
    let k_2 = 0.03_f32;
    let stabilizer_1: f32 = (k_1 * dynamic_range).powf(2.0);
    let stabilizer_2: f32 = (k_2 * dynamic_range).powf(2.0);
    ((2.0 * mu_x * mu_y + stabilizer_1) * (2.0 * sigma_xy + stabilizer_2))
        / ((mu_x.powf(2.0) + mu_y.powf(2.0) + stabilizer_1) * (sigma_x + sigma_y + stabilizer_2))
}

//fn luminance(x: &DynamicImage, y: &DynamicImage) -> f64 {
//    let mu_x = image_mean(&x);
//    let mu_y = image_mean(&y);
//    // works only with 8bit image
//    //const L: f64 = 255.0;
//    //let (k_1, k_2): (f64, f64) = (0.01, 0.03);
//    let c1 = 6.502500000000001; // (k_1 * L).powf(2.0);
//
//    (2.0 * mu_x * mu_y + c1) / (mu_x.powf(2.0) + mu_y.powf(2.0) + c1)
//}
fn contrast(x: &DynamicImage, y: &DynamicImage) -> f64 {
    let sigma_x = todo!();
}

//fn average_luminance(image: &DynamicImage) -> f32 {
//    let n = image.width() * image.height();
//    let mut avg = 0.0;
//
//    for p in image.pixels() {
//        avg += luminance([p.2.0[0], p.2.0[1], p.2.0[2]]) / n as f32;
//    }
//    return avg;
//}

pub fn test(a: &DynamicImage, b: &DynamicImage) {
    let grayscale = |img: &DynamicImage, title: &str, f: &dyn Fn(u8, u8, u8) -> f32| {
        let mut tmp = DynamicImage::new_luma8(img.width(), img.height());
        for pixel in img.pixels() {
            tmp.put_pixel(
                pixel.0,
                pixel.1,
                image::Luma::from([(f(pixel.2 .0[0], pixel.2 .0[1], pixel.2 .0[2])) as u8])
                    .to_rgba(),
            );
        }
        let _ = tmp.save(title);
    };
    grayscale(&a, "601.png", &luminance::luma601);
    grayscale(&b, "709.png", &luminance::luma709);
    grayscale(&a, "luminance.png", &|r: u8, g: u8, b: u8| {
        luminance::luminance([r, g, b]) * 255_f32
    });
}
fn image_mean(image: &DynamicImage) -> f64 {
    image
        .pixels()
        .map(|pixel| grayscale_pixel(pixel.2[0], pixel.2[1], pixel.2[2]))
        .sum()
}

fn grayscale_pixel(r: u8, g: u8, b: u8) -> f64 {
    r as f64 * 0.299 + g as f64 * 0.587 + b as f64 * 0.114
}

mod luminance {
    /// Convert rgb values to luma value based on CCIR 601 https://www.itu.int/rec/R-REC-BT.601/
    pub fn luma601(r: u8, g: u8, b: u8) -> f32 {
        0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32
    }
    /// Convert rgb values to luma value based on BT. 709 https://www.itu.int/rec/R-REC-BT.709
    pub fn luma709(r: u8, g: u8, b: u8) -> f32 {
        0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32
    }

    /// Apply gamma encode on decimal colors (?) (0.5, 0.0, 0.0) = (127, 0, 0)
    fn gamma_encode(value: f32) -> f32 {
        value.powf(1.0 / 2.2)
    }
    /// convert a gamma-encoded color value to a linear value (https://stackoverflow.com/a/56678483)
    fn srgb_to_lin(value: f32) -> f32 {
        // Send this function a decimal sRGB gamma encoded color value
        // between 0.0 and 1.0, and it returns a linearized value.
        if value <= 0.04045 {
            return value / 12.92;
        } else {
            return ((value + 0.055) / 1.055).powf(2.4);
        }
    }
    /// Compute luminance of an rgb8 pixel
    #[allow(non_snake_case)]
    pub fn luminance(pixel: [u8; 3]) -> f32 {
        let vR = gamma_encode(pixel[0] as f32 / 255.0);
        let vG = gamma_encode(pixel[1] as f32 / 255.0);
        let vB = gamma_encode(pixel[2] as f32 / 255.0);
        let luminance =
            0.2126 * srgb_to_lin(vR) + 0.7152 * srgb_to_lin(vG) + 0.0722 * srgb_to_lin(vB);
        return luminance;
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
