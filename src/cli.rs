use clap::*;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(trailing_var_arg = true)]
pub struct Arguments {
    /// Selects which method to use
    #[arg()]
    pub method: Method,
    /// How high-res the difference calculation should be
    #[clap(flatten)]
    pub scaling: Scaling,
    /// Whether to generate a heatmap of similarity
    #[arg(long, action)]
    pub heatmap: bool,
    /// The images to compare
    #[clap(required = true, num_args = 2)]
    pub images: Vec<PathBuf>,
}

#[derive(ValueEnum, Clone)]
pub enum Method {
    /// Use euclidean distance, aka Pythagorean distance
    Euclidean,
    /// Downscale the image and generate a fingerprint
    Fingerprint,
    /// Use Structural Similarity Measurement
    SSIM,
}

#[derive(Clone, Args)]
#[group(required = false, multiple = false)]
pub struct Scaling {
    /// Resize by amount, 1 = full, 2 = half, 4 = quarter
    #[clap(short, long)]
    pub ratio: Option<String>,
    /// Split into averaged segments, uses smallest image to determine segment width and height
    #[clap(short, long)]
    pub segments: Option<u32>,
}
