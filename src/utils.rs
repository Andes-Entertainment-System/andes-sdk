use std::{fs::File, path::PathBuf};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum UtilsError {
    #[error("Invalid file extension for image at '{0}'. Only indexed PNGs are supported.")]
    InvalidFileExtension(PathBuf),
    #[error("Invalid colour format for image at '{0}'. Only indexed PNGs are supported.")]
    InvalidColourFormat(PathBuf),
}

pub struct IndexedImage {
    pub buffer: Vec<u8>,
    pub palette: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

fn load_indexed_png(source: &PathBuf) -> anyhow::Result<IndexedImage> {
    let decoder = png::Decoder::new(File::open(source)?);

    let reader = decoder.read_info()?;
    let info = reader.info();
    let buf = vec![0; reader.output_buffer_size()];

    if info.color_type != png::ColorType::Indexed {
        return Err(UtilsError::InvalidColourFormat(source.clone()).into());
    }

    let palette = match &info.palette {
        Some(x) => Ok(x.to_vec()),
        None => Err(UtilsError::InvalidColourFormat(source.clone())),
    }?;

    Ok(IndexedImage {
        buffer: buf,
        palette: palette,
        width: info.width as usize,
        height: info.height as usize,
    })
}

pub fn load_indexed_image(source: &PathBuf) -> anyhow::Result<IndexedImage> {
    match source.extension() {
        Some(os_str) => match os_str.to_str() {
            Some("png") => load_indexed_png(source),
            _ => Err(UtilsError::InvalidFileExtension(source.clone()).into()),
        },
        None => Err(UtilsError::InvalidFileExtension(source.clone()).into()),
    }
}
