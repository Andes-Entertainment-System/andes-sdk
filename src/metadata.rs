use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufWriter, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

use crate::utils;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
pub struct MetadataConfig {
    id: String,
    title: String,
    author: String,
    big_thumbnail_path: PathBuf,
    small_thumbnail_path: PathBuf,
}

#[derive(Error, Debug)]
pub enum MetadataError {
    #[error("The small thumbnail specified for this project is not the correct size. Small thumbnails should be 40x32.")]
    SmallThumbnailMisSize,
    #[error("The big thumbnail specified for this project is not the correct size. Big thumbnails should be 256x192.")]
    BigThumbnailMisSize,
}

pub fn compile_all(project_path: &Path) -> anyhow::Result<()> {
    let meta_config_file = File::open(project_path.join("metadata.yml"))
        .context("Failed to load metadata config file.")?;
    let meta_config: MetadataConfig = serde_yml::from_reader(meta_config_file)?;

    let metadata_file = fs::File::create(project_path.join("build/metadata.bin"))?;
    let mut meta_buffer = BufWriter::new(metadata_file);

    meta_buffer.write_all(meta_config.id.as_bytes())?;
    meta_buffer.write_all(meta_config.title.as_bytes())?;
    meta_buffer.write_all(meta_config.author.as_bytes())?;

    let small_thumbnail_image =
        utils::load_indexed_image(&project_path.join(meta_config.small_thumbnail_path))?;
    let big_thumbnail_image =
        utils::load_indexed_image(&project_path.join(meta_config.big_thumbnail_path))?;

    if small_thumbnail_image.width != 40 || small_thumbnail_image.height != 32 {
        return Err(MetadataError::SmallThumbnailMisSize.into());
    }

    if big_thumbnail_image.width != 256 || big_thumbnail_image.height != 192 {
        return Err(MetadataError::BigThumbnailMisSize.into());
    }

    meta_buffer.write_all(&small_thumbnail_image.buffer)?;
    meta_buffer.write_all(&big_thumbnail_image.buffer)?;

    Ok(())
}
