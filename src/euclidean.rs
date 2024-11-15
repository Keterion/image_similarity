use image::{DynamicImage, GenericImageView, Pixel, Rgba};
use indicatif::ProgressBar;
use log::{debug, info};

pub fn euclidean_diff_average(
    img1: DynamicImage,
    img2: DynamicImage,
    pixels: u32,
    progress_bar: &ProgressBar,
) -> f32 {
    //    let rgb = |img: &DynamicImage| {
    //        progress_bar.reset();
    //        let v: Vec<(f32, f32, f32)> = img
    //            .pixels()
    //            .map(|pxl| {
    //                (
    //                    pxl.2.channels()[0] as f32,
    //                    pxl.2.channels()[1] as f32,
    //                    pxl.2.channels()[2] as f32,
    //                )
    //            })
    //            .collect();
    //        let (mut r, mut g, mut b): (f32, f32, f32) = (v[0].0 / pixels as f32, v[0].1 / pixels as f32, v[0]);
    //        progress_bar.inc(1);
    //        for &val in v.iter().skip(1) {
    //            r += val.0 as f32 / pixels as f32;
    //            g += val.1 as f32 / pixels as f32;
    //            b += val.2 as f32 / pixels as f32;
    //            progress_bar.inc(1);
    //        }
    //        (r, g, b)
    //    };
    //    let (img1_r, img1_g, img1_b): (f32, f32, f32) = rgb(&img1);
    //    let (img2_r, img2_g, img2_b): (f32, f32, f32) = rgb(&img2);
    //    (((img1_r - img2_r).powf(2.0) + (img1_g - img2_g).powf(2.0) + (img1_b - img2_b).powf(2.0))
    //        as f32)
    //        .sqrt()
    let mut sum = 0.0;
    for value in euclidean_diff(img1, img2, pixels, progress_bar) {
        sum += value / 441.67295;
    }
    sum / pixels as f32
}
pub fn euclidean_diff(
    img1: DynamicImage,
    img2: DynamicImage,
    pixels: u32,
    progress_bar: &ProgressBar,
) -> Vec<f32> {
    let diff = |img1: (f32, f32, f32), img2: (f32, f32, f32)| -> f32 {
        ((img1.0 - img2.0).powf(2.0) + (img1.1 - img2.1).powf(2.0) + (img1.2 - img2.2).powf(2.0))
            .sqrt()
    };

    let split = |pxl: Rgba<u8>| -> (f32, f32, f32) {
        let c = pxl.channels();
        (c[0] as f32, c[1] as f32, c[2] as f32)
    };

    let mut diffs: Vec<f32> = Vec::with_capacity(pixels as usize);
    let start = std::time::Instant::now();
    let mut pixels1 = img1.pixels();
    let mut pixels2 = img2.pixels();
    // somehow the two ways are different
    for _ in 0..pixels as usize {
        if let Some(pxl1) = pixels1.nth(0) {
            if let Some(pxl2) = pixels2.nth(0) {
                diffs.push(diff(split(pxl1.2), split(pxl2.2)));
            } else {
                debug!("No pixel for image2");
                diffs.push(1.0);
            }
        } else {
            debug!("No pixel for image1");
            diffs.push(1.0);
        }
        progress_bar.inc(1);
    }
    info!("Time elapsed: {:#?}", start.elapsed());
    diffs
}
