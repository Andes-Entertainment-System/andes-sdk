use std::{
    io::{Seek, SeekFrom, Write},
    path::PathBuf,
};

use same_file::is_same_file;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tiled::LayerType;

use super::ResCompilerArgs;

#[derive(Serialize, Deserialize)]
pub struct TileMapDef {
    id: String,
    path: PathBuf,
}

#[derive(Error, Debug)]
pub enum TileMapError {
    #[error("Invalid file extension for tilemap at '{0}'. Only Tiled TMX tilemaps are supported.")]
    InvalidFileExtension(PathBuf),
    #[error("Tilemap at '{0}' uses no tilesets, therefore it is invalid.")]
    NoTileSets(PathBuf),
    #[error("Tilemap at '{0}' uses more than one tileset, which is not supported.")]
    MoreThanOneTileSet(PathBuf),
    #[error("Tilemap at '{0}' uses a TSX tileset whose source image is not registered as a tileset on the resource config file.")]
    TilesetNotRegistered(PathBuf),
    #[error("Tilemap at '{0}' has a tile grid whose dimensions aren't powers of 2 and/or are smaller than 8.")]
    InvalidTileGridSize(PathBuf),

    // the rust formatter just wouldn't give up on formatting this one like this, EVEN THOUGH the other ones are even longer
    #[error(
        "Tilemap at '{0}' uses a TSX tileset composed of chunks that don't match the size of the map's tile grid."
    )]
    TileGridAndTileSetDontMatch(PathBuf),
}

pub fn compile(
    ResCompilerArgs {
        ref mut header_buffer,
        ref mut data_buffer,
        ref mut source_buffer,
        res_config,
        ..
    }: &mut ResCompilerArgs,
) -> anyhow::Result<()> {
    header_buffer.write_all(b"\n// ---- tilemaps ----\n")?;
    source_buffer.write_all(b"\n// ---- tilemaps ----\n")?;

    for item in res_config.tilemaps.iter() {
        let mut tiled_loader = tiled::Loader::new();
        let tilemap = match item.path.extension() {
            Some(os_str) => match os_str.to_str() {
                Some("tmx") => Ok(tiled_loader.load_tmx_map(&item.path)?),
                _ => Err(TileMapError::InvalidFileExtension(item.path.clone())),
            },
            None => Err(TileMapError::InvalidFileExtension(item.path.clone())),
        }?;

        if tilemap.tilesets().len() > 1 {
            return Err(TileMapError::MoreThanOneTileSet(item.path.clone()).into());
        }

        // find the tileset resource associated with the tileset used by this map
        let tileset = tilemap
            .tilesets()
            .first()
            .ok_or(TileMapError::NoTileSets(item.path.clone()))?;
        let tileset_image_src = match tileset.image.as_ref() {
            Some(image) => Ok(image.source.clone()),
            None => Err(TileMapError::TilesetNotRegistered(item.path.clone())),
        }?;
        let tileset_id = res_config
            .tilesets
            .iter()
            .find(|x| is_same_file(&x.path, &tileset_image_src).unwrap_or(false))
            .map_or_else(
                || Err(TileMapError::TilesetNotRegistered(item.path.clone())),
                |tileset| Ok(tileset.id.clone()),
            )?;

        // chunk size checks
        if !tilemap.tile_width.is_power_of_two()
            || !tilemap.tile_width < 8
            || !tilemap.tile_height.is_power_of_two()
            || !tilemap.tile_height < 8
        {
            return Err(TileMapError::InvalidTileGridSize(item.path.clone()).into());
        }

        if tilemap.tile_width != tileset.tile_width || tilemap.tile_height != tileset.tile_height {
            return Err(TileMapError::TileGridAndTileSetDontMatch(item.path.clone()).into());
        }

        let mut chunk_layout: Vec<u16> = vec![0; (tilemap.width * tilemap.height) as usize];

        for layer in tilemap.layers() {
            match layer.layer_type() {
                LayerType::Tiles(tiles) => {
                    let width = tiles.width().unwrap_or(tilemap.width) as usize;
                    let height = tiles.height().unwrap_or(tilemap.height) as usize;

                    let process_tile = if layer.name.ends_with("high") {
                        // high priority layer
                        |id: u32, flip_h: bool, flip_v: bool, chunk: &mut u16| {
                            *chunk = (id & 8191) as u16;
                            *chunk |= (flip_v as u16) << 13 | (flip_h as u16) << 14 | 1 << 15;
                        }
                    } else if layer.name.ends_with("priority") {
                        // priority layer (changes tile priority according to tile presence, no tile is low and tile is high)
                        |id: u32, _flip_h: bool, _flip_v: bool, chunk: &mut u16| {
                            *chunk &= !(1 << 15);
                            *chunk |= (id.min(1) as u16) << 15;
                        }
                    } else {
                        // low priority layer
                        |id: u32, flip_h: bool, flip_v: bool, chunk: &mut u16| {
                            *chunk = (id & 8191) as u16;
                            *chunk |= (flip_v as u16) << 13 | (flip_h as u16) << 14;
                        }
                    };

                    for y in 0..height {
                        for x in 0..width {
                            match tiles.get_tile(x as i32, y as i32) {
                                Some(tile) => {
                                    process_tile(
                                        tile.id(),
                                        tile.flip_h,
                                        tile.flip_v,
                                        &mut chunk_layout[x + y * width],
                                    );
                                }
                                None => {
                                    process_tile(0, false, false, &mut chunk_layout[x + y * width]);
                                }
                            };
                        }
                    }
                }
                // LayerType::Objects(objects) => {}
                _ => (),
            }
        }

        let data_address = data_buffer.seek(SeekFrom::Current(0))?;

        for descriptor in chunk_layout {
            data_buffer.write_all(&descriptor.to_le_bytes())?;
        }

        header_buffer.write_fmt(format_args!("extern TileMapResource RES_{};\n", item.id))?;
        source_buffer.write_fmt(format_args!(
            "TileMapResource RES_{} = {{ .layoutAddress = {}, .layoutSize = {}, .layoutWidth = {}, .layoutHeight = {}, .chunkWidth = {}, .chunkHeight = {}, .tileSet = &RES_{} }};\n",
            item.id,
            data_address,
            data_buffer.seek(SeekFrom::Current(0))? - data_address,
            tilemap.width,
            tilemap.height,
            tilemap.tile_width / 8,
            tilemap.tile_height / 8,
            tileset_id,
        ))?;
    }

    Ok(())
}
