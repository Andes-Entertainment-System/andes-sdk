use std::{
    fs::File,
    io::{BufWriter, Seek, SeekFrom, Write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use super::ResConfig;

#[derive(Serialize, Deserialize)]
pub struct TileSetDef {
    id: String,
    path: String,
    #[serde(default)]
    skip_duplicates: bool,
}

pub fn compile(
    res_path: &PathBuf,
    res_config: &ResConfig,
    data_buffer: &mut BufWriter<File>,
    header_buffer: &mut BufWriter<File>,
    source_buffer: &mut BufWriter<File>,
) -> anyhow::Result<()> {
    header_buffer.write_all(b"\n// ---- tilesets ----\n")?;
    source_buffer.write_all(b"\n// ---- tilesets ----\n")?;

    for item in res_config.tilesets.iter() {
        let decoder = png::Decoder::new(File::open(res_path.join(&item.path))?);
        let mut reader = decoder.read_info()?;
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf)?;

        let bytes = &buf[..info.buffer_size()];
        let width = info.width as usize;
        let height = info.height as usize;

        let mut tiles: Vec<[u8; 64]> = Vec::new();
        let mut tile_hashes: Vec<md5::Digest> = Vec::new();

        let data_address = data_buffer.seek(SeekFrom::Current(0))?;

        for ty in (0..height as usize).step_by(8) {
            for tx in (0..width as usize).step_by(8) {
                let mut tile: [u8; 64] = [0; 64];

                for y in 0..8 {
                    for x in 0..8 {
                        tile[x + y * 8] = bytes[tx + x + (ty + y) * width];
                    }
                }

                if item.skip_duplicates {
                    let tile_hash = md5::compute(tile);

                    if !tile_hashes.contains(&tile_hash) {
                        data_buffer.write_all(&tile)?;
                        tile_hashes.push(tile_hash);
                        tiles.push(tile);
                    }
                } else {
                    data_buffer.write_all(&tile)?;
                    tiles.push(tile);
                }
            }
        }

        header_buffer.write_fmt(format_args!("extern TileSetResource RES_{};\n", item.id))?;
        source_buffer.write_fmt(format_args!(
            "TileSetResource RES_{} = {{ .address = {}, .size = {} }};\n",
            item.id,
            data_address,
            tiles.len() * 64
        ))?;
    }

    Ok(())
}
