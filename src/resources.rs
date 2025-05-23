use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufWriter, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

pub mod audio;
pub mod palette;
pub mod rawdata;
pub mod spriteset;
pub mod tilemap;
pub mod tileset;

#[derive(Serialize, Deserialize)]
pub struct ResConfig {
    #[serde(default)]
    audio: Vec<audio::AudioDef>,
    #[serde(default)]
    palettes: Vec<palette::PaletteDef>,
    #[serde(default)]
    rawdata: Vec<rawdata::RawDataDef>,
    #[serde(default)]
    spritesets: Vec<spriteset::SpriteSetDef>,
    #[serde(default)]
    tilesets: Vec<tileset::TileSetDef>,
    #[serde(default)]
    tilemaps: Vec<tilemap::TileMapDef>,
}

pub struct ResCompilerArgs {
    resources_path: PathBuf,
    res_config: ResConfig,
    data_buffer: BufWriter<File>,
    header_buffer: BufWriter<File>,
    source_buffer: BufWriter<File>,
    resolved: ResolvedResources,
}
pub struct ResolvedResources {
    tilesets: HashMap<String, tileset::ResolvedTileSet>,
}

impl Default for ResolvedResources {
    fn default() -> ResolvedResources {
        ResolvedResources {
            tilesets: HashMap::new(),
        }
    }
}

fn write_preamble(
    ResCompilerArgs {
        ref mut header_buffer,
        ref mut data_buffer,
        ref mut source_buffer,
        ..
    }: &mut ResCompilerArgs,
) -> anyhow::Result<()> {
    data_buffer.write_all(b"ANDES     ")?; // 5 last bytes are reserved for versioning and stuff
    data_buffer.seek(SeekFrom::Current(8))?; // the next 8 bytes are where the resource data length is stored

    header_buffer
        .write_all(b"// AUTOMATICALLY GENERATED BY ANDES SDK. MODIFYING NOT RECOMMENDED.\n\n")?;
    source_buffer
        .write_all(b"// AUTOMATICALLY GENERATED BY ANDES SDK. MODIFYING NOT RECOMMENDED.\n\n")?;

    header_buffer.write_all(b"#pragma once\n\n#include <andes_res_types.h>\n\n")?;
    source_buffer.write_all(b"#include <andes_resources.h>\n\n")?;

    Ok(())
}

fn write_data_length(
    ResCompilerArgs {
        ref mut data_buffer,
        ..
    }: &mut ResCompilerArgs,
) -> anyhow::Result<()> {
    let res_data_length = data_buffer.seek(SeekFrom::Current(0))?.to_le_bytes();
    data_buffer.seek(SeekFrom::Start(10))?; // andes app magic header is 10 bytes long
    data_buffer.write_all(&res_data_length)?;

    data_buffer.seek(SeekFrom::End(0))?;

    Ok(())
}

pub fn compile_all(project_path: &Path) -> anyhow::Result<()> {
    let resources_path = project_path.join("resources");

    let res_config: ResConfig =
        serde_yml::from_reader(File::open(resources_path.join("config.yml"))?)?;

    let _ = fs::create_dir(project_path.join("build"));

    let data_file = fs::File::create(project_path.join("build/resources.bin"))?;

    let header_file = fs::File::create(resources_path.join("andes_resources.h"))?;
    let source_file = fs::File::create(resources_path.join("andes_resources.c"))?;

    let mut compiler_args = ResCompilerArgs {
        resources_path,
        res_config,
        data_buffer: BufWriter::new(data_file),
        header_buffer: BufWriter::new(header_file),
        source_buffer: BufWriter::new(source_file),
        resolved: ResolvedResources::default(),
    };

    write_preamble(&mut compiler_args)?;

    audio::compile(&mut compiler_args)?;
    palette::compile(&mut compiler_args)?;
    rawdata::compile(&mut compiler_args)?;
    spriteset::compile(&mut compiler_args)?;
    tileset::compile(&mut compiler_args)?;
    tilemap::compile(&mut compiler_args)?;

    write_data_length(&mut compiler_args)?;

    Ok(())
}
