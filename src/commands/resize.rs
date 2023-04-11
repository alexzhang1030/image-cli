use std::path::Path;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct ResizeArgs {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: String,

    #[arg(long, required = false)]
    width: Option<u32>,

    #[arg(long, required = false)]
    height: Option<u32>,
}

pub fn handler_resize(args: &ResizeArgs) {
    let ResizeArgs { width, height, input, output } = args;
    let image_path = Path::new(&input);
    let file = image::open(image_path).expect("Cannot open image");
    let (width, height) = (width.unwrap_or(file.width()), height.unwrap_or(file.height()));

    let resized = file.resize(width, height, image::imageops::FilterType::Lanczos3);

    resized.save(output).expect("Cannot save image");
}