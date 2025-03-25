use std::io;

use std::io::Seek;
use std::io::Write;

use std::fs::File;

use image::DynamicImage;
use image::ImageFormat;
use image::ImageReader;

pub fn img2pnm2wtr<W>(i: &DynamicImage, wtr: &mut W) -> Result<(), io::Error>
where
    W: Write + Seek,
{
    i.write_to(wtr, ImageFormat::Pnm).map_err(io::Error::other)
}

pub fn pngfilename2img(name: &str) -> Result<DynamicImage, io::Error> {
    let rdr: ImageReader<_> = ImageReader::open(name)?;
    rdr.decode().map_err(io::Error::other)
}

pub fn pngfilename2ppmfilename(ipng: &str, oppm: &str) -> Result<(), io::Error> {
    let img: DynamicImage = pngfilename2img(ipng)?;
    let mut f: File = File::create(oppm)?;
    img2pnm2wtr(&img, &mut f)?;
    f.flush()?;
    f.sync_all()
}
