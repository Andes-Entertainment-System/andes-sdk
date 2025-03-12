use std::{fs::File, path::PathBuf};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum UtilsError {
    #[error("Invalid file extension for image at '{0}'. Only indexed PNGs are supported.")]
    InvalidFileExtension(PathBuf),
    #[error("Invalid colour format for image at '{0}'. Only indexed PNGs are supported.")]
    InvalidColourFormat(PathBuf),
}

pub struct Image {
    pub buffer: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

pub fn load_indexed_image(source: &PathBuf) -> anyhow::Result<Image> {
    let decoder = match source.extension() {
        Some(os_str) => match os_str.to_str() {
            Some("png") => Ok(png::Decoder::new(File::open(source)?)),
            _ => Err(UtilsError::InvalidFileExtension(source.clone())),
        },
        None => Err(UtilsError::InvalidFileExtension(source.clone())),
    }?;

    let mut reader = decoder.read_info()?;
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf)?;

    if info.color_type != png::ColorType::Indexed {
        return Err(UtilsError::InvalidColourFormat(source.clone()).into());
    }

    Ok(Image {
        buffer: buf,
        width: info.width as usize,
        height: info.height as usize,
    })
}
