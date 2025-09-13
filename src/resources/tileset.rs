use std::{
    io::{Seek, SeekFrom, Write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::utils;

use super::ResCompilerArgs;

#[derive(Serialize, Deserialize)]
pub struct TileSetDef {
    pub id: String,
    pub path: PathBuf,
    #[serde(default)]
    keep_duplicates: bool,
    #[serde(default)]
    export_plane_arrangement: bool,
    #[serde(default)]
    plane_arrangement_offset: usize,
}

pub struct ResolvedTileSet {
    pub arrangement: Vec<u16>,
    pub width: usize,
    pub height: usize,
}

pub fn compile(
    ResCompilerArgs {
        resources_path,
        build_buffer,
        ref mut header_buffer,
        ref mut source_buffer,
        res_config,
        resolved,
        ..
    }: &mut ResCompilerArgs,
) -> anyhow::Result<()> {
    header_buffer.write_all(b"\n// ---- tilesets ----\n")?;
    source_buffer.write_all(b"\n// ---- tilesets ----\n")?;

    for item in res_config.tilesets.iter() {
        let utils::IndexedImage {
            buffer,
            width,
            height,
            ..
        } = utils::load_indexed_image(&resources_path.join(&item.path))?;

        let mut tile_amount = 0;
        let mut tile_hashes: Vec<md5::Digest> = Vec::new();
        let mut tile_arrangement: Vec<u16> = Vec::new();

        let data_address = build_buffer.seek(SeekFrom::Current(0))?;

        for ty in (0..height as usize).step_by(8) {
            for tx in (0..width as usize).step_by(8) {
                // we need the flipped tiles' hashes to check for flipped tile duplicates
                let mut tile: [u8; 64] = [0; 64];
                let mut tile_hflipped: [u8; 64] = [0; 64];
                let mut tile_vflipped: [u8; 64] = [0; 64];
                let mut tile_hvflipped: [u8; 64] = [0; 64];

                for y in 0..8 {
                    for x in 0..8 {
                        tile[x + y * 8] = buffer[tx + x + (ty + y) * width];
                        tile_hflipped[7 - x + y * 8] = buffer[tx + x + (ty + y) * width];
                        tile_vflipped[x + (7 - y) * 8] = buffer[tx + x + (ty + y) * width];
                        tile_hvflipped[7 - x + (7 - y) * 8] = buffer[tx + x + (ty + y) * width];
                    }
                }

                let tile_hash = md5::compute(tile);

                if !tile_hashes.contains(&tile_hash) || item.keep_duplicates {
                    build_buffer.write_all(&tile)?;
                    tile_hashes.push(tile_hash);
                    tile_amount += 1;

                    tile_arrangement
                        .push((tile_hashes.len() - 1 + item.plane_arrangement_offset) as u16);
                } else {
                    tile_arrangement.push(
                        (tile_hashes
                            .iter()
                            .position(|x| *x == tile_hash)
                            .unwrap_or(0)
                            + item.plane_arrangement_offset) as u16,
                    );
                }
            }
        }

        let stringified_arrangement = tile_arrangement
            .iter()
            .map(|x| x.to_string() + ", ")
            .collect::<String>();

        resolved.tilesets.insert(
            item.id.clone(),
            ResolvedTileSet {
                arrangement: tile_arrangement,
                width: width / 8,
                height: height / 8,
            },
        );

        header_buffer.write_fmt(format_args!("extern TileSetResource RES_{};\n", item.id))?;

        if item.export_plane_arrangement {
            source_buffer.write_fmt(format_args!(
                "uint16_t ARRANGE_{}[] = {{ {} }};\n",
                item.id, stringified_arrangement
            ))?;
            source_buffer.write_fmt(format_args!(
                "TileSetResource RES_{} = {{ .address = {}, .size = {}, .planeArrangement = ARRANGE_{} }};\n",
                item.id,
                data_address,
                tile_amount * 64,
                item.id
            ))?;
        } else {
            source_buffer.write_fmt(format_args!(
                "TileSetResource RES_{} = {{ .address = {}, .size = {}, .planeArrangement = NULL }};\n",
                item.id,
                data_address,
                tile_amount * 64,
            ))?;
        }
    }

    Ok(())
}
