use image::{DynamicImage, Rgba, GenericImageView, Pixel};
use indicatif::ProgressBar;
use log::debug;

pub fn euclidean_diff_average(
    img1: DynamicImage,
    img2: DynamicImage,
    pixels: u32,
    progress_bar: &ProgressBar,
) -> f32 {
    let rgb = |img: &DynamicImage| {
        progress_bar.reset();
        let v: Vec<(u32, u32, u32)> = img
            .pixels()
            .map(|pxl| {
                (
                    pxl.2.channels()[0] as u32,
                    pxl.2.channels()[1] as u32,
                    pxl.2.channels()[2] as u32,
                )
            })
            .collect();
        let (mut r, mut g, mut b): (u32, u32, u32) = v[0];
        progress_bar.inc(1);
        for &val in v.iter().skip(1) {
            r += val.0;
            g += val.1;
            b += val.2;
            progress_bar.inc(1);
        }
        (r / pixels, g / pixels, b / pixels)
    };
    let (img1_r, img1_g, img1_b): (u32, u32, u32) = rgb(&img1);
    let (img2_r, img2_g, img2_b): (u32, u32, u32) = rgb(&img2);
    (((img1_r - img2_r).pow(2) + (img1_g - img2_g).pow(2) + (img1_b - img2_b).pow(2)) as f32).sqrt()
}
pub fn euclidean_diff(
    img1: DynamicImage,
    img2: DynamicImage,
    pixels: u32,
    progress_bar: &ProgressBar,
) -> Vec<f32> {
    let mut diffs: Vec<f32> = Vec::with_capacity(pixels as usize);
    let diff = |img1: (u8, u8, u8), img2: (u8, u8, u8)| -> f32 {
        debug!("Generating diff for {:#?} and {:#?}", img1, img2);
        ((
            ((img1.0.saturating_sub(img2.0)) as u32).pow(2)
            + ((img1.1.saturating_sub(img2.1)) as u32).pow(2)
            + ((img1.2.saturating_sub(img2.2)) as u32).pow(2)) as f32
            ).sqrt()
    };
    let split = |pxl: Rgba<u8>| -> (u8, u8, u8) {
        (pxl.channels()[0], pxl.channels()[1], pxl.channels()[2])
    };
    for i in 0..pixels as usize {
        if let Some(pxl1) = img1.pixels().nth(i) {
            if let Some(pxl2) = img2.pixels().nth(i) {
                diffs.push(diff(split(pxl1.2), split(pxl2.2)))
            } else {
                diffs.push(diff(split(pxl1.2), (0, 0, 0)));
            }
        } else if let Some(pxl2) = img2.pixels().nth(i) {
            diffs.push(diff((0, 0, 0), split(pxl2.2)));
        }
        progress_bar.inc(1);
    }
    diffs
}
