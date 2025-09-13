use std::{
    fs::File,
    io::BufWriter,
    path::{Path, PathBuf},
};

use crate::utils::{self, write_measured};
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

pub fn compile(project_path: &Path, build_buffer: &mut BufWriter<File>) -> anyhow::Result<()> {
    println!("Compiling metadata...");

    let meta_config_file = File::open(project_path.join("metadata.yml"))
        .context("Failed to load metadata config file.")?;
    let meta_config: MetadataConfig = serde_yml::from_reader(meta_config_file)?;

    write_measured(build_buffer, meta_config.id.as_bytes())?;
    write_measured(build_buffer, meta_config.title.as_bytes())?;
    write_measured(build_buffer, meta_config.author.as_bytes())?;

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

    write_measured(build_buffer, &small_thumbnail_image.palette)?;
    write_measured(build_buffer, &small_thumbnail_image.buffer)?;
    write_measured(build_buffer, &big_thumbnail_image.palette)?;
    write_measured(build_buffer, &big_thumbnail_image.buffer)?;

    Ok(())
}
