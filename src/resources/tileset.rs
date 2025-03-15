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
}

pub struct ResolvedTileSet {
    pub arrangement: Vec<u16>,
    pub width: usize,
    pub height: usize,
}

pub fn compile(
    ResCompilerArgs {
        ref mut header_buffer,
        ref mut data_buffer,
        ref mut source_buffer,
        res_config,
        resolved,
        ..
    }: &mut ResCompilerArgs,
) -> anyhow::Result<()> {
    header_buffer.write_all(b"\n// ---- tilesets ----\n")?;
    source_buffer.write_all(b"\n// ---- tilesets ----\n")?;

    for item in res_config.tilesets.iter() {
        let utils::Image {
            buffer,
            width,
            height,
            ..
        } = utils::load_indexed_image(&item.path)?;

        let mut tile_amount = 0;
        let mut tile_hashes: Vec<md5::Digest> = Vec::new();
        let mut tile_arrangement: Vec<u16> = Vec::new();

        let data_address = data_buffer.seek(SeekFrom::Current(0))?;

        for ty in (0..height as usize).step_by(8) {
            for tx in (0..width as usize).step_by(8) {
                let mut tile: [u8; 64] = [0; 64];

                for y in 0..8 {
                    for x in 0..8 {
                        tile[x + y * 8] = buffer[tx + x + (ty + y) * width];
                    }
                }

                if !item.keep_duplicates {
                    let tile_hash = md5::compute(tile);

                    if !tile_hashes.contains(&tile_hash) {
                        data_buffer.write_all(&tile)?;
                        tile_hashes.push(tile_hash);
                        tile_amount += 1;

                        tile_arrangement.push((tile_hashes.len() - 1) as u16);
                    } else {
                        tile_arrangement.push(
                            tile_hashes
                                .iter()
                                .position(|x| *x == tile_hash)
                                .unwrap_or(0) as u16,
                        );
                    }
                } else {
                    data_buffer.write_all(&tile)?;
                }
            }
        }

        resolved.tilesets.insert(
            item.id.clone(),
            ResolvedTileSet {
                arrangement: tile_arrangement,
                width: width / 8,
                height: height / 8,
            },
        );

        header_buffer.write_fmt(format_args!("extern TileSetResource RES_{};\n", item.id))?;
        source_buffer.write_fmt(format_args!(
            "TileSetResource RES_{} = {{ .address = {}, .size = {} }};\n",
            item.id,
            data_address,
            tile_amount * 64,
        ))?;
    }

    Ok(())
}
