use std::path::PathBuf;

use bincode::{config, encode_into_std_write};
use clap::Parser;
use image::io::Reader as ImageReader;
use wasm_retro_gamekit::asset::image::CompressedRgbaImage;
use wasm_retro_gamekit::asset::Asset;
use wasm_retro_gamekit::compress::{Compression, Data};
use wasm_retro_gamekit::graphics::color::Rgba32;

#[derive(Parser)]
#[command(author = "Jerome Boisvert-Chouinard", version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "rle16")]
    compression: Compression,

    file_in: PathBuf,
    file_out: PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut image_data: Data<Rgba32> = Data::new(args.compression);

    let img = ImageReader::open(args.file_in).unwrap().decode().unwrap();
    let w = img.width() as usize;
    let h = img.height() as usize;
    let rgba_buffer = img.as_rgba8().unwrap();
    for p in rgba_buffer.pixels() {
        let [r, g, b, a] = p.0;
        let c = Rgba32::rgba(r, g, b, a);
        image_data.push(c);
    }

    let asset_dd = Asset::RgbaImage(CompressedRgbaImage::from_pixels(w, h, image_data)).into_blob();
    let mut file_out = std::fs::File::create(args.file_out)?;
    encode_into_std_write(asset_dd, &mut file_out, config::standard()).unwrap();
    Ok(())
}
