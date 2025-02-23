use std::io::Write;

use serde::{Deserialize, Serialize};

use super::ResCompilerArgs;

#[derive(Serialize, Deserialize)]
pub struct TileMapDef {
    id: String,
    path: String,
}

pub fn compile(
    ResCompilerArgs {
        ref mut header_buffer,
        ref mut data_buffer,
        ref mut source_buffer,
        res_config,
        res_path,
        ..
    }: &mut ResCompilerArgs,
) -> anyhow::Result<()> {
    header_buffer.write_all(b"\n// ---- tilemaps ----\n")?;
    source_buffer.write_all(b"\n// ---- tilemaps ----\n")?;

    for item in res_config.tilemaps.iter() {
        /*
        header_buffer.write_fmt(format_args!("extern TileSetResource RES_{};\n", item.id))?;
        source_buffer.write_fmt(format_args!(
            "TileSetResource RES_{} = {{ .address = {}, .size = {} }};\n",
            item.id,
            data_address,
            tiles.len() * 64
        ))?;*/
    }

    Ok(())
}
