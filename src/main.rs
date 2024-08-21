use clap::*;
use image::*;
use indicatif::*;
use indicatif_log_bridge::*;
use log::{debug, info};

mod cli;
mod euclidean;
mod tools;

use cli::*;
use euclidean::*;
use tools::*;

fn main() {
    // logging
    let logger =
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).build();
    let multi = MultiProgress::new();
    LogWrapper::new(multi.clone(), logger).try_init().unwrap();

    // preparation
    let args = Arguments::parse();
    let mut img1 = image::open(&args.images[0]).unwrap();
    let mut img2 = image::open(&args.images[1]).unwrap();
    let mut use_segments: bool = false;
    let mut segments = 1;

    
    let tmp = average_sections(&img1, 2);
    dbg!(&tmp);
    let mut i_tmp = image::RgbImage::new(3, 3);
    for (pixel, val) in i_tmp.pixels_mut().zip(tmp) {
        pixel.0 = [
            val[0] as u8,
            val[1] as u8,
            val[2] as u8,
        ];
    }
    i_tmp.save("tmp.png");

    let resize =
        |image: &DynamicImage, factor: u32, filter_type: imageops::FilterType| -> DynamicImage {
            image.resize(image.width() / factor, image.height() / factor, filter_type)
        };
    if let Some(ratio) = args.scaling.ratio {
        debug!("Rescaling images with ratio {}", ratio);
        let r: u32 = ratio.parse().expect("Bad ratio specified");
        if r > 1 {
            img1 = resize(&img1, r, imageops::FilterType::Lanczos3);
            img2 = resize(&img2, r, imageops::FilterType::Lanczos3);
        }
    }
    if let Some(new_segments) = args.scaling.segments {
        debug!("Using {} averaged segments", new_segments);
        use_segments = true;
        segments = new_segments;
    }

    let (width, height) = get_max_image_sizes(vec![&img1, &img2]);

    let pg = multi.add(ProgressBar::new((width * height) as u64));
    match args.method {
        Method::Euclidean => {
            if args.heatmap {
                info!("Generating euclidean difference per pixel with heatmap");
                let mut heatmap = image::RgbImage::new(width, height);
                let diffs = euclidean_diff(img1, img2, width * height, &pg);
                info!("Generated differences");
                for (i, pixel) in heatmap.pixels_mut().enumerate() {
                    pixel.0 = [diffs[i] as u8; 3];
                }
                heatmap.save("heatmap.png").unwrap();
            } else {
                info!("Generating euclidean difference averaged for the image");
                let diff = euclidean_diff_average(img1, img2, width * height, &pg);
                println!("{}", diff);
            }
        }
    }
}
