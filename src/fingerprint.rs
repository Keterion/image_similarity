use image::{DynamicImage, GenericImageView};

pub fn gen_fingerprint(image: &DynamicImage) -> [u64; 3] {
    let downscaled = image.resize(8, 8, image::imageops::FilterType::Lanczos3);
    let mut r: [u8; 64] = [0; 64];
    let mut r_mean: u32 = 0;
    let mut g: [u8; 64] = [0; 64];
    let mut g_mean: u32 = 0;
    let mut b: [u8; 64] = [0; 64];
    let mut b_mean: u32 = 0;
    for (i, (_, _, pixel)) in downscaled.pixels().enumerate() {
        r[i] = pixel.0[0];
        r_mean += pixel.0[0] as u32;
        g[i] = pixel.0[1];
        g_mean += pixel.0[1] as u32;
        b[i] = pixel.0[2];
        b_mean += pixel.0[2] as u32;
    }
    r_mean = r_mean / (downscaled.width() * downscaled.height());
    g_mean = g_mean / (downscaled.width() * downscaled.height());
    b_mean = b_mean / (downscaled.width() * downscaled.height());
    let aaa = |mean: u32, values: [u8; 64]| -> u64 {
        //let mut tmp = RgbImage::new(8, 8);
        dbg!(&mean);
        let bits: [u8; 64] = values.map(|value| {
            if value as u32 > mean {
                1
            } else {
                0
            }
        });
        //for (value, pixel) in values.iter().zip(tmp.pixels_mut()) {
        //    if *value > mean {
        //        pixel.0 = [255; 3];

        //    } else {
        //        pixel.0 = [0; 3];

        //    }
        //}
        //tmp.save("asdfjl.png");
        let mut res: u64 = 0;
        for (i, bit) in bits.iter().enumerate() {
            res += (*bit as u64).pow(i as u32);
        }
        res
    };
    
    [
        aaa(r_mean, r),
        aaa(g_mean, g),
        aaa(b_mean, b),
    ]
}
